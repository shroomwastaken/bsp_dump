#[derive(Debug, Clone)]
pub struct GameLumpHeader {
	pub lump_count: i32,
	pub game_lump_info: Vec<GameLumpInfo> // of length
}

#[derive(Debug, Clone, Copy)]
pub struct GameLumpInfo {
	pub id: i32,
	pub flags: u16,
	pub version: u16,

	// offset from beginning of file
	// (except for console portal 2, there its from beginning of this lump)
	pub file_offset: i32,

	pub file_length: i32,
}

#[derive(Debug, Clone)]
pub enum GameLumpData {
	StaticProps(StaticProps),
}

// sprp

#[derive(Debug, Clone)]
pub struct StaticProps {
	pub dict: StaticPropDictLump,
	pub leafs: StaticPropLeafLump,
	pub num_entries: i32,
	// pub entries: Vec<StaticPropLump>,
}

#[derive(Debug, Clone)]
pub struct StaticPropDictLump {
	pub dict_entries: i32,

	// this vector is of length dict_entries
	// the Strings are all null-padded to 128 bytes
	pub names: Vec<String>, // model names
}

#[derive(Debug, Clone)]
pub struct StaticPropLeafLump {
	pub leaf_entries: i32,

	// this vector is of length leaf_entries
	pub leafs: Vec<u16>,
}