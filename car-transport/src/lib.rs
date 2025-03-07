//! Implementation of the transport protocol for the car.

#![no_std]

/// A light custom transport protocol template that comes on top of bluetooth or serial communication.
///
/// It transport data in the following format:
/// ```text
/// +------------------+------------------+
/// | sub-type id (1b) | payload (varies) |
/// +------------------+------------------+
/// ```
pub trait Transport: Sized {
	/// Size to allocate for a buffer
	const BUFFER_SIZE: usize = Self::MAX_PAYLOAD_SIZE + 1;

	/// Maximum size of the payload in bytes
	const MAX_PAYLOAD_SIZE: usize;

	/// The unique id of the transport sub-type
	fn id(&self) -> u8;

	/// Encodes the payload into the buffer
	fn encode(&self, buffer: &mut [u8]) -> u8;

	/// Serializes the full message with the sub-type id into the buffer
	fn serialize(&self, buffer: &mut [u8]) -> usize {
		assert!(buffer.len() <= Self::MAX_PAYLOAD_SIZE + 1);

		buffer[0] = self.id();

		let size = self.encode(&mut buffer[1..]);

		usize::from(size) + 1
	}

	/// Deserializes the full message from the buffer
	///
	/// # Errors
	/// The deserialization can fail if the buffer contains invalid protocol data.
	fn deserialize(buffer: &[u8]) -> Result<Self, TransportError>;
}

/// A [`Transport`] related error
#[derive(Debug, PartialEq, Eq)]
pub enum TransportError {
	/// There is no such unique id
	InvalidId,
	/// The payload length is invalid for the given id
	InvalidPayload,
}

/// Messages sent by the controller
#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt_macros::Format))]
pub enum Message {
	/// Ping the car
	///
	/// Car should answer with [`Answer::Pong`]
	Ping,

	/// Get the current speed
	///
	/// Car should answer with [`Answer::Speed`]
	GetSpeed,
	/// Get the current direction
	///
	/// Car should answer with [`Answer::Direction`]
	GetDirection,
	/// Get the current battery level
	///
	/// Car should answer with [`Answer::BatteryLevel`]
	GetBatteryLevel,
	/// Get the last measured distance with the ultrasonic sensor
	///
	/// Car should answer with [`Answer::UltrasonicDistance`]
	GetUltrasonicDistance,

	/// Set the current speed
	///
	/// Car should answer with [`Answer::AckSpeed`]
	SetSpeed(i8),
	/// Set the current direction
	///
	/// Car should answer with [`Answer::AckDirection`]
	SetDirection(i8),
}

impl Transport for Message {
	const MAX_PAYLOAD_SIZE: usize = 1;

	fn id(&self) -> u8 {
		match self {
			Self::Ping => 0,
			Self::GetSpeed => 1,
			Self::GetDirection => 2,
			Self::GetBatteryLevel => 3,
			Self::GetUltrasonicDistance => 4,

			Self::SetSpeed(_) => 100,
			Self::SetDirection(_) => 101,
		}
	}

	fn encode(&self, buffer: &mut [u8]) -> u8 {
		match self {
			Self::Ping
			| Self::GetSpeed
			| Self::GetDirection
			| Self::GetBatteryLevel
			| Self::GetUltrasonicDistance => 0,

			Self::SetSpeed(speed) => {
				buffer[0] = speed.to_be_bytes()[0];
				1
			}
			Self::SetDirection(direction) => {
				buffer[0] = direction.to_be_bytes()[0];
				1
			}
		}
	}

	fn deserialize(buffer: &[u8]) -> Result<Self, TransportError> {
		let message = match buffer[0] {
			0 => Self::GetSpeed,
			1 => Self::GetDirection,
			2 => Self::GetBatteryLevel,
			3 => Self::GetUltrasonicDistance,

			100 => Self::SetSpeed(i8::from_be_bytes([buffer[1]])),
			101 => Self::SetDirection(i8::from_be_bytes([buffer[1]])),

			_ => return Err(TransportError::InvalidId),
		};

		Ok(message)
	}
}

/// Messages sent by the car microcontroller
#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt_macros::Format))]
pub enum Answer {
	/// Acknowledge a ping from the controller
	/// Can also be used to check if there is still a controller connected
	///
	/// Answer to [`Message::Ping`]
	Pong,

	/// Send the current speed
	///
	/// Answer to [`Message::GetSpeed`]
	Speed(i8),
	/// Send the current direction
	///
	/// Answer to [`Message::GetDirection`]
	Direction(i8),
	/// Send the current battery level
	///
	/// Answer to [`Message::GetBatteryLevel`]
	BatteryLevel(u8),
	/// Send the last measured distance with the ultrasonic sensor
	///
	/// Answer to [`Message::GetUltrasonicDistance`]
	UltrasonicDistance(Option<u8>),

	/// Acknowledge the speed change
	///
	/// Answer to [`Message::SetSpeed`]
	AckSpeed,
	/// Acknowledge the direction change
	///
	/// Answer to [`Message::SetDirection`]
	AckDirection,
}

impl Transport for Answer {
	const MAX_PAYLOAD_SIZE: usize = 2;

	fn id(&self) -> u8 {
		match self {
			Self::Pong => 0,

			Self::Speed(_) => 1,
			Self::Direction(_) => 2,
			Self::BatteryLevel(_) => 3,
			Self::UltrasonicDistance(_) => 4,

			Self::AckSpeed => 100,
			Self::AckDirection => 101,
		}
	}

	fn encode(&self, buffer: &mut [u8]) -> u8 {
		match self {
			Self::Speed(speed) => {
				buffer[0] = speed.to_be_bytes()[0];
				1
			}
			Self::Direction(direction) => {
				buffer[0] = direction.to_be_bytes()[0];
				1
			}
			Self::BatteryLevel(voltage) => {
				buffer[0] = *voltage;
				1
			}
			Self::UltrasonicDistance(distance) => {
				if let Some(distance) = distance {
					buffer[0] = 1;
					buffer[1] = *distance;
				}

				2
			}

			Self::Pong | Self::AckSpeed | Self::AckDirection => 0,
		}
	}

	fn deserialize(buffer: &[u8]) -> Result<Self, TransportError> {
		let answer = match buffer[0] {
			0 => Self::Speed(i8::from_be_bytes([buffer[1]])),
			1 => Self::Direction(i8::from_be_bytes([buffer[1]])),
			2 => Self::BatteryLevel(buffer[1]),
			3 => Self::UltrasonicDistance(match buffer[1] {
				0 => None,
				1 => Some(buffer[2]),
				_ => return Err(TransportError::InvalidPayload),
			}),

			100 => Self::AckSpeed,
			101 => Self::AckDirection,

			_ => return Err(TransportError::InvalidId),
		};

		Ok(answer)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn message_max_payload_size_is_right() {
		#[rustfmt::skip]
		let messages ={
			use Message::*;
			[GetSpeed, GetDirection, GetBatteryLevel, GetUltrasonicDistance, SetSpeed(0), SetDirection(0)]
		};

		let mut buffer = [0u8; Message::MAX_PAYLOAD_SIZE + 1];

		let max_length = messages
			.iter()
			.map(|message| message.encode(&mut buffer))
			.max()
			.unwrap();

		assert_eq!(usize::from(max_length), Message::MAX_PAYLOAD_SIZE);
	}

	#[test]
	fn answer_max_payload_size_is_right() {
		#[rustfmt::skip]
		let messages ={
			use Answer::*;
			[Speed(0), Direction(0), BatteryLevel(0), UltrasonicDistance(None), AckSpeed, AckDirection]
		};

		let mut buffer = [0u8; Answer::MAX_PAYLOAD_SIZE];

		let max_length = messages
			.iter()
			.map(|message| message.encode(&mut buffer))
			.max()
			.unwrap();

		assert_eq!(usize::from(max_length), Answer::MAX_PAYLOAD_SIZE);
	}

	#[test]
	fn can_serialize_message() -> Result<(), TransportError> {
		let message = Message::GetSpeed;
		let mut buffer = [0u8; Message::MAX_PAYLOAD_SIZE + 1];

		let length = message.serialize(&mut buffer);
		assert_eq!(length, 1);
		assert_eq!(&buffer[..length], &[0u8]);

		let message = Message::deserialize(&buffer[..length])?;
		assert_eq!(message, Message::GetSpeed);

		Ok(())
	}

	#[test]
	fn can_serialize_message_with_data() -> Result<(), TransportError> {
		let message = Message::SetSpeed(100);
		let mut buffer = [0u8; Message::MAX_PAYLOAD_SIZE + 1];

		let length = message.serialize(&mut buffer);
		assert_eq!(length, 2);
		assert_eq!(&buffer[..length], &[100u8, 100i8.to_be_bytes()[0]]);

		let message = Message::deserialize(&buffer[..length])?;
		assert_eq!(message, Message::SetSpeed(100));

		Ok(())
	}

	#[test]
	fn can_serialize_answer() -> Result<(), TransportError> {
		let answer = Answer::AckSpeed;
		let mut buffer = [0u8; Answer::MAX_PAYLOAD_SIZE + 1];

		let length = answer.serialize(&mut buffer);
		assert_eq!(length, 1);
		assert_eq!(&buffer[..length], &[100u8]);

		let deserialized_message = Answer::deserialize(&buffer[..length])?;
		assert_eq!(deserialized_message, Answer::AckSpeed);

		Ok(())
	}

	#[test]
	fn can_serialize_answer_with_data() -> Result<(), TransportError> {
		let answer = Answer::Direction(100);
		let mut buffer = [0u8; Message::MAX_PAYLOAD_SIZE + 1];

		let length = answer.serialize(&mut buffer);
		assert_eq!(length, 2);
		assert_eq!(&buffer[..length], &[1u8, 100i8.to_be_bytes()[0]]);

		let answer = Answer::deserialize(&buffer[..length])?;
		assert_eq!(answer, Answer::Direction(100));

		Ok(())
	}

	#[test]
	fn can_serialize_message_with_signed_integer() -> Result<(), TransportError> {
		let answer = Answer::Direction(-100);
		let mut buffer = [0u8; Message::MAX_PAYLOAD_SIZE + 1];

		let length = answer.serialize(&mut buffer);
		assert_eq!(length, 2);
		assert_eq!(&buffer[..length], &[1u8, (-100i8).to_be_bytes()[0]]);

		let answer = Answer::deserialize(&buffer[..length])?;
		assert_eq!(answer, Answer::Direction(-100));

		Ok(())
	}
}
