use berlewelch::{*, field::Gfe29};
use rustyline::{error::ReadlineError, Editor};
use std::str::FromStr;

fn main() {
	let mut rl = Editor::new().expect("Failed to create readline");
	let action: Result<i32, _> = input(&mut rl, "Encode [1] or Decode [2]?: ");
	let errors = loop {
		if let Ok(errors) = input::<usize>(&mut rl, "Error Tolerance (k): ") {
			break errors;
		} else {
			println!("Try again, must be a valid non-negative integer");
		}
	};
	let message: String = input(&mut rl, "Message (C29 Encoding): ").unwrap();
	match action {
		Ok(1) => {
			let gfe_message = if let Some(message) = c29_to_gfe(&message) {
				message
			} else {
				println!("Invalid message entered, exiting.");
				return;
			};
			let encoded = encode(errors, &gfe_message);
			let text_encoded = gfe_to_c29(&encoded).unwrap();
			println!("Encoded: \"{text_encoded}\"");
		}
		Ok(2) => {
			let mut gfe_message = if let Some(message) = c29_to_gfe(&message) {
				message
			} else {
				println!("Invalid message entered, exiting.");
				return;
			};
			if let Err(_e) = decode(errors, &mut gfe_message) {
				println!("Failed to decode messsage");
				return;
			};
			let text_decoded = gfe_to_c29(&gfe_message[..(gfe_message.len() - 2 * errors)]).unwrap();
			println!("Decoded: \"{text_decoded}\"");
		}
		_ => println!("Unknown action"),
	}
}

fn input<T: FromStr>(rl: &mut Editor<()>, msg: &str) -> Result<T, <T as FromStr>::Err> {
	loop {
		match rl.readline(msg) {
			Ok(line) => break line.parse(),
			Err(ReadlineError::Interrupted) => {}
			Err(ReadlineError::Eof) => {}
			Err(e) => eprintln!("\nError: {e}"),
		}
	}
}

fn ascii_to_c29(ascii: u8) -> Option<u8> {
	Some(match ascii {
		b' ' => 26,
		b'.' => 27,
		b'_' => 28,
		a if b'a' <= a && a <= b'z' => a - b'a',
		_ => return None,
	})
}

fn c29_to_ascii(c29: u8) -> Option<u8> {
	Some(match c29 {
		26 => b' ',
		27 => b'.',
		28 => b'_',
		x if c29 <= 25 => b'a' + x,
		_ => return None,
	})
}

fn c29_to_gfe(msg: &str) -> Option<Vec<Gfe29>> {
	msg.as_bytes()
		.iter()
		.map(|&x| ascii_to_c29(x).map(|x| Gfe29::from(x as i64)))
		.collect::<Option<Vec<_>>>()
}

fn gfe_to_c29(msg: &[Gfe29]) -> Option<String> {
	msg.iter()
		.map(|&x| u8::try_from(*x).ok().and_then(|x| c29_to_ascii(x)))
		.collect::<Option<Vec<u8>>>()
		.map(|bytes| String::from_utf8(bytes).unwrap())
}
