use crate::lumps::lumptype::Lumps;

#[derive(Debug, Clone, Copy)]
pub enum BSPVersion {
	None,
	VBSP, GoldSrc,
	Quake, // TODO: Quake 2
}

#[derive(Debug, Clone)]
pub struct BSPFile {
	pub header: Header,
	pub lump_data: Lumps,
}

impl BSPFile {
	pub fn new(header: Header) -> BSPFile {
		BSPFile {
			header: header,
			lump_data: match header.bspver {
				BSPVersion::VBSP => Lumps::VBSP(vec![]),
				BSPVersion::GoldSrc => Lumps::GoldSrc(vec![]),
				BSPVersion::Quake => Lumps::Quake(vec![]),
				BSPVersion::None => { panic!("no") }
			},
		}
	}
}

#[derive(Debug, Clone, Copy)]
pub struct Header {
	pub bspver: BSPVersion, // store what engine this is

	pub ident: i32, // magic number
	pub version: i32, // file version (VBSP ONLY)
	pub lumps: [LumpInfo; 64], // lump info array (64 for VBSP, 15 for GoldSrc and Quake)
	pub map_revision: i32, // map version number (VBSP ONLY)
}

impl Header {
	pub fn new() -> Header {
		Header {
			bspver: BSPVersion::None,
			ident: 0,
			version: 0,
			lumps: [LumpInfo::new(); 64],
			map_revision: 0,
		}
	}
}

#[derive(Debug, Clone, Copy)]
// these differ between vesions
// i'll keep all fields here to not deal with enums again
pub struct LumpInfo {
	pub file_offset: u32, // offset into file (bytes)
	pub length: u32, // size of lump (bytes)

	// VBSP ONLY
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
