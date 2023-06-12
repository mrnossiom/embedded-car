//! Contains `Bluetooth` communication logic with the `HC-06` module

use futures::StreamExt;
use std::io;
use tokio::time::{sleep, Duration};

#[cfg(not(any(feature = "ble", feature = "classic-bt")))]
compile_error!("You need one of `ble` and `classic-bt` features enabled.");

#[cfg(all(feature = "ble", feature = "classic-bt"))]
compile_error!("You cannot have both `ble` and `classic-bt` features enabled at the same time. Please choose one of them.");

#[cfg(feature = "classic-bt")]
use bluer::{
	rfcomm::{
		stream::{OwnedReadHalf, OwnedWriteHalf},
		SocketAddr, Stream,
	},
	AdapterEvent, Device,
};
#[cfg(feature = "classic-bt")]
use futures::pin_mut;
#[cfg(feature = "classic-bt")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[cfg(feature = "ble")]
use btleplug::{
	api::{
		Central, CentralEvent, CharPropFlags, Characteristic, Manager as _, Peripheral as _,
		ScanFilter, ValueNotification, WriteType,
	},
	platform::{Manager, Peripheral},
};
#[cfg(feature = "ble")]
use std::{fmt, pin::Pin};

#[cfg(feature = "ble")]
/// Implements the `Bluetooth` communication logic
pub struct Bluetooth {
	/// The distant `Bluetooth` device
	pub peripheral: Peripheral,
	/// The `Bluetooth` characteristic to send and receive data
	pub characteristic: Characteristic,
	/// Events received through the `Bluetooth` characteristic
	events: Pin<Box<dyn futures::Stream<Item = ValueNotification> + Send>>,
}

#[cfg(feature = "ble")]
impl fmt::Debug for Bluetooth {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("CarBluetooth")
			.field("peripheral", &self.peripheral)
			.field("characteristic", &self.characteristic)
			.finish()
	}
}

#[cfg(feature = "ble")]
impl Bluetooth {
	/// Creates a new [`CarBluetooth`] from the given [`Peripheral`]
	///
	/// # Errors
	/// In case we don't find a device or if the peripheral does not have a characteristic to write and notify
	pub async fn new(peripheral: Peripheral) -> Result<Self, BluetoothError> {
		peripheral.discover_services().await?;
		let mut characteristics = peripheral.characteristics().into_iter();

		let characteristic = characteristics
			.find(|c| {
				c.properties.contains(
					CharPropFlags::READ
						| CharPropFlags::WRITE_WITHOUT_RESPONSE
						| CharPropFlags::NOTIFY,
				)
			})
			.ok_or(BluetoothError::Io(io::Error::new(
				io::ErrorKind::Unsupported,
				"Bluetooth device does not have a characteristic to write and notify",
			)))?;

		peripheral.subscribe(&characteristic).await?;
		let events = peripheral.notifications().await?;

		Ok(Self {
			peripheral,
			characteristic,
			events,
		})
	}

	/// Find a bluetooth peripheral by it's name and connect to it. Default timeout is 2s.
	///
	/// # Errors
	/// In case we cannot build a bluetooth manager
	/// In case the peripheral is not found or the connection fails
	pub async fn connect_by_name(
		name: &str,
		timeout: Option<Duration>,
	) -> Result<Self, BluetoothError> {
		let manager = Manager::new().await?;

		// We use the first Bluetooth adapter
		let adapters = manager.adapters().await?;
		let central = adapters
			.into_iter()
			.next()
			.ok_or(BluetoothError::BluetoothNotSupported)?;

		central.start_scan(ScanFilter::default()).await?;

		let mut events = central.events().await?;
		let timeout = sleep(timeout.unwrap_or_else(|| Duration::from_secs(2)));
		tokio::pin!(timeout);

		let peripheral = loop {
			tokio::select! { biased;
				event = events.next() => {
					if let Some(CentralEvent::DeviceDiscovered(id)) = event {
						let peripheral = central.peripheral(&id).await?;
						let Some(Some(peripheral_name)) = peripheral.properties().await?.map(|p| p.local_name) else { continue; };

						if peripheral_name == name {
							central.stop_scan().await?;

							break peripheral;
						}
					}
				}
				_ = &mut timeout => {
					return Err(BluetoothError::Io(io::Error::new(io::ErrorKind::TimedOut, "Bluetooth device not found")));
				}
			}
		};

		peripheral.connect().await?;

		Self::new(peripheral).await
	}

	/// Write bytes to the `Bluetooth` device
	///
	/// # Errors
	/// In case the write operation fails
	pub async fn write(&mut self, bytes: &[u8]) -> Result<(), BluetoothError> {
		self.peripheral
			.write(&self.characteristic, bytes, WriteType::WithoutResponse)
			.await?;

		Ok(())
	}

	/// Read bytes from the `Bluetooth` device
	///
	/// # Errors
	/// In case the read operation fails
	pub async fn read(&mut self) -> Result<Vec<u8>, BluetoothError> {
		let bytes = self.peripheral.read(&self.characteristic).await?;

		Ok(bytes)
	}

	/// Receive a message from the bluetooth device
	///
	/// # Errors
	/// In case the read operation fails
	pub async fn receive(&mut self, bytes: &mut [u8]) -> Result<usize, BluetoothError> {
		let stream = self.events.next().await.ok_or_else(|| {
			std::io::Error::new(
				std::io::ErrorKind::TimedOut,
				"Bluetooth notification timeout occurred",
			)
		})?;
		let data = &stream.value;

		// Only write up until the length of the buffer
		let length = bytes.len().min(data.len());
		bytes[..length].copy_from_slice(&data[..length]);

		// Return the number of bytes received from the stream
		Ok(data.len())
	}
}

#[cfg(feature = "classic-bt")]
/// Implements the `Bluetooth` communication logic
#[derive(Debug)]
pub struct Bluetooth {
	// The distant `Bluetooth` device
	pub device: Device,
	// The `Bluetooth` characteristic to send and receive data
	pub read: OwnedReadHalf,
	pub write: OwnedWriteHalf,
}

#[cfg(feature = "classic-bt")]
impl Bluetooth {
	/// Creates a new [`Bluetooth`] from the given [`Device`]
	///
	/// # Errors
	/// In case we don't find a device or if the peripheral does not have a characteristic to write and notify
	pub async fn new(device: Device) -> Result<Self, BluetoothError> {
		let target = SocketAddr::new(device.address(), 0);
		log::trace!("Creating L2CAP stream to {:?}", target);
		let stream = Stream::connect(target).await?;

		let (read, write) = stream.into_split();

		Ok(Self {
			device,
			read,
			write,
		})
	}

	/// Find a bluetooth peripheral by it's name and connect to it. Default timeout is 2s.
	///
	/// # Errors
	/// In case we cannot build a bluetooth manager
	/// In case the peripheral is not found or the connection fails
	pub async fn connect_by_name(
		name: &str,
		timeout: Option<Duration>,
	) -> Result<Self, BluetoothError> {
		let session = bluer::Session::new().await?;

		// We use the first Bluetooth adapter
		let adapter = session.default_adapter().await?;
		adapter.set_powered(true).await?;

		let device_events = adapter.discover_devices().await?;
		pin_mut!(device_events);

		let timeout = sleep(timeout.unwrap_or_else(|| Duration::from_secs(2)));
		tokio::pin!(timeout);

		let device = loop {
			tokio::select! {
				Some(device_event) = device_events.next() => {
					match device_event {
						AdapterEvent::DeviceAdded(addr) => {
							log::debug!("Device added: {addr}");

							let device = adapter.device(addr)?;
							let device_name = device.name().await?;

							if let Some(device_name)  = device_name {
								log::info!("Found device with name: {device_name}");

								if name == device_name {
									break device;
								}
							}
						}
						ev => {
							log::debug!("Bluetooth discovery event: {ev:?}");
						},
					}
				}
				_ = &mut timeout => {
					return Err(BluetoothError::Io(io::Error::new(io::ErrorKind::TimedOut, "Bluetooth device not found")));
				}
				else => return Err(BluetoothError::Io(io::Error::new(io::ErrorKind::TimedOut, "No more events to process"))),
			}
		};

		log::trace!("Connecting to found device");
		device.connect().await?;

		Self::new(device).await
	}

	/// Write bytes to the `Bluetooth` device
	///
	/// # Errors
	/// In case the write operation fails
	pub async fn write(&mut self, bytes: &[u8]) -> Result<(), BluetoothError> {
		self.write.write_all(bytes).await?;

		Ok(())
	}

	/// Read bytes from the `Bluetooth` device
	///
	/// # Errors
	/// In case the read operation fails
	pub async fn read(&mut self) -> Result<Vec<u8>, BluetoothError> {
		let mut bytes = Vec::new();

		self.read.read_to_end(&mut bytes).await?;

		Ok(bytes)
	}
}

/// Errors that can occur when using [`CarBluetooth`]
#[derive(Debug, thiserror::Error)]
pub enum BluetoothError {
	/// Bluetooth is not supported on this device
	#[error("Bluetooth is not supported on this device")]
	BluetoothNotSupported,

	/// An error occurred while communicating with the `bluetooth` device
	#[error(transparent)]
	Io(#[from] std::io::Error),

	/// An error occurred while using the underlying library or communicating with the bluetooth device
	#[cfg(feature = "ble")]
	#[error(transparent)]
	Lib(#[from] btleplug::Error),

	/// An error occurred while using the underlying library or communicating with the bluetooth device
	#[cfg(feature = "classic-bt")]
	#[error(transparent)]
	Lib(#[from] bluer::Error),
}
