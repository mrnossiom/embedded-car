//! `SG-90` servo motor driver

use core::ops::{Add, Div, Mul, Sub};
use embassy_stm32::{
	gpio::OutputType,
	time::hz,
	timer::{
		low_level::CountingMode,
		simple_pwm::{PwmPin, SimplePwm},
		Channel, Channel1Pin, Channel2Pin, GeneralInstance4Channel,
	},
	Peripheral,
};

/// Represents a small `SG-90` servo motor.
pub struct Sg90<'a, TimerPeripheral: GeneralInstance4Channel> {
	/// The underlying `PWM` to control the servo motor
	pwm: SimplePwm<'a, TimerPeripheral>,
}

impl<'a, TimerPeripheral: GeneralInstance4Channel> Sg90<'a, TimerPeripheral> {
	/// Creates a `SG90` servo handle from the pwn pin.
	pub fn from_pin<
		Timer: Peripheral<P = TimerPeripheral> + 'a,
		PwmA: Channel1Pin<TimerPeripheral>,
	>(
		pwm_pin: PwmA,
		timer: Timer,
	) -> Sg90<'a, TimerPeripheral> {
		let pwm_pin = PwmPin::new_ch1(pwm_pin, OutputType::PushPull);

		let mut pwm = SimplePwm::new(
			timer,
			Some(pwm_pin),
			None,
			None,
			None,
			hz(50),
			CountingMode::default(),
		);

		pwm.enable(Channel::Ch1);
		pwm.enable(Channel::Ch2);

		Self { pwm }
	}

	/// Returns the actual maximum duty
	pub fn get_max_duty(&self) -> u32 {
		// self.pwm.get_max_duty()
		53320
	}

	/// Returns the actual maximum duty
	pub fn get_duty_unit(&self) -> u32 {
		// self.pwm.get_max_duty()
		53320 / 20
	}

	/// Changes the motor speed by a percentage
	fn set_duty(&mut self, duty: u32) -> &mut Self {
		self.pwm.set_duty(Channel::Ch1, duty);

		self
	}

	/// Changes the motor speed by a percentage
	pub fn set_angle(&mut self, angle: u8) -> &mut Self {
		// Asserts the number is between 1 and 100
		assert!(angle <= 180);

		let max_duty = self.pwm.get_max_duty();
		// for 0.5-2.5ms duty range
		// let duty_range = (max_duty * 25 / 1000, max_duty * 125 / 1000);

		// for 1-2ms duty range
		let duty_range = (max_duty * 25 / 1000, max_duty * 125 / 1000);

		let duty = map_range((0, 180), duty_range, angle as u32);

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
