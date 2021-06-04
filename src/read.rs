use std::fs::File;
use std::io::Read;

pub enum ReadError {
	Open,
	Read,
}

pub fn read_file(path: &str) -> Result<String, ReadError> {
	let mut file = File::open(path).or(Err(ReadError::Open))?;
	let mut content = String::new();

	file.read_to_string(&mut content).or(Err(ReadError::Read))?;
	Ok(content)
}
