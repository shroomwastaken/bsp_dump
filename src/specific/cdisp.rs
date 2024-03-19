// copied from
// https://github.com/ValveSoftware/source-sdk-2013/blob/master/sp/src/public/bspfile.h#L583
#[derive(Debug, Clone, Copy)]
pub struct CDispSubNeighbor {
	pub neighbor: u16, // index into dispinfo
	pub neighbor_orientation: u8, // rotation of the neighbor
	// where the neighbor fits into this side of the disp
	pub span: u8,
	pub neighbor_span: u8, // where it fits onto the neighbor
	pub padding: u8,
}

#[derive(Debug, Clone, Copy)]
pub struct CDispNeighbor {
	pub sub_neighbors: [CDispSubNeighbor; 2],
}

#[derive(Debug, Clone, Copy)]
pub struct CDispCornerNeighbors {
	pub neighbors: [u16; 4],
	pub num_neighbors: u8,
	pub padding: u8,
}
