#![warn(
	clippy::missing_docs_in_private_items,
	clippy::unwrap_used,
	clippy::nursery,
	clippy::pedantic,
	clippy::cargo
)]

//! Embedded Car controller, includes communication logic and control logic

pub(crate) mod bluetooth;
pub(crate) mod gamepad;

pub use bluetooth::Bluetooth;
pub use gamepad::Controller;
