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

const VBSP_MAGIC: i32 = 0x50534256;
const IBSP_MAGIC: i32 = 0x50534249;
const GOLDSRC_MAGIC: i32 = 0x0000001e;

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

	dump::dump(args[1].clone(), file)
}
