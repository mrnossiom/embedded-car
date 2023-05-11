#![no_std]
#![no_main]

use panic_halt as _;

use cortex_m_rt::entry;
use stm32f1xx_hal::{
	pac,
	prelude::*,
	serial::{Config, Serial},
};

#[entry]
fn main() -> ! {
	let dp = pac::Peripherals::take().unwrap();

	let mut flash = dp.FLASH.constrain();
	let rcc = dp.RCC.constrain();

	let clocks = rcc
		.cfgr
		.use_hse(8.MHz())
		.sysclk(48.MHz())
		.pclk1(24.MHz())
		.freeze(&mut flash.acr);

	assert!(clocks.usbclk_valid());

	// Configure the on-board LED (PC13, green)
	let mut gpioc = dp.GPIOC.split();
	let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);
	led.set_high(); // Turn off

	let mut afio = dp.AFIO.constrain();
	let mut gpiob = dp.GPIOB.split();

	let pin_tx = gpiob.pb6.into_alternate_push_pull(&mut gpiob.crl);
	let pin_rx = gpiob.pb7;

	let serial = Serial::new(
		dp.USART1,
		(pin_tx, pin_rx),
		&mut afio.mapr,
		Config::default()
			.baudrate(9600.bps())
			.wordlength_9bits()
			.parity_none(),
		&clocks,
	);

	loop {
		let mut buf = [0u8; 64];

		match serial.read() {
			Ok(count) if count > 0 => {
				led.set_low(); // Turn on

				// Echo back in upper case
				for c in buf[0..count].iter_mut() {
					if 0x61 <= *c && *c <= 0x7a {
						*c &= !0x20;
					}
				}

				let mut write_offset = 0;
				while write_offset < count {
					match serial.write(&buf[write_offset..count]) {
						Ok(len) if len > 0 => {
							write_offset += len;
						}
						_ => {}
					}
				}
			}
			_ => {}
		}

		led.set_high(); // Turn off
	}
}
