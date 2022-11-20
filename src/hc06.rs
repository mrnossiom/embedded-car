//! `HC-06` bluetooth module driver

use core::panic;
use embassy_stm32::{
	usart::{self, BasicInstance, Config, Parity, Uart},
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
	pub fn change_pin(&mut self, pin: &str) -> Result<(), usart::Error> {
		assert!(pin.len() == 4, "The pin must be 4 characters long");

		self.uart.blocking_write(b"AT+PIN")?;
		self.uart.blocking_write(pin.as_bytes())?;

		self.uart.blocking_flush()?;

		let mut buffer = [0u8; 8];
		self.uart.blocking_read(&mut buffer)?;

		if buffer == *b"OKsetpin" {
			Ok(())
		} else {
			panic!("Failed to change pin");
		}
	}

	/// Change the name of the bluetooth module.
	pub fn change_name(&mut self, name: &str) -> Result<(), usart::Error> {
		self.uart.blocking_write(b"AT+NAME")?;
		self.uart.blocking_write(name.as_bytes())?;

		Ok(())
	}

	// TODO: change the baudrate of the UART peripheral
	/// Changes the baudrate of the bluetooth data exchange.
	pub fn change_baud_rate(&mut self, baud_rate: &str) -> Result<(), usart::Error> {
		self.uart.blocking_write(b"AT+BAUD")?;
		self.uart.blocking_write(baud_rate.as_bytes())?;

		Ok(())
	}

	// TODO: change the parity check of the UART peripheral
	/// Changes the parity check of the bluetooth module.
	pub fn change_parity_check(&mut self, parity_check: Parity) -> Result<(), usart::Error> {
		self.uart.blocking_write(b"AT+")?;

		match parity_check {
			Parity::ParityNone => self.uart.blocking_write(b"PN")?,
			Parity::ParityOdd => self.uart.blocking_write(b"PO")?,
			Parity::ParityEven => self.uart.blocking_write(b"PE")?,
		}

		match parity_check {
			Parity::ParityNone => {
				let mut buffer = [0u8; 7];
				self.uart.blocking_read(&mut buffer)?;

				if buffer == *b"OK NONE" {
					Ok(())
				} else {
					panic!("Failed to disable parity check");
				}
			}
			Parity::ParityOdd => {
				let mut buffer = [0u8; 6];
				self.uart.blocking_read(&mut buffer)?;

				if buffer == *b"OK ODD" {
					Ok(())
				} else {
					panic!("Failed to enable odd parity check");
				}
			}
			Parity::ParityEven => {
				let mut buffer = [0u8; 7];
				self.uart.blocking_read(&mut buffer)?;

				if buffer == *b"OK EVEN" {
					Ok(())
				} else {
					panic!("Failed to enable even parity check");
				}
			}
		}
	}
}
