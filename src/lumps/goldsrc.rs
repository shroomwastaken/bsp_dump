// info fully taken from
// https://developer.valvesoftware.com/w/index.php?title=BSP_(GoldSrc)

use crate::utils::Vector3;
use crate::flags::GoldSrcContentsFlags;

#[derive(Debug, Clone)]
pub enum GoldSrcLumpType {
	None,
	Entities(Vec<Vec<(String, String)>>),
	Planes(Vec<Plane>),
	Textures(Textures),
	Vertices(Vec<Vector3>),
	Visibility,
	Nodes(Vec<Node>),
	TexInfo(Vec<TexInfo>),
	Faces(Vec<Face>),
	Lighting(Vec<Lightmap>),
	ClipNodes(Vec<ClipNodes>),
	Leaves(Vec<Leaf>),
	MarkSurfaces(Vec<u16>),
	Edges(Vec<[u16; 2]>),
	SurfEdges(Vec<i32>),
	Models(Vec<Model>),
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

#[derive(Debug, Clone)]
pub struct Node {
	pub plane_idx: u32,

	// if > 0, index into nodes lump
	// else bitwise inverse indices into leafs
	pub children_idxs: [i16; 2],

	pub mins: [i16; 3],
	pub maxs: [i16; 3],
	pub first_face: u16,
	pub num_faces: u16,
}

#[derive(Debug, Clone)]
pub struct TexInfo {
	pub s: Vector3, // direction
	pub s_shift: f32, // texture shift in s direction
	pub t: Vector3, // direction
	pub t_shift: f32, // texture shift in t direction
	pub miptex_idx: u32, // index into textures
	pub flags: u32, // flags
}

#[derive(Debug, Clone)]
pub struct Face {
	pub plane_idx: u16, // plane this face is parallel to
	pub plane_side: u16, // set if differs normal orientation
	pub first_surfedge_idx: u32,
	pub num_surfedges: u16,
	pub texinfo_idx: u16,
	pub styles: [u8; 4],

	// offset into raw lightmap data
	// if < 0 no lightmap was baked for this face
	pub lightmap_offset: i32,
}

#[derive(Debug, Clone)]
pub struct Lightmap {
	pub color: [u8; 3],
}

#[derive(Debug, Clone)]
pub struct ClipNodes {
	pub plane_idx: i32,

	// can be negative idk what that means
	pub children_idxs: [i16; 2]
}

#[derive(Debug, Clone)]
pub struct Leaf {
	pub contents: GoldSrcContentsFlags,
	pub vis_ofs: i32,
	pub mins: [i16; 3],
	pub maxs: [i16; 3],
	pub first_maksurf_idx: u16,
	pub num_marksurfaces: u16,
	pub ambient_levels: [u8; 4],
}

#[derive(Debug, Clone)]
pub struct Model {
	pub mins: Vector3,
	pub maxs: Vector3,
	pub origin: Vector3,
	pub headnodes_idx: [i32; 4],
	pub num_visleafs: i32,
	pub first_face_idx: i32,
	pub num_faces: i32,
}
