//! `HC-06` or `HM-10` bluetooth module driver (don't know yet)

use car_transport::{Answer, Message, Transport};
use defmt::{assert_eq, unwrap};
use embassy_stm32::{
	interrupt::typelevel::Binding,
	usart::{self, BasicInstance, Config, InterruptHandler, Uart},
	Peripheral,
};

/// Represents a `HC-06` bluetooth module.
pub struct Hc06<'a, UartInstance, TxDma, RxDma>
where
	UartInstance: BasicInstance,
{
	/// The underlying UART instance.
	uart: Uart<'a, UartInstance, TxDma, RxDma>,
}

impl<'a, UartInstance, TxDma, RxDma> Hc06<'a, UartInstance, TxDma, RxDma>
where
	UartInstance: BasicInstance,

	TxDma: usart::TxDma<UartInstance>,
	RxDma: usart::RxDma<UartInstance>,
{
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
	pub fn from_pins<
		UartPeripheral: Peripheral<P = UartInstance> + 'a,
		TxPin: usart::TxPin<UartInstance>,
		RxPin: usart::RxPin<UartInstance>,
		Irq: Binding<UartInstance::Interrupt, InterruptHandler<UartInstance>> + 'a,
		TxDmaPeripheral: Peripheral<P = TxDma> + 'a,
		RxDmaPeripheral: Peripheral<P = RxDma> + 'a,
	>(
		peripheral: UartPeripheral,
		tx: TxPin,
		rx: RxPin,
		irq: Irq,
		tx_dma: TxDmaPeripheral,
		rx_dma: RxDmaPeripheral,
	) -> Hc06<'a, UartInstance, TxDma, RxDma> {
		let mut config = Config::default();
		config.baudrate = 9600;

		let uart = Uart::new(peripheral, rx, tx, irq, tx_dma, rx_dma, config).unwrap();

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
