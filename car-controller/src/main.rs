use btleplug::{
	api::{bleuuid::uuid_from_u16, Central, Manager as _, Peripheral as _, ScanFilter},
	platform::{Adapter, Manager, Peripheral},
};
use std::{error::Error, time::Duration};
use tokio::time;
use uuid::Uuid;

const LIGHT_CHARACTERISTIC_UUID: Uuid = uuid_from_u16(0xFFE9);

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	let manager = Manager::new().await.unwrap();

	// Get the first bluetooth adapter
	let adapters = manager.adapters().await?;
	let central = adapters.into_iter().next().unwrap();

	// Start scanning for devices
	central.start_scan(ScanFilter::default()).await?;
	// Instead of waiting, you can use central.events() to get a stream which will
	// notify you of new devices, for an example of that see examples/event_driven_discovery.rs
	time::sleep(Duration::from_secs(2)).await;

	// Find the device we're interested in
	let light = find_light(&central).await.unwrap();

	// Connect to the device
	light.connect().await?;

	// Discover services and characteristics
	light.discover_services().await?;

	// Find the characteristic we want
	let chars = light.characteristics();
	let _cmd_char = chars
		.iter()
		.find(|c| c.uuid == LIGHT_CHARACTERISTIC_UUID)
		.unwrap();

	// Play with the light

	Ok(())
}

async fn find_light(central: &Adapter) -> Option<Peripheral> {
	for p in central.peripherals().await.unwrap() {
		if p.properties()
			.await
			.unwrap()
			.unwrap()
			.local_name
			.iter()
			.any(|name| name.contains("LEDBlue"))
		{
			return Some(p);
		}
	}
	None
}
