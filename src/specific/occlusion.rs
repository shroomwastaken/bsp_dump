use crate::utils::Vector3;

#[derive(Debug, Clone, Copy)]
pub struct OccluderData {
	pub flags: i32,
	pub first_poly: i32,
	pub poly_count: i32,
	pub mins: Vector3,
	pub maxs: Vector3,
	pub area: i32,
}

#[derive(Debug, Clone, Copy)]
pub struct OccluderPolyData {
	pub first_vertex_index: i32,
	pub vertex_count: i32,
	pub plane_num: i32,
}
