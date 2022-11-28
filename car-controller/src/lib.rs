#![warn(clippy::missing_docs_in_private_items)]

//! Embedded Car controller, includes communication logic and control logic

pub(crate) mod bluetooth;

pub use bluetooth::CarBluetooth;
