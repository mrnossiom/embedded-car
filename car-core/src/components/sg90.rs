//! `SG-90` servo motor driver

use core::ops::{Add, Div, Mul, Sub};

use embassy_stm32::{
	Peri,
	gpio::OutputType,
	time::hz,
	timer::{
		Channel1Pin, Channel2Pin, GeneralInstance4Channel,
		low_level::CountingMode,
		simple_pwm::{PwmPin, SimplePwm},
	},
};

/// Represents a small `SG-90` servo motor.
pub struct Sg90<'a, TimerPeripheral: GeneralInstance4Channel> {
	/// The underlying `PWM` to control the servo motor
	pwm: SimplePwm<'a, TimerPeripheral>,
}

impl<'a, TimerPeripheral: GeneralInstance4Channel> Sg90<'a, TimerPeripheral> {
	/// Creates a `SG90` servo handle from the pwn pin.
	pub fn from_pin(
		pwm_pin: Peri<'a, impl Channel2Pin<TimerPeripheral>>,
		timer: Peri<'a, TimerPeripheral>,
	) -> Sg90<'a, TimerPeripheral> {
		let pwm_pin = PwmPin::new_ch2(pwm_pin, OutputType::PushPull);

		let mut pwm = SimplePwm::new(
			timer,
			None,
			Some(pwm_pin),
			None,
			None,
			hz(50),
			CountingMode::default(),
		);

		pwm.ch2().enable();

		Self { pwm }
	}

	/// Changes the motor speed by a percentage
	fn set_duty(&mut self, duty: u16) -> &mut Self {
		self.pwm.ch2().set_duty_cycle(duty);
		self
	}

	/// Changes the motor speed by a percentage
	pub fn set_angle(&mut self, angle: u8) -> &mut Self {
		// Asserts the number is between 1 and 180
		assert!(angle <= 180);

		let max_duty = self.pwm.max_duty_cycle();
		// for 0.5-2.5ms duty range
		// let duty_range = (max_duty * 25 / 1000, max_duty * 125 / 1000);

		// for 1-2ms duty range
		let duty_range = (max_duty * 25 / 1000, max_duty * 125 / 1000);

		let duty = map_range((0, 180), duty_range, angle.into());

		self.set_duty(duty);
		self
	}

	// // 50Hz => 20ms => 20_000μs
	// // Servo motor Pulse Width is from 500 to 2400 μs

	// // 20_000μs / 500 = 40
	// 	pwm.set_duty(Channel::Ch1, max / 40);

	// 	// (2400+500) / 2 = 1450
	// 	// 20_000μs / 1450 = 13.7 (13 works better tough)
	// 	pwm.set_duty(Channel::Ch1, max / 13);

	// 	// 20_000μs / 2400 = 8.3 (Rounded)
	// 	pwm.set_duty(Channel::Ch1, max / 8);
}

fn map_range<T: Copy>(from_range: (T, T), to_range: (T, T), s: T) -> T
where
	T: Add<T, Output = T> + Sub<T, Output = T> + Mul<T, Output = T> + Div<T, Output = T>,
{
	to_range.0 + (s - from_range.0) * (to_range.1 - to_range.0) / (from_range.1 - from_range.0)
}
