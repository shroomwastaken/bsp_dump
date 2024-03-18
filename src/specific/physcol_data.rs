use crate::utils::Vector3;

#[derive(Debug, Clone, Copy)]
pub struct CollideHeader {
	pub size: i32,
	pub id: i32, // usually b"VPHY"?
	pub version: u16, // usually 256?
	pub model_type: u16,
}

#[derive(Debug, Clone, Copy)]
pub struct MoppSurfaceHeader {
	pub size: i32,
}

#[derive(Debug, Clone, Copy)]
pub struct CompactSurfaceHeader {
	pub surface_size: i32,
	pub drag_axis_areas: Vector3,
	pub axis_map_size: i32,
}

#[derive(Debug, Clone)]
pub struct CollisionData {
	pub collide_header: CollideHeader,
	pub second_header: ModelHeaders,
	pub data: Vec<u8>, // just bytes
}

#[derive(Debug, Clone)]
pub enum ModelHeaders {
	None,
	CompactSurfaceHeader(CompactSurfaceHeader), // model type 0
	MoppSurfaceHeader(MoppSurfaceHeader), // model type 1
}