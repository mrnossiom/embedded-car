//! `HC-06` or `HM-10` bluetooth module driver (don't know yet)

use car_transport::{Answer, Message, Transport};
use embassy_stm32::{
	Peri,
	interrupt::typelevel::Binding,
	mode,
	usart::{self, Config, InterruptHandler, Uart},
};

/// Represents a `HC-06` bluetooth module.
pub struct Hc06<'a> {
	/// The underlying UART instance.
	uart: Uart<'a, mode::Async>,
}

impl<'a> Hc06<'a> {
	/// Creates a new `HC-06` handle from the `UART` peripheral and `rx`, `tx` pins.
	/// You can also provide `DMA`s peripherals to enable `Direct Memory Access` transfers.
	///
	/// # Example
	/// ```
	/// let p = embassy_stm32::init(Default::default());
	///
	/// let bluetooth_irq = interrupt::take!(USART1);
	/// let hc06 = Hc06::from_pins(p.USART1, p.PD9, p.PD8, bluetooth_irq, NoDma, NoDma);
	/// ```
	pub fn from_pins<T: usart::Instance>(
		peri: Peri<'a, T>,
		rx: Peri<'a, impl usart::RxPin<T>>,
		tx: Peri<'a, impl usart::TxPin<T>>,
		irq: impl Binding<T::Interrupt, InterruptHandler<T>> + 'a,
		tx_dma: Peri<'a, impl usart::TxDma<T>>,
		rx_dma: Peri<'a, impl usart::RxDma<T>>,
	) -> Hc06<'a> {
		let mut config = Config::default();
		config.baudrate = 9600;

		let uart = Uart::new(peri, rx, tx, irq, tx_dma, rx_dma, config).unwrap();

		Self { uart }
	}

	/// Returns whether a server has successfully answered our ping
	pub async fn ping(&mut self) -> Result<bool, Error> {
		let mut buffer = [0_u8; Answer::BUFFER_SIZE];
		let length = Answer::Pong.serialize(&mut buffer);

		self.uart.write(&buffer[..length]).await?;

		defmt::debug!("Sent Pong");
		let mut buffer = [0_u8; Message::BUFFER_SIZE];
		self.uart.read(&mut buffer).await?;
		defmt::trace!("Received {}", &buffer);

		let message = Message::deserialize(&buffer).map_err(|_| Error::UnableToDeserialize)?;
		defmt::debug!("Received {:?}", &message);

		Ok(message == Message::Ping)
	}

	pub async fn ping_text(&mut self) -> Result<bool, Error> {
		self.uart.write(b"PING").await?;

		defmt::debug!("Sent Pong");

		let mut buffer = [0_u8; 7];
		self.uart.read(&mut buffer).await?;
		defmt::debug!("Received {}", &buffer);

		// let message = Message::deserialize(&buffer).map_err(|_| Error::UnableToDeserialize)?;
		// defmt::debug!("Received {:?}", &message);

		Ok(&buffer == b"PONGFDP")
	}
}

/// Represents a `HC-06` bluetooth module error.
#[derive(defmt::Format)]
pub enum Error {
	/// The module did not respond with `OK` or the right answer for AT commands.
	NotOkResponse,

	/// Could not deserialize the answer
	UnableToDeserialize,

	/// There was a problem with the UART communication itself.
	USArt(usart::Error),
}

impl From<usart::Error> for Error {
	fn from(error: usart::Error) -> Self {
		Self::USArt(error)
	}
}
