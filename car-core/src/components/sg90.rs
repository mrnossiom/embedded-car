//! `SG-90` servo motor driver

use embassy_stm32::{
	pwm::{
		simple_pwm::{PwmPin, SimplePwm},
		CaptureCompare16bitInstance, Channel, Channel1Pin,
	},
	time::hz,
	Peripheral,
};

/// Represents a small `SG-90` servo motor.
pub struct Sg90<'a, TimerPeripheral>
where
	TimerPeripheral: CaptureCompare16bitInstance,
{
	/// The underlying `PWM` to control the servo motor
	pwm: SimplePwm<'a, TimerPeripheral>,
}

impl<'a, TimerPeripheral> Sg90<'a, TimerPeripheral>
where
	TimerPeripheral: CaptureCompare16bitInstance,
{
	/// Creates a `SG90` servo handle from the pwn pin.
	pub fn from_pin<
		Timer: Peripheral<P = TimerPeripheral> + 'a,
		PwmA: Channel1Pin<TimerPeripheral>,
	>(
		pwm_pin: PwmA,
		timer: Timer,
	) -> Sg90<'a, TimerPeripheral> {
		let pwm_pin = PwmPin::new_ch1(pwm_pin);
		let pwm = SimplePwm::new(timer, Some(pwm_pin), None, None, None, hz(50));

		Self { pwm }
	}

	/// Returns the actual maximum duty
	pub fn get_max_duty(&self) -> u16 {
		self.pwm.get_max_duty() - 1
	}

	/// Changes the motor speed by a percentage
	pub fn set_duty(&mut self, duty: u16) -> &mut Self {
		self.pwm.set_duty(Channel::Ch1, duty);

		self
	}

	/// Changes the motor speed by a percentage
	pub fn set_duty_percentage(&mut self, duty: u8) -> &mut Self {
		// Asserts the number is between 1 and 100
		assert!(duty <= 100);

		// `checked_div` is used to allow using 0 as a percentage
		self.set_duty(
			self.get_max_duty()
				.checked_div(u16::from(duty))
				.unwrap_or(0),
		);

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
