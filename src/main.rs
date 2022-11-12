#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use {defmt_rtt as _, panic_probe as _};

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::{
	gpio::{Level, Output, Speed},
	peripherals::{PC13, TIM1},
	pwm::{
		simple_pwm::{PwmPin, SimplePwm},
		Channel,
	},
	time::hz,
};
use embassy_time::{Duration, Timer};

#[embassy_executor::task]
async fn blinker(mut led: Output<'static, PC13>, interval: Duration) {
	loop {
		led.set_high();
		Timer::after(interval).await;

		led.set_low();
		Timer::after(interval).await;
	}
}

#[embassy_executor::task]
async fn motor(mut pwm: SimplePwm<'static, TIM1>, interval: Duration) {
	let max = pwm.get_max_duty();
	pwm.enable(Channel::Ch1);

	info!("PWM initialized");
	info!("PWM max duty {}", max);

	// 50Hz => 20ms => 20_000μs
	// Servo motor Pulse Width is from 500 to 2400 μs
	loop {
		// 20_000μs / 500 = 40
		pwm.set_duty(Channel::Ch1, max / 40);
		Timer::after(interval).await;

		// (2400+500) / 2 = 1450
		// 20_000μs / 1450 = 13.7 (13 works better tough)
		pwm.set_duty(Channel::Ch1, max / 13);
		Timer::after(interval).await;

		// 20_000μs / 2400 = 8.3 (Rounded)
		pwm.set_duty(Channel::Ch1, max / 8);
		Timer::after(interval).await;

		info!("finished cycle")
	}
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
	let p = embassy_stm32::init(Default::default());
	info!("Hello World!");

	let board_led = Output::new(p.PC13, Level::Low, Speed::Medium);
	unwrap!(spawner.spawn(blinker(board_led, Duration::from_millis(500))));

	let ch1 = PwmPin::new_ch1(p.PA8);
	let pwm = SimplePwm::new(p.TIM1, Some(ch1), None, None, None, hz(50));
	unwrap!(spawner.spawn(motor(pwm, Duration::from_millis(2000))));
}
