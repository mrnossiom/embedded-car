//! The implementation of a controller abstraction to control the car

use color_eyre::eyre::{eyre, Context};
use gilrs::{
	ff::{self, BaseEffect, BaseEffectType, Effect, EffectBuilder, Ticks},
	Axis, Gamepad, GamepadId, Gilrs,
};

/// The current state that the car should follow
#[derive(Debug)]
pub struct ControlState {
	/// The speed of the car, between -1.0 and 1.0
	pub speed: f32,
	/// The direction of the car, between -1.0 and 1.0
	pub direction: f32,
}

/// A controller that can be used to control the car
#[derive(Debug)]
pub struct Controller {
	/// The initiated gamepad manager
	pub manager: Gilrs,
	/// The id of the active gamepad
	gamepad_id: GamepadId,
}

impl Controller {
	/// Initialises a new controller
	///
	/// # Errors
	/// In case no gamepad is connected, or the gamepad is not supported
	pub fn new() -> color_eyre::Result<Self> {
		let manager = Gilrs::new().map_err(|err| match err {
			gilrs::Error::InvalidAxisToBtn => eyre!("Invalid axis to button"),
			gilrs::Error::NotImplemented(_) => eyre!("Not implemented for the current platform"),
			gilrs::Error::Other(error) => eyre!("Other error: {}", error),
		})?;

		let gamepad_id = manager
			.gamepads()
			.find(|(_gp_id, gp)| gp.is_connected())
			.map(|(gp_id, _)| gp_id)
			.ok_or(color_eyre::eyre::eyre!("No gamepad connected"))?;

		Ok(Self {
			manager,
			gamepad_id,
		})
	}

	/// Returns the active [`Gamepad`]
	#[must_use]
	#[inline]
	pub fn active_controller(&self) -> Gamepad<'_> {
		self.manager.gamepad(self.gamepad_id)
	}

	/// Process all pending events
	pub fn update(&mut self) {
		// TODO: add filters

		while self.manager.next_event().is_some() {}
	}

	/// Returns the current control state
	#[must_use]
	pub fn state(&self) -> ControlState {
		let gamepad = self.active_controller();

		ControlState {
			speed: gamepad.value(Axis::LeftStickY),
			direction: gamepad.value(Axis::LeftStickX),
		}
	}

	/// Makes the controller buzz
	///
	/// # Errors
	/// In case the force feedback effect could not be created or played
	pub fn buzz(&mut self) -> color_eyre::Result<Effect> {
		let gamepad = self.active_controller();

		let duration = Ticks::from_ms(150);
		let effect = EffectBuilder::new()
			.add_effect(BaseEffect {
				kind: BaseEffectType::Strong { magnitude: 60_000 },
				..Default::default()
			})
			.repeat(ff::Repeat::For(duration))
			.add_gamepad(&gamepad)
			.finish(&mut self.manager)
			.wrap_err("Could not create effect")?;

		effect.play().wrap_err("Could not play effect")?;

		Ok(effect)
	}
}
