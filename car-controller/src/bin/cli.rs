use car_controller::{Bluetooth, Controller};

#[cfg(feature = "classic-bt")]
const BLUETOOTH_MODULE_HC_06: &str = "HC-06";
#[cfg(feature = "ble")]
const BLUETOOTH_MODULE_HM_10: &str = "BT05";

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
