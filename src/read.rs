use std::fs::File;
use std::io::Read;

/**
 * Possible errors returned from the `read_file` function.
 */
pub enum ReadError {
	Open,
	Read,
}

/**
 * Reads the content from a specified file.
 */
pub fn read_file(path: &str) -> Result<String, ReadError> {
	let mut file = File::open(path).or(Err(ReadError::Open))?;
	let mut content = String::new();

	file.read_to_string(&mut content).or(Err(ReadError::Read))?;
	Ok(content)
}
