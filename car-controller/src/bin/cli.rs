use car_controller::CarBluetooth;

const BLUETOOTH_MODULE_HC_05: &str = "BT05";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	let mut bluetooth = CarBluetooth::connect_by_name(BLUETOOTH_MODULE_HC_05, None).await?;

	bluetooth.write(b"Hello, World!").await?;

	let mut bytes = [0_u8; 13];
	bluetooth.receive(&mut bytes).await?;

	println!("{}", bytes.iter().map(|b| *b as char).collect::<String>());

	Ok(())
}
