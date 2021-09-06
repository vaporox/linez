use cli::Args;
use std::fs;
use std::path::PathBuf;

fn main() {
	let args = Args::parse();

	if args.positionals.is_empty() {
		cli::exit("No query specified");
	}

	let input = args.positionals.iter().collect::<PathBuf>();
	let path = cli::unwrap(fs::canonicalize(input));

	let content = cli::unwrap(fs::read_to_string(&path));
	let lines = content.matches('\n').count() + !content.ends_with('\n') as usize;

	if args.options.contains_key("compact") {
		return println!("{}", lines);
	}

	let path = path.to_string_lossy();
	let plural = if lines != 1 { "s" } else { "" };

	println!("'{}' consists of {} line{}!", path, lines, plural);
}
