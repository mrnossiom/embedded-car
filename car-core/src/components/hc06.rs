//! `HC-06` bluetooth module driver

use car_transport::{Answer, Message, Transport};
use defmt::trace;
use embassy_stm32::{
	usart::{self, BasicInstance, Config, Uart},
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
		Irq: Peripheral<P = UartInstance::Interrupt> + 'a,
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

		let mut uart = Uart::new(peripheral, rx, tx, irq, tx_dma, rx_dma, config);

		Self { uart }
	}

	///
	pub async fn ping(&mut self) -> Result<bool, Hc06Error> {
		let mut buffer = [0_u8; Answer::BUFFER_SIZE];
		let length = Answer::Pong.serialize(&mut buffer);

		self.uart.write(&buffer[..length]).await?;

		trace!("Sent Pong");
		let mut buffer = [0_u8; Message::BUFFER_SIZE];
		self.uart.read(&mut buffer).await?;

		let message = Message::deserialize(&buffer).unwrap();

		Ok(message == Message::Ping)
	}
}

/// Represents a `HC-06` bluetooth module error.
#[derive(defmt::Format)]
pub enum Hc06Error {
	/// The module did not respond with `OK` or the right answer for AT commands.
	NotOkResponse,

	/// There was a problem with the UART communication itself.
	USArt(usart::Error),
}

impl From<usart::Error> for Hc06Error {
	fn from(error: usart::Error) -> Self {
		Self::USArt(error)
	}
}
