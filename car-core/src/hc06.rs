//! `HC-06` bluetooth module driver

use defmt::trace;
use embassy_stm32::{
	usart::{self, BasicInstance, Config, Parity, Uart},
	Peripheral,
};
use heapless::String;

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

		let uart = Uart::new(peripheral, rx, tx, irq, tx_dma, rx_dma, config);

		Self { uart }
	}

	/// Change the pin password of the bluetooth module.
	pub fn change_pin(&mut self, pin: &str) -> Result<(), Hc06Error> {
		assert!(pin.len() == 4, "The pin must be 4 characters long");

		self.uart.blocking_write(b"AT+PIN")?;
		self.uart.blocking_write(pin.as_bytes())?;

		self.uart.blocking_flush()?;

		let mut buffer = [0u8; 8];
		self.uart.blocking_read(&mut buffer)?;

		if buffer == *b"OKsetpin" {
			Ok(())
		} else {
			Err(Hc06Error::NotOkResponse)
		}
	}

	/// Change the name of the bluetooth module.
	pub fn change_name(&mut self, name: &str) -> Result<(), Hc06Error> {
		self.uart.blocking_write(b"AT+NAME")?;
		self.uart.blocking_write(name.as_bytes())?;

		// TODO: check module response

		Ok(())
	}

	// TODO: change the baudrate of the UART peripheral
	/// Changes the baudrate of the bluetooth data exchange.
	pub fn change_baud_rate(&mut self, baud_rate: &str) -> Result<(), Hc06Error> {
		self.uart.blocking_write(b"AT+BAUD")?;
		self.uart.blocking_write(baud_rate.as_bytes())?;

		// TODO: check module response

		Ok(())
	}

	// TODO: change the parity check of the UART peripheral
	/// Changes the parity check of the bluetooth module.
	pub fn change_parity_check(&mut self, parity_check: Parity) -> Result<(), Hc06Error> {
		self.uart.blocking_write(b"AT+")?;

		match parity_check {
			Parity::ParityNone => self.uart.blocking_write(b"PN")?,
			Parity::ParityOdd => self.uart.blocking_write(b"PO")?,
			Parity::ParityEven => self.uart.blocking_write(b"PE")?,
		}

		// TODO: check module response

		Ok(())
	}

	///
	pub async fn ping(&mut self) -> Result<(), Hc06Error> {
		self.uart.write(b"AT").await?;

		let mut buffer = [0_u8; 2];

		self.uart.read(&mut buffer).await?;
		let string = buffer.iter().map(|b| *b as char).collect::<String<2>>();

		trace!("AT ping response : {}", string);

		if string == "OK" {
			Ok(())
		} else {
			Err(Hc06Error::NotOkResponse)
		}
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
