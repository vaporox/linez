mod util;

use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
	let args = env::args().skip(1);
	let (options, paths) = args.partition::<Vec<_>, _>(|e| e.starts_with("--"));

	if paths.is_empty() {
		util::exit("No query specified");
	}

	let input = paths.iter().collect::<PathBuf>();
	let path = util::unwrap(fs::canonicalize(input));

	let content = util::unwrap(fs::read_to_string(&path));
	let lines = content.matches('\n').count() + !content.ends_with('\n') as usize;

	if options.iter().any(|e| e == "--compact") {
		return println!("{}", lines);
	}

	let path = path.to_string_lossy();
	let plural = if lines != 1 { "s" } else { "" };

	println!("'{}' consists of {} line{}!", path, lines, plural);
}
