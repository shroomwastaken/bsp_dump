// info taken from the quake source code
// https://github.com/id-Software/Quake/blob/master/WinQuake/bspfile.h

use crate::utils::Vector3;
use crate::flags::GoldSrcContentsFlags; // matches quake 1 flags perfectly

#[derive(Debug, Clone)]
pub enum QuakeLumpType {
	None,
	Entities(Vec<Vec<(String, String)>>),
	Planes(Vec<Plane>),
	Textures(Texture),
	Vertices(Vec<Vertex>),
	Visibility,
	Nodes(Vec<Node>),
	TexInfo(Vec<TexInfo>),
	Faces(Vec<Face>),
	Lighting,
	ClipNodes(Vec<ClipNode>),
	Leaves(Vec<Leaf>),
	MarkSurfaces(Vec<u16>),
	Edges(Vec<Edge>),
	SurfEdges(Vec<i32>),
	Models(Vec<Model>),
}

#[derive(Debug, Clone)]
pub struct Plane {
	pub normal: Vector3,
	pub dist: f32,
	pub plane_type: i32, // TODO: define the flags
}

#[derive(Debug, Clone)]
pub struct Texture {
	pub num_miptex: i32,
	// length seems to be hardcoded but could also be num_miptex
	pub data_offset: Vec<i32>,

	pub miptexs: Vec<Miptex>, // ill store it
}

#[derive(Debug, Clone)]
pub struct Miptex {
	pub name: String, // of length 16
	pub width: u32,
	pub height: u32,
	pub offsets: [u32; 4], // this one is definitely hardcoded
}

#[derive(Debug, Clone)]
pub struct Vertex {
	pub point: Vector3,
}

#[derive(Debug, Clone)]
pub struct Node {
	pub planenum: i32,
	pub children: [i16; 2], // negative numbers are -(leaf + 1) as usual
	pub mins: [i16; 3],
	pub maxs: [i16; 3],
	pub first_face: u16,
	pub num_faces: u16,
}

#[derive(Debug, Clone)]
pub struct TexInfo {
	pub vecs: [[f32; 2]; 4],
	pub miptex: i32,
	pub flags: i32, // TODO: define the one flag lmao
}

#[derive(Debug, Clone)]
pub struct Face {
	pub planenum: i16,
	pub side: i16,
	pub first_edge: i32,
	pub num_edges: i16,
	pub texinfo: i16,
	pub styles: [u8; 4], // TODO: define styles
	pub lightofs: i32,
}

#[derive(Debug, Clone)]
pub struct ClipNode {
	pub planenum: i32,
	pub children: [i16; 2],
}

#[derive(Debug, Clone)]
pub struct Leaf {
	pub contents: GoldSrcContentsFlags,
	pub visofs: i32, // -1 = no vis info
	pub mins: [u16; 3],
	pub maxs: [u16; 3],
	pub first_marksurface: u16,
	pub num_marksurfaces: u16,
	pub ambient_level: [u8; 4],
}

#[derive(Debug, Clone)]
pub struct Edge {
	// edge 0 is never used due to negative edge nums being used for
	// the counterclockwise use of edges in a face (see surfedge lump)
	pub v: [u16; 2], // vertex numbers
}

#[derive(Debug, Clone)]
pub struct Model {
	pub mins: Vector3,
	pub maxs: Vector3,
	pub origin: Vector3,
	pub headnode: [i32; 4],
	pub visleafs: i32, // not including the solid leaf 0
	pub firstface: i32,
	pub numfaces: i32,
}
