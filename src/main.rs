mod reader;
mod file_structure;
mod utils;
mod lumps;
mod parse;
mod dump;
mod specific;
mod flags;

use std::{fs, env};
use reader::Reader;

const VERSION: &str = "v0.0.1";

fn main() {
    let args: Vec<String> = env::args()
	.collect();

	let file: Vec<u8> = fs::read(args[1].clone())
	.unwrap_or_else(|e| {
		println!("error while opening file: {e}");
		std::process::exit(0);
	});

	let mut reader: Reader = Reader::new(file);
	let file = parse::parse_file(&mut reader);

	dump::dump_file(args[1].clone(), file)
}
