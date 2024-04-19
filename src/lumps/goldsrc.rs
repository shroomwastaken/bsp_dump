// info fully taken from
// https://developer.valvesoftware.com/w/index.php?title=BSP_(GoldSrc)

use crate::utils::Vector3;

#[derive(Debug, Clone)]
pub enum GoldSrcLumpType {
	None,
	Entities(Vec<Vec<(String, String)>>),
	Planes(Vec<Plane>),
	Textures(Textures),
	Vertices,
	Visibility,
	Nodes,
	TexInfo,
	Faces,
	Lighting,
	ClipNodes,
	Leaves,
	MarkSurfaces,
	Edges,
	SurfEdges,
	Models,
}

#[derive(Debug, Clone)]
pub struct Plane {
	pub normal: Vector3,
	pub dist: f32,
	pub r#type: i32, // see utils::int_to_gsrc_planetype()
}

#[derive(Debug, Clone)]
pub struct Textures {
	pub num_textures: u32,

	// offsets to miptex structures from beginning of lump
	pub offsets: Vec<i32>,
	pub miptexs: Vec<Miptex>,
}

#[derive(Debug, Clone)]
pub struct Miptex {
	pub name: String, // of length 16
	pub width: u32,
	pub height: u32,

	// zeroes if the texture is stored in the wad file
	// if not zero, offsets from the beginning of this struct to the mipmap
	pub offsets: [u32; 4],
}
