#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use {defmt_rtt as _, panic_probe as _};

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::{
	gpio::{Level, Output, Speed},
	peripherals::PC13,
};
use embassy_time::{Duration, Timer};

#[embassy_executor::task]
async fn blinker(mut led: Output<'static, PC13>, interval: Duration) {
	loop {
		info!("high");
		led.set_high();
		Timer::after(interval).await;

		info!("low");
		led.set_low();
		Timer::after(interval).await;
	}
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
	let p = embassy_stm32::init(Default::default());
	let led = Output::new(p.PC13, Level::Low, Speed::Medium);

	info!("Hello World!");

	unwrap!(spawner.spawn(blinker(led, Duration::from_millis(300))));
}
