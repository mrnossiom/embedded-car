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

pub struct CarBluetooth {
	peripheral: Peripheral,

	rx_characteristic: Characteristic,
	tx_characteristic: Characteristic,

	events: Pin<Box<dyn Stream<Item = ValueNotification> + Send>>,
}

impl fmt::Debug for CarBluetooth {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("CarBluetooth")
			.field("peripheral", &self.peripheral)
			.field("rx_characteristic", &self.rx_characteristic)
			.field("tx_characteristic", &self.tx_characteristic)
			.finish()
	}
}

impl CarBluetooth {
	pub async fn new(peripheral: Peripheral) -> Result<Self, CarBluetoothError> {
		peripheral.discover_services().await?;
		let mut characteristics = peripheral.characteristics().into_iter();

		let rx_characteristic = characteristics
			.find(|c| c.properties.contains(CharPropFlags::READ))
			.ok_or(CarBluetoothError::Io(io::Error::new(
				io::ErrorKind::Unsupported,
				"Bluetooth device does not have a characteristic to read from",
			)))?;

		let tx_characteristic = characteristics
			.find(|c| {
				c.properties
					.contains(CharPropFlags::WRITE_WITHOUT_RESPONSE & CharPropFlags::NOTIFY)
			})
			.ok_or(CarBluetoothError::Io(io::Error::new(
				io::ErrorKind::Unsupported,
				"Bluetooth device does not have a characteristic to write and notify",
			)))?;

		peripheral.subscribe(&tx_characteristic).await?;
		let events = peripheral.notifications().await?;

		Ok(Self {
			peripheral,
			rx_characteristic,
			tx_characteristic,
			events,
		})
	}

	/// Find a bluetooth peripheral by it's name and connect to it. Default timeout is 2s.
	pub async fn connect_by_name(
		name: &str,
		timeout: Option<Duration>,
	) -> Result<Self, CarBluetoothError> {
		let manager = Manager::new().await?;

		// We use the first Bluetooth adapter
		let adapters = manager.adapters().await?;
		let central = adapters.into_iter().next().unwrap();

		central.start_scan(ScanFilter::default()).await?;

		let mut events = central.events().await?;
		let timeout = sleep(timeout.unwrap_or_else(|| Duration::from_secs(2)));
		tokio::pin!(timeout);

		let peripheral = loop {
			tokio::select! { biased;
				event = events.next() => {
					if let Some(CentralEvent::DeviceDiscovered(id)) = event {
						let peripheral = central.peripheral(&id).await?;

						if peripheral.properties().await?.unwrap().local_name == Some(name.to_string()) {
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

	pub async fn send(&mut self, bytes: &[u8]) -> Result<(), CarBluetoothError> {
		self.peripheral
			.write(&self.tx_characteristic, bytes, WriteType::WithoutResponse)
			.await?;

		Ok(())
	}

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

#[derive(Debug, thiserror::Error)]
pub enum CarBluetoothError {
	#[error(transparent)]
	Io(#[from] std::io::Error),

	#[error(transparent)]
	BtlePlug(#[from] btleplug::Error),
}
