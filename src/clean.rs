use regex::Regex;

const SINGLE: &str = r"/(?:\./)+";
const DOUBLE: &str = r"[^/]+/\.\./";

pub fn clean_path(path: &str) -> String {
	let single_regex = Regex::new(SINGLE).unwrap();
	let double_regex = Regex::new(DOUBLE).unwrap();

	let mut result = single_regex.replace_all(path, "/").to_string();

	while double_regex.is_match(&result) {
		result = double_regex.replace_all(&result, "").to_string();
	}

	result
}
