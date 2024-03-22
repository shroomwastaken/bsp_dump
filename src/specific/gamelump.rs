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

}
