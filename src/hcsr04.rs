//! `HC-SR04` ultrasonic sensor driver

use embassy_stm32::gpio::{Input, Level, Output, Pin, Pull, Speed};
use embassy_time::{Duration, Instant, Timer};

// TODO: think about a timer to limit ping calls to 1 per 60ms, to ensure echo from previous pings is not returned
/// Represents a `HC-SR04` ultrasonic sensor.
///
/// No need here for a kind of `waiting_for_echo` flag, because the
/// borrow checker ensures there are only one mutable reference.
pub struct HcSr04<'a, TriggerPin, EchoPin>
where
	TriggerPin: Pin,
	EchoPin: Pin,
{
	/// The pin that triggers the ping.
	trigger: Output<'a, TriggerPin>,
	/// The pin that receives the echo.
	echo: Input<'a, EchoPin>,
}

impl<'a, TriggerPin, EchoPin> HcSr04<'a, TriggerPin, EchoPin>
where
	TriggerPin: Pin,
	EchoPin: Pin,
{
	/// Creates a `HC-SCR04` sensor handle from the trigger and echo pins.
	pub fn from_pins(trigger: TriggerPin, echo: EchoPin) -> HcSr04<'a, TriggerPin, EchoPin> {
		let trigger = Output::new(trigger, Level::Low, Speed::Low);
		let echo = Input::new(echo, Pull::None);

		Self { trigger, echo }
	}

	/// Returns the distance in centimeters (cm).
	pub async fn ping_distance(&mut self) -> u32 {
		let ping_duration = self.ping().await;

		// TODO: explain more
		// Magic number is from the datasheet
		ping_duration / 58
	}

	/// Returns the duration of the echo in microseconds (us).
	pub async fn ping(&mut self) -> u32 {
		self.trigger.set_high();
		Timer::after(Duration::from_micros(10)).await;
		self.trigger.set_low();

		// Wait for any old echo to finish
		while self.echo.is_high() {}

		// Wait for an echo
		while self.echo.is_low() {}

		let start = Instant::now();

		// Wait for echo to end
		while self.echo.is_high() {}

		start.elapsed().as_micros() as u32
	}
}
