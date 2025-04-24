//! Embedded Car project

#![no_std]
#![no_main]
// The executor is single threaded
#![allow(clippy::future_not_send)]

use core::sync::atomic::{AtomicBool, Ordering};

use defmt::unwrap;
use embassy_executor::Spawner;
use embassy_stm32::{
	Config, bind_interrupts,
	gpio::{Level, Output, Speed},
	peripherals, usart,
};
use embassy_time::{Duration, Timer};
use {defmt_rtt as _, panic_probe as _};

mod components;

use components::{Hc06, HcSr04, L298N, Sg90};

/// Indicate if the program is connected to a computer.
pub static IS_CONNECTED_TO_CONTROLLER: AtomicBool = AtomicBool::new(false);

#[embassy_executor::task]
/// Tells if the program is running on the microcontroller.
async fn alive_blinker(mut led: Output<'static>) {
	loop {
		let nb_of_blinks = if IS_CONNECTED_TO_CONTROLLER.load(Ordering::Relaxed) {
			5
		} else {
			3
		};

		for _ in 0..nb_of_blinks {
			led.set_low();
			Timer::after(Duration::from_millis(150)).await;
			led.set_high();
			Timer::after(Duration::from_millis(100)).await;
		}

		Timer::after(Duration::from_millis(1000)).await;
	}
}

bind_interrupts!(struct Interrupts {
	USART2 => usart::InterruptHandler<peripherals::USART2>;
});

#[embassy_executor::main]
async fn main(spawner: Spawner) {
	let p = embassy_stm32::init(Config::default());

	let board_led = Output::new(p.PC13, Level::Low, Speed::Low);
	unwrap!(spawner.spawn(alive_blinker(board_led)));

	// TODO: check connections for PA3 et PA2
	let mut bluetooth = Hc06::from_pins(p.USART2, p.PA3, p.PA2, Interrupts, p.DMA1_CH7, p.DMA1_CH6);

	loop {
		unwrap!(bluetooth.ping().await);
	}

	// let _ultrasonic = HcSr04::from_pins(p.PB4, p.PB5, p.EXTI5);
	// TODO: pins already in use
	// let mut servo = Sg90::from_pin(p.PB3, p.TIM2);

	// let _motor_driver = L298N::from_pins(p.PA7, p.PA6, p.PA8, p.PA5, p.PA4, p.PA9, p.TIM1);

	// unwrap!(bluetooth.ping_text().await);

	// loop {
	// 	servo.set_angle(0);
	// 	Timer::after(Duration::from_secs(1)).await;
	// 	servo.set_angle(90);
	// 	Timer::after(Duration::from_secs(1)).await;
	// 	servo.set_angle(180);
	// 	Timer::after(Duration::from_secs(1)).await;
	// }
}
