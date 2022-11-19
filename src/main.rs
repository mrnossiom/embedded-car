#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use {defmt_rtt as _, panic_probe as _};

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::{
	gpio::{Level, Output, Speed},
	peripherals::{PA4, PA5, PA6, PA7, PC13, TIM1},
};
use embassy_time::{Duration, Timer};

mod l298n;

use l298n::L298N;

#[embassy_executor::task]
async fn alive_blinker(mut led: Output<'static, PC13>, interval: Duration) {
	loop {
		led.toggle();
		Timer::after(interval).await;
	}
}

#[embassy_executor::task]
async fn run_forest_run(mut motor_driver: L298N<'static, PA7, PA6, PA5, PA4, TIM1>) {
	let interval = Duration::from_millis(1000);

	info!("Forest is running!");

	for index in 1..=10 {
		motor_driver.set_duty_percentage(Some(index * 10), Some(index * 10));

		motor_driver.forward();
		Timer::after(interval).await;

		motor_driver.brake();
		Timer::after(interval).await;

		motor_driver.reverse();
		Timer::after(interval).await;

		motor_driver.brake();
		Timer::after(interval).await;

		debug!("finished cycle")
	}

	info!("Forest no longer wants to run!");
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
	let p = embassy_stm32::init(Default::default());
	info!("Hello World!");

	let board_led = Output::new(p.PC13, Level::Low, Speed::Low);
	unwrap!(spawner.spawn(alive_blinker(board_led, Duration::from_millis(500))));

	let motor_driver = L298N::from_pins(p.PA7, p.PA6, p.PA8, p.PA5, p.PA4, p.PA9, p.TIM1);
	unwrap!(spawner.spawn(run_forest_run(motor_driver)));
}
