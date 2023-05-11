//! Embedded Car project

#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]
#![warn(
	clippy::missing_docs_in_private_items,
	clippy::unwrap_used,
	clippy::nursery,
	clippy::pedantic,
	clippy::cargo
)]

use {defmt_rtt as _, panic_probe as _};

use core::sync::atomic::{AtomicBool, Ordering};
use defmt::unwrap;
use embassy_executor::Spawner;
use embassy_stm32::{
	gpio::{Level, Output, Speed},
	interrupt,
	peripherals::PC13,
	Config,
};
use embassy_time::{Duration, Timer};

mod components;

use components::{Hc06, HcSr04, Sg90, L298N};

/// Indicate if the program is connected to a computer.
pub static IS_CONNECTED_TO_CONTROLLER: AtomicBool = AtomicBool::new(false);

#[embassy_executor::task]
/// Tells if the program is running on the microcontroller.
async fn alive_blinker(mut led: Output<'static, PC13>) {
	loop {
		led.toggle();
		Timer::after(if IS_CONNECTED_TO_CONTROLLER.load(Ordering::Relaxed) {
			Duration::from_millis(500)
		} else {
			Duration::from_millis(125)
		})
		.await;
	}
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
	let p = embassy_stm32::init(Config::default());

	// The fourth timer is used by `embassy-time` for timers.
	// It is enabled by the `time-driver-tim4` feature on the `embassy-stm32` crate.
	// I think, it is possible to use this timer with precaution.
	//
	// I chose to drop ownership to avoid any conflict.
	//
	// Link to relevant part of the build script from `embassy-stm32` crate:
	// https://github.com/embassy-rs/embassy/blob/2528f451387e6c7b27c3140cd87d47521d1971a2/embassy-stm32/build.rs#L716-L765
	let _ = p.TIM4;

	let board_led = Output::new(p.PC13, Level::Low, Speed::Low);
	unwrap!(spawner.spawn(alive_blinker(board_led)));

	let bluetooth_irq = interrupt::take!(USART1);
	let mut bluetooth = Hc06::from_pins(
		p.USART1,
		p.PB6,
		p.PB7,
		bluetooth_irq,
		p.DMA1_CH4,
		p.DMA1_CH5,
	);

	let _ultrasonic = HcSr04::from_pins(p.PB4, p.PB5, p.EXTI5);
	let _servo = Sg90::from_pin(p.PA15, p.TIM2);

	let _motor_driver = L298N::from_pins(p.PA7, p.PA6, p.PA8, p.PA5, p.PA4, p.PA9, p.TIM1);

	unwrap!(bluetooth.ping().await);
}
