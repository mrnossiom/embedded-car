//! Contains `Bluetooth` communication logic with the `HC-06` module

use btleplug::{
	api::{
		Central, CentralEvent, CharPropFlags, Characteristic, Manager as _, Peripheral as _,
		ScanFilter, ValueNotification, WriteType,
	},
	platform::{Manager, Peripheral},
};
use futures::{Stream, StreamExt};
use std::{fmt, io, pin::Pin};
use tokio::time::{sleep, Duration};

/// Implements the `Bluetooth` communication logic
pub struct Bluetooth {
	/// The distant `Bluetooth` device
	pub peripheral: Peripheral,
	/// The `Bluetooth` characteristic to send and receive data
	pub characteristic: Characteristic,
	/// Events received through the `Bluetooth` characteristic
	events: Pin<Box<dyn Stream<Item = ValueNotification> + Send>>,
}

impl fmt::Debug for Bluetooth {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("CarBluetooth")
			.field("peripheral", &self.peripheral)
			.field("characteristic", &self.characteristic)
			.finish()
	}
}

impl Bluetooth {
	/// Creates a new [`CarBluetooth`] from the given [`Peripheral`]
	///
	/// # Errors
	/// In case we don't find a device or if the peripheral does not have a characteristic to write and notify
	pub async fn new(peripheral: Peripheral) -> Result<Self, CarBluetoothError> {
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
			.ok_or(CarBluetoothError::Io(io::Error::new(
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
	) -> Result<Self, CarBluetoothError> {
		let manager = Manager::new().await?;

		// We use the first Bluetooth adapter
		let adapters = manager.adapters().await?;
		let central = adapters
			.into_iter()
			.next()
			.ok_or(CarBluetoothError::BluetoothNotSupported)?;

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
					return Err(CarBluetoothError::Io(io::Error::new(io::ErrorKind::TimedOut, "Bluetooth device not found")));
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
	pub async fn write(&mut self, bytes: &[u8]) -> Result<(), CarBluetoothError> {
		self.peripheral
			.write(&self.characteristic, bytes, WriteType::WithoutResponse)
			.await?;

		Ok(())
	}

	/// Read bytes from the `Bluetooth` device
	///
	/// # Errors
	/// In case the read operation fails
	pub async fn read(&mut self) -> Result<Vec<u8>, CarBluetoothError> {
		let bytes = self.peripheral.read(&self.characteristic).await?;

		Ok(bytes)
	}

	/// Receive a message from the bluetooth device
	///
	/// # Errors
	/// In case the read operation fails
	pub async fn receive(&mut self, bytes: &mut [u8]) -> Result<usize, CarBluetoothError> {
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

/// Errors that can occur when using [`CarBluetooth`]
#[derive(Debug, thiserror::Error)]
pub enum CarBluetoothError {
	/// Bluetooth is not supported on this device
	#[error("Bluetooth is not supported on this device")]
	BluetoothNotSupported,

	/// An error occurred while communicating with the `bluetooth` device
	#[error(transparent)]
	Io(#[from] std::io::Error),

	/// An error occurred while using the underlying library or communicating with the bluetooth device
	#[error(transparent)]
	BtlePlug(#[from] btleplug::Error),
}
