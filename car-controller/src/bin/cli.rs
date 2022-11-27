use car_controller::CarBluetooth;

const BLUETOOTH_MODULE_HC_05: &str = "BT05";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	let _bluetooth = CarBluetooth::connect_by_name(BLUETOOTH_MODULE_HC_05, None).await?;

	Ok(())
}
