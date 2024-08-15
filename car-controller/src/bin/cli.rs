//! Sends car control commands to the car's bt module

use car_controller::{Bluetooth, Controller};

#[cfg(feature = "classic-bt")]
/// Bluetooth name of the HC-06 Classic BT module
const BLUETOOTH_MODULE_HC_06: &str = "RenaultClio";
#[cfg(feature = "ble")]
/// Bluetooth name of the HM-10 BLE module
const BLUETOOTH_MODULE_HM_10: &str = "RenaultClioBLE";

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
	color_eyre::install()?;
	pretty_env_logger::init();

	let _controller = Controller::new()?;
	let mut bluetooth = Bluetooth::connect_by_name(BLUETOOTH_MODULE_HC_06, None).await?;

	loop {
		bluetooth.write(&[20, 40, 0, 10]).await?;
		let res = bluetooth.read().await?;

		if res != [0, 0] {
			log::debug!("{res:?}");
			break;
		}
	}

	Ok(())
}
