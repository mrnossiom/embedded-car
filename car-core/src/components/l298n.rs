//! Manages a new L298N a Dual H-Bridge Motor Controller module
//!
//! You may need the `L298` and `L298N` datasheet to understand this module.
//! Both are available in the [`hardware-specs`](https://github.com/mrnossiom/embedded-car/tree/main/hardware-specs) folder in the repository.

use embassy_stm32::{
	Peri,
	gpio::{Level, Output, OutputType, Pin, Speed},
	time::hz,
	timer::{
		Channel1Pin, Channel2Pin, GeneralInstance4Channel,
		low_level::CountingMode,
		simple_pwm::{PwmPin, SimplePwm},
	},
};

/// Manages a new L298N a Dual H-Bridge Motor Controller module
pub struct L298N<'a, TimerPin: GeneralInstance4Channel> {
	/// The left motor controller
	left: SingleMotor<'a>,
	/// The right motor controller
	right: SingleMotor<'a>,

	/// The `PWM` to control the speed of the motors
	/// `Ch1` is used for the left motor
	/// `Ch2` is used for the right motor
	pwm: SimplePwm<'a, TimerPin>,
}

impl<'a, TimerPin: GeneralInstance4Channel> L298N<'a, TimerPin> {
	/// Creates a new `L298N` motor controller
	pub fn from_pins(
		in1: Peri<'a, impl Pin>,
		in2: Peri<'a, impl Pin>,
		pwm_left: Peri<'a, impl Channel1Pin<TimerPin> + 'a>,

		in3: Peri<'a, impl Pin>,
		in4: Peri<'a, impl Pin>,
		pwm_right: Peri<'a, impl Channel2Pin<TimerPin> + 'a>,

		timer: Peri<'a, TimerPin>,
	) -> L298N<'a, TimerPin> {
		let pwm_left = PwmPin::new_ch1(pwm_left, OutputType::PushPull);
		let pwm_right = PwmPin::new_ch2(pwm_right, OutputType::PushPull);
		let mut pwm = SimplePwm::new(
			timer,
			Some(pwm_left),
			Some(pwm_right),
			None,
			None,
			hz(50),
			CountingMode::default(),
		);

		// Set both motors to full power
		pwm.ch1().set_duty_cycle_fully_on();
		pwm.ch2().set_duty_cycle_fully_on();

		L298N {
			left: SingleMotor::from_pins(in1, in2),
			right: SingleMotor::from_pins(in3, in4),
			pwm,
		}
	}

	/// Makes the motor forward direction
	/// with Ven = H then C = H ; D = L Forward
	pub fn forward(&mut self) -> &mut Self {
		self.left.forward();
		self.right.forward();
		self
	}

	/// Makes the motor reverse direction
	/// with Ven = H then C = L ; D = H Reverse
	pub fn reverse(&mut self) -> &mut Self {
		self.left.reverse();
		self.right.reverse();
		self
	}

	/// Brakes the motor (Fast Motor Stop)
	pub fn brake(&mut self) -> &mut Self {
		self.left.brake();
		self.right.brake();
		self
	}

	/// Stops the motor and sets `PWM` duty to 0 for both motors. (Free Running Motor Stop)
	pub fn stop(&mut self) -> &mut Self {
		self.pwm.ch1().set_duty_cycle_fully_off();
		self.pwm.ch2().set_duty_cycle_fully_off();

		self.left.stop();
		self.right.stop();
		self
	}

	/// Returns the actual maximum duty
	pub fn get_max_duty(&self) -> u16 {
		self.pwm.max_duty_cycle() - 1
	}

	/// Changes the motor speed by a percentage
	pub fn set_duty(&mut self, duty_left: Option<u16>, duty_right: Option<u16>) -> &mut Self {
		if let Some(duty) = duty_left {
			self.pwm.ch1().set_duty_cycle(duty);
		}
		if let Some(duty) = duty_right {
			self.pwm.ch2().set_duty_cycle(duty);
		}
		self
	}

	/// Changes the motor speed by a percentage
	pub fn set_duty_percentage(
		&mut self,
		duty_left: Option<u8>,
		duty_right: Option<u8>,
	) -> &mut Self {
		// Asserts the number is between 1 and 100
		assert!(duty_left.map_or(true, |duty| duty <= 100));
		assert!(duty_right.map_or(true, |duty| duty <= 100));

		// `checked_div` is used to allow using 0 as a percentage
		self.set_duty(
			duty_left.map(|duty| self.get_max_duty().checked_div(duty.into()).unwrap_or(0)),
			duty_right.map(|duty| self.get_max_duty().checked_div(duty.into()).unwrap_or(0)),
		);

		self
	}
}

/// Manages a single motor
pub struct SingleMotor<'a> {
	/// The first control pin
	pub(crate) in_a: Output<'a>,
	/// The second control pin
	pub(crate) in_b: Output<'a>,
}

impl<'a> SingleMotor<'a> {
	/// Creates a new `SingleMotor` from the two control pins.
	fn from_pins(in_a: Peri<'a, impl Pin>, in_b: Peri<'a, impl Pin>) -> Self {
		SingleMotor {
			in_a: Output::new(in_a, Level::Low, Speed::Low),
			in_b: Output::new(in_b, Level::Low, Speed::Low),
		}
	}

	/// Makes the motor forward direction
	/// with Ven = H then C = H ; D = L Forward
	pub fn forward(&mut self) -> &mut Self {
		self.in_a.set_low();
		self.in_b.set_high();
		self
	}

	/// Makes the motor reverse direction
	/// with Ven = H then C = L ; D = H Reverse
	pub fn reverse(&mut self) -> &mut Self {
		self.in_a.set_high();
		self.in_b.set_low();
		self
	}

	/// Brakes the motor - Fast Motor Stop
	/// with Ven = H then C = D Fast Motor Stop
	pub fn brake(&mut self) -> &mut Self {
		self.in_a.set_high();
		self.in_b.set_high();
		self
	}

	/// Stops the motor - Free Running Motor Stop
	/// Ven = L then with C = X ; D = X
	pub fn stop(&mut self) -> &mut Self {
		self.in_a.set_high();
		self.in_b.set_high();
		self
	}
}
