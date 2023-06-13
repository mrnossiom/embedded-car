use car_controller::{Bluetooth, Controller};

const BLUETOOTH_MODULE_HC_05: &str = "BT05";

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
	color_eyre::install()?;

	let mut _controller = Controller::new()?;
	let mut bluetooth = Bluetooth::connect_by_name(BLUETOOTH_MODULE_HC_05, None).await?;

	loop {
		bluetooth.write(&[20, 40, 0, 10]).await?;
		let res = bluetooth.read().await?;

		if res != [0, 0] {
			dbg!(res);
			break;
		}
	}

	Ok(())
}
