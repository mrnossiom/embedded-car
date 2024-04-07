//! Embedded Car controller, includes communication logic and control logic

pub(crate) mod bluetooth;
pub(crate) mod gamepad;

pub use bluetooth::Bluetooth;
pub use gamepad::Controller;
