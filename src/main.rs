mod clean;
mod read;

use clean::*;
use read::*;
use std::env::{args, current_dir};

/**
 * The entry point of the application.
 */
fn main() {
	let input = match args().nth(1) {
		Some(val) if !val.is_empty() => val,
		_ => return eprintln!("No file path provided!"),
	};

	let buf = current_dir().unwrap().join(input);
	let path = clean_path(buf.to_str().unwrap());

	let content = match read_file(&path) {
		Ok(val) => val,
		Err(error) => {
			return match error {
				ReadError::Open => eprintln!("Could not resolve path: '{}'", path),
				ReadError::Read => eprintln!("Contains invalid content: '{}'", path),
			}
		}
	};

	let lines = content.matches('\n').count() + !content.ends_with('\n') as usize;
	let plural = if lines != 1 { "s" } else { "" };

	println!("'{}' consists of {} line{}!", path, lines, plural);
}
