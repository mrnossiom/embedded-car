//! Embedded Car project

#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]
#![warn(
	clippy::unwrap_used,
	clippy::todo,
	clippy::too_many_lines,
	clippy::unicode_not_nfc,
	clippy::unused_async,
	clippy::use_self,
	clippy::dbg_macro,
	clippy::doc_markdown,
	clippy::else_if_without_else,
	clippy::implicit_clone,
	clippy::match_bool,
	clippy::missing_panics_doc,
	clippy::redundant_closure_for_method_calls,
	clippy::redundant_else,
	clippy::must_use_candidate,
	clippy::return_self_not_must_use,
	clippy::missing_docs_in_private_items,
	rustdoc::broken_intra_doc_links
)]

use {defmt_rtt as _, panic_probe as _};

use defmt::{debug, info, unwrap};
use embassy_executor::Spawner;
use embassy_stm32::{
	dma::NoDma,
	gpio::{Level, Output, Speed},
	interrupt,
	peripherals::{PA4, PA5, PA6, PA7, PB4, PB5, PC13, TIM1},
};
use embassy_time::{Duration, Timer};

mod hc06;
mod hcsr04;
mod l298n;
mod sg90;

use hc06::Hc06;
use hcsr04::HcSr04;
use l298n::L298N;
use sg90::Sg90;

#[embassy_executor::task]
/// Tells if the program is running on the microcontroller.
async fn alive_blinker(mut led: Output<'static, PC13>, interval: Duration) {
	loop {
		led.toggle();
		Timer::after(interval).await;
	}
}

#[embassy_executor::task]
/// Tests the car.
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

#[embassy_executor::task]
/// Play with the `HC-SR04` ultrasonic sensor.
async fn yield_distance(mut sensor: HcSr04<'static, PB4, PB5>) {
	loop {
		let distance = sensor.ping_distance().await;
		info!("Distance: {:?} mm", distance);

		Timer::after(Duration::from_millis(1000)).await;
	}
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
	let p = embassy_stm32::init(Default::default());

	// The fourth timer is used by `embassy-time` for timers.
	// It is enabled by the `time-driver-tim4` feature on the `embassy-stm32` crate.
	// I think, it is possible to use this timer with precaution.
	//
	// I chose to drop it to avoid any conflict.
	//
	// Link to relevant part of the build script from `embassy-stm32` crate:
	// https://github.com/embassy-rs/embassy/blob/2528f451387e6c7b27c3140cd87d47521d1971a2/embassy-stm32/build.rs#L716-L765
	#[allow(clippy::drop_non_drop)]
	drop(p.TIM4);

	info!("Hello World!");

	let board_led = Output::new(p.PC13, Level::Low, Speed::Low);
	unwrap!(spawner.spawn(alive_blinker(board_led, Duration::from_millis(500))));

	let _servo = Sg90::from_pin(p.PA15, p.TIM2);

	let ultrasonic = HcSr04::from_pins(p.PB4, p.PB5, p.EXTI5);
	unwrap!(spawner.spawn(yield_distance(ultrasonic)));

	let bluetooth_irq = interrupt::take!(USART1);
	let _bluetooth = Hc06::from_pins(p.USART1, p.PB6, p.PB7, bluetooth_irq, NoDma, NoDma);

	let motor_driver = L298N::from_pins(p.PA7, p.PA6, p.PA8, p.PA5, p.PA4, p.PA9, p.TIM1);
	unwrap!(spawner.spawn(run_forest_run(motor_driver)));
}
