//! Embedded Car project

#![no_std]
#![no_main]
// The executor is single threaded
#![allow(clippy::future_not_send)]

use {defmt_rtt as _, panic_probe as _};

use core::sync::atomic::{AtomicBool, Ordering};
use defmt::unwrap;
use embassy_executor::Spawner;
use embassy_stm32::{
	bind_interrupts,
	gpio::{Level, Output, Speed},
	peripherals, usart, Config,
};
use embassy_time::{Duration, Timer};

mod components;

use components::{Hc06, HcSr04, Sg90, L298N};

/// Indicate if the program is connected to a computer.
pub static IS_CONNECTED_TO_CONTROLLER: AtomicBool = AtomicBool::new(false);

#[embassy_executor::task]
/// Tells if the program is running on the microcontroller.
async fn alive_blinker(mut led: Output<'static>) {
	loop {
		led.toggle();
		Timer::after(if IS_CONNECTED_TO_CONTROLLER.load(Ordering::Relaxed) {
			Duration::from_millis(1000)
		} else {
			Duration::from_millis(125)
		})
		.await;
	}
}

bind_interrupts!(struct Interrupts {
	USART1 => usart::InterruptHandler<peripherals::USART1>;
});

#[embassy_executor::main]
async fn main(spawner: Spawner) {
	let p = embassy_stm32::init(Config::default());

	let board_led = Output::new(p.PC13, Level::Low, Speed::Low);
	unwrap!(spawner.spawn(alive_blinker(board_led)));

	let mut bluetooth = Hc06::from_pins(p.USART1, p.PB6, p.PB7, Interrupts, p.DMA1_CH4, p.DMA1_CH5);

	let _ultrasonic = HcSr04::from_pins(p.PB4, p.PB5, p.EXTI5);
	let _servo = Sg90::from_pin(p.PA15, p.TIM2);

	let _motor_driver = L298N::from_pins(p.PA7, p.PA6, p.PA8, p.PA5, p.PA4, p.PA9, p.TIM1);

	unwrap!(bluetooth.ping().await);
}
