//! `HC-SR04` ultrasonic sensor driver

use embassy_stm32::{
	exti::ExtiInput,
	gpio::{Input, Level, Output, Pin, Pull, Speed},
	Peripheral,
};
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
	echo: ExtiInput<'a, EchoPin>,
}

impl<'a, TriggerPin, EchoPin> HcSr04<'a, TriggerPin, EchoPin>
where
	TriggerPin: Pin,
	EchoPin: Pin,
{
	/// Creates a `HC-SCR04` sensor handle from the trigger and echo pins.
	pub fn from_pins<IrqChannel: Peripheral<P = EchoPin::ExtiChannel> + 'a>(
		trigger: TriggerPin,
		echo: EchoPin,
		channel: IrqChannel,
	) -> HcSr04<'a, TriggerPin, EchoPin> {
		let trigger = Output::new(trigger, Level::Low, Speed::Low);
		let echo = Input::new(echo, Pull::None);
		let echo = ExtiInput::new(echo, channel);

		Self { trigger, echo }
	}

	/// Returns the distance in centimeters (`cm`).
	pub async fn ping_distance(&mut self) -> u64 {
		let ping_duration = self.ping().await;

		/// Constant to convert the duration of the echo to a distance in centimeters (`cm`).
		/// (`343.21m/s` / `1000` (speed of light in cm/us)) / `2` (round trip)
		const SOUND_US_TO_MM: f64 = (343.21 / 10_000.0) / 2.0;

		(ping_duration as f64 * SOUND_US_TO_MM) as u64
	}

	/// Returns the duration of the echo in microseconds (`us`).
	pub async fn ping(&mut self) -> u64 {
		self.trigger.set_high();
		Timer::after(Duration::from_micros(10)).await;
		self.trigger.set_low();

		// Wait for any old echo to finish
		self.echo.wait_for_low().await;

		// Wait for an echo
		self.echo.wait_for_high().await;

		let start = Instant::now();

		// Wait for echo to end
		self.echo.wait_for_low().await;

		start.elapsed().as_micros()
	}
}
