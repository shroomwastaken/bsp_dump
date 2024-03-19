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

// more ugly string parsing code :))))
pub fn parse_keydata_string(
	keydata: String,
) -> Vec<(String, Vec<(String, String)>)> {
	let objs: Vec<(&str, &str)> = keydata.split("\n}\n")
	.map(|s| {
		if s != "\0" {
			s.split_once(" {\n")
			.unwrap()
		} else {
			("", "")
		} 
	})
	.collect();

	let mut res: Vec<(String, Vec<(String, String)>)> = vec![]; 
	for obj in objs {
		if obj == ("", "") { continue }
		// this is one of the pieces of code of all time
		let attrs: Vec<(String, String)> = obj.1.split("\n")
		.map(|s| {
			s.split(" ")
			.map(|s| {
				s.trim_matches('\"')
				.to_owned()
			})
			.collect::<Vec<String>>()
		})
		.map(|vec| {
			(vec[0].clone(), vec[1].clone())
		})
		.collect();
	
		res.push((obj.0.to_owned(), attrs));
	}

	res
}