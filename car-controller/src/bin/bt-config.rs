//! Used to configure the bluetooth module

use std::{
	io::{self, Read, Write},
	time::Duration,
};

use serialport::SerialPort;

// setfacl --modify u:(whoami):rw /dev/ttyUSB0

/// Path to the USB tty
const BT_TTY_PATH: &str = "/dev/ttyUSB0";

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let module = Hc06::new(BT_TTY_PATH).unwrap();

	Ok(())
}

/// Abstract interaction with bt module
struct Hc06 {
	port: Box<dyn SerialPort>,
}

impl Hc06 {
	/// Init
	fn new(path: &str) -> io::Result<Self> {
		let mut port = serialport::new(path, 9600)
			.timeout(Duration::from_millis(1500))
			.baud_rate(9600)
			.open()
			.unwrap();

		write!(port, "AT")?;
		let mut serial_buf: [u8; 2] = [0; 2];
		port.read_exact(&mut serial_buf).unwrap();
		assert_eq!(&serial_buf, b"OK");

		Ok(Self { port })
	}

	fn send_command(cmd: AtCommand) {}
}

// AT+NAMERenaultClio
// AT+PIN1844

/// AT command set to configure the bluetooth module
enum AtCommand {
	/// `AT`
	///
	/// => `OK`
	Ping,

	/// `AT+BAUD{id}`
	///
	/// => `OK{baud_value}`
	SetBaudrate(u8),

	/// `AT+NAME{new_name}`
	/// String is max 20 characters
	///
	/// => `OK{name}`
	SetName(String),

	/// `AT+PIN{4 pin code}`
	///
	/// => `OKsetpin`
	SetPin(u16),

	/// `AT+PN`
	///
	/// => `OK NONE`
	SetNoParityCheck,

	/// `AT+PO`
	///
	/// => `OK ODD`
	SetOddParityCheck,

	/// `AT+PE`
	///
	/// => `OK EVEN`
	SetEvenParityCheck,

	/// `AT+VERSION`
	///
	/// => `{id and version}`
	GetVersion,
}
