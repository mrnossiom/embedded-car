//! Used to configure the bluetooth module

use std::{
	io::{self, ErrorKind, Read, Write, stdin, stdout},
	str,
	time::Duration,
};

use clap::{Parser, ValueEnum};
use serialport::SerialPort;

// setfacl --modify u:(whoami):rw /dev/ttyUSB0

#[derive(Parser)]
struct Args {
	#[clap(long, default_value_t = String::from("/dev/ttyUSB0"))]
	module_tty_path: String,

	#[clap(long)]
	module_kind: ModuleKind,

	#[clap(long, default_value_t = 250)]
	timeout_ms: u64,
}

#[derive(Debug, Clone, ValueEnum)]
enum ModuleKind {
	Hc06,
	Hm10,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let args = Args::parse();

	let mut module = BluetoothModule::new_serial(
		&args.module_kind,
		&args.module_tty_path,
		Duration::from_millis(args.timeout_ms),
	)
	.unwrap();

	loop {
		print!("{:?}> ", &args.module_kind);
		stdout().flush()?;

		let mut input = String::new();
		stdin().read_line(&mut input)?;

		if input.is_empty() {
			break;
		}

		let module_response = module.send_and_read_response(input.trim())?;
		println!("{module_response}");
	}

	Ok(())
}

/// Abstract interaction with bt module
struct BluetoothModule {
	kind: ModuleKind,
	port: Box<dyn SerialPort>,
}

impl BluetoothModule {
	fn new_serial(kind: &ModuleKind, path: &str, timeout: Duration) -> io::Result<Self> {
		let port = serialport::new(path, 9600).timeout(timeout).open().unwrap();
		Ok(Self {
			kind: kind.clone(),
			port,
		})
	}

	fn send_and_read_response(&mut self, msg: &str) -> io::Result<String> {
		let end = match self.kind {
			ModuleKind::Hc06 => "\r\n",
			ModuleKind::Hm10 => "",
		};

		write!(self.port, "{}{}", msg, end)?;
		self.port.flush()?;

		let mut buffer = [0; 20];
		let mut index = 0usize;

		loop {
			match buffer.get(index.saturating_sub(2)..index) {
				// HC06 end-of-transmission?
				Some(b"\r\n") => break,
				_ => match self.port.read(&mut buffer[index..]) {
					Ok(num) => index += num,
					// HM10 has no end marker
					Err(err) if err.kind() == ErrorKind::TimedOut => break,
					Err(err) => return Err(err),
				},
			}
		}

		let response = str::from_utf8(&buffer[..index]).unwrap();

		Ok(response.to_string())
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
