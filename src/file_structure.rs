use crate::lumps::LumpType;

#[derive(Debug, Clone)]
pub struct BSPFile {
	pub header: Header,
	pub lump_data: Vec<LumpType>,
}

impl BSPFile {
	pub fn new() -> BSPFile {
		BSPFile {
			header: Header::new(),
			lump_data: vec![],
		}
	}
}

#[derive(Debug, Clone, Copy)]
pub struct Header {
	pub ident: i32, // "VBSP" little-endian: 0x50534256
	pub version: i32, // file version
	pub lumps: [LumpInfo; 64], // lump info array
	pub map_revision: i32, // map version number
}

impl Header {
	pub fn new() -> Header {
		Header {
			ident: 0,
			version: 0,
			lumps: [LumpInfo::new(); 64],
			map_revision: 0,
		}
	}
}

#[derive(Debug, Clone, Copy)]
pub struct LumpInfo {
	pub file_offset: u32, // offset into file (bytes)
	pub length: u32, // size of lump (bytes)
	pub version: u32, // lump format version (usually 0)
	pub ident: [u8; 4], // lump ident code (usually [0, 0, 0, 0])

	// not actually in the file ill just use this so i can sort by file offset
	// and still read anything correctly
	pub index: u8,
}

impl LumpInfo {
	pub fn new() -> LumpInfo {
		LumpInfo {
			file_offset: 0,
			length: 0,
			version: 0,
			ident: [0; 4],
			index: 0,
		}
	}
}

// TODO: add LZMA header
