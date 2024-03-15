use std::collections::HashMap;
use crate::{
	file_structure::{BSPFile, Header, LumpInfo},
	lumps::{self, LumpType},
	reader::Reader,
	utils::Vector3,
};

pub fn parse_file(
	reader: &mut Reader,
) -> BSPFile {
	let mut file: BSPFile = BSPFile::new();
	parse_header(reader, &mut file.header);
	parse_data_lumps(reader, &file.header.lumps, &mut file.lump_data);
	file
}

pub fn parse_header(
	reader: &mut Reader,
	header: &mut Header,
) {
	header.ident = reader.read_int();

	// if ident isn't "VBSP"
	if header.ident != 0x50534256 {
		println!("invalid file header! exiting...");
		std::process::exit(0);
	}

	header.version = reader.read_int();

	// read lump info
	for i in 0..64 {
		header.lumps[i].file_offset = reader.read_uint();
		header.lumps[i].length = reader.read_uint();
		header.lumps[i].version = reader.read_uint();
		header.lumps[i].ident = reader.read_bytes(4).try_into().unwrap();
		header.lumps[i].index = i as u8;
	}

	header.map_revision = reader.read_int();

	println!("parsed header!");
}

pub fn parse_data_lumps(
	reader: &mut Reader,
	lump_info: &[LumpInfo; 64],
	lump_data: &mut Vec<LumpType>,
) {
	let mut current_index: usize = 0;
	let mut info: &LumpInfo = &lump_info[current_index];
	
	//      ====LUMP_ENTITIES====
	reader.index = info.file_offset as usize;
	let ent_string: String = reader.read_string();
	lump_data.push(LumpType::Entities(parse_entity_string(ent_string)));
	println!("parsed entities lump! ({current_index})");
	
	//      ====LUMP_PLANES====
	current_index += 1;
	info = &lump_info[current_index];
	reader.index = info.file_offset as usize;

	let mut planes: Vec<lumps::Plane> = vec![];
	while reader.index < (info.file_offset + info.length) as usize {
		planes.push(lumps::Plane {
			normal: reader.read_vector3(),
			dist: reader.read_float(),
			r#type: reader.read_int(),
		});
	}
	println!("parsed planes lump! ({current_index})");
	lump_data.push(LumpType::Planes(planes));

	//      ====LUMP_TEXDATA====
	current_index += 1;
	info = &lump_info[current_index];
	reader.index = info.file_offset as usize;

	let mut texdata: Vec<lumps::TexData> = vec![];
	while reader.index < (info.file_offset + info.length) as usize {
		texdata.push(lumps::TexData {
			reflectivity: reader.read_vector3(),
			name_string_table_id: reader.read_int(),
			width: reader.read_int(),
			height: reader.read_int(),
			view_width: reader.read_int(),
			view_height: reader.read_int(),
		});
	}
	println!("parsed texdata lump! ({current_index})");
	lump_data.push(LumpType::TexData(texdata));

	//      ====LUMP_VERTEXES====
	current_index += 1;
	info = &lump_info[current_index];
	reader.index = info.file_offset as usize;

	let mut vertexes: Vec<Vector3> = vec![];
	while reader.index < (info.file_offset + info.length) as usize {
		vertexes.push(reader.read_vector3());
	}
	println!("parsed vertexes lump! ({current_index})");
	lump_data.push(LumpType::Vertexes(vertexes));

	//      ====LUMP_VISIBILITY====
	current_index += 1;
	info = &lump_info[current_index];
	reader.index = info.file_offset as usize;

	let mut viss: Vec<lumps::Vis> = vec![];
	let num_clusters: i32 = reader.read_int();
	let mut byte_offsets: Vec<[i32; 2]> = vec![];
	for _ in 0..num_clusters {
		byte_offsets.push(
			[reader.read_int(), reader.read_int()]
		);
	}
	viss.push(lumps::Vis {
		num_clusters,
		byte_offsets,
	});
	println!("parsed visibility lump! ({current_index})");
	lump_data.push(LumpType::Visibility(viss));
	
	//      ====LUMP_NODES====
	current_index += 1;
	info = &lump_info[current_index];
	reader.index = info.file_offset as usize;

	let mut nodes: Vec<lumps::Node> = vec![];
	while reader.index < (info.file_offset + info.length) as usize {
		nodes.push(lumps::Node {
			plane_num: reader.read_int(),
			children: [reader.read_int(), reader.read_int()],
			mins: [reader.read_short(), reader.read_short(), reader.read_short()],
			maxs: [reader.read_short(), reader.read_short(), reader.read_short()],
			first_face: reader.read_ushort(),
			numfaces: reader.read_ushort(),
			area: reader.read_short(),
			padding: reader.read_short(),
		});
	}
	println!("parsed nodes lump! ({current_index})");
	lump_data.push(LumpType::Nodes(nodes));

	//      ====LUMP_TEXINFOS====
	current_index += 1;
	info = &lump_info[current_index];
	reader.index = info.file_offset as usize;

	let mut texinfos: Vec<lumps::TexInfo> = vec![];
	while reader.index < (info.file_offset + info.length) as usize {
		texinfos.push(lumps::TexInfo {
			texture_vecs: [
				[
					reader.read_float(), reader.read_float(),
					reader.read_float(), reader.read_float()
				],
				[
					reader.read_float(), reader.read_float(),
					reader.read_float(), reader.read_float()
				],
			],
			lightmap_vecs: [
				[
					reader.read_float(), reader.read_float(),
					reader.read_float(), reader.read_float()
				],
				[
					reader.read_float(), reader.read_float(),
					reader.read_float(), reader.read_float()
				],
			],
			flags: reader.read_int(),
			texdata: reader.read_int(),
		});
	}
	println!("parsed texinfo lump! ({current_index})");
	lump_data.push(LumpType::TexInfo(texinfos));

	//      ====LUMP_FACES====
	current_index += 1;
	info = &lump_info[current_index];
	reader.index = info.file_offset as usize;

	let mut faces: Vec<lumps::Face> = vec![];
	while reader.index < (info.file_offset + info.length) as usize {
		faces.push(lumps::Face {
			plane_num: reader.read_ushort(),
			side: reader.read_byte(),
			on_node: reader.read_byte(),
			first_edge: reader.read_uint(),
			num_edges: reader.read_short(),
			tex_info: reader.read_short(),
			disp_info: reader.read_short(),
			surface_fog_volume_id: reader.read_short(),
			styles: reader.read_int().to_le_bytes(), // lmao
			light_offset: reader.read_int(),
			area: reader.read_float(),
			lightmap_texture_mins: [reader.read_int(), reader.read_int()],
			lightmap_texture_size: [reader.read_int(), reader.read_int()],
			orig_face: reader.read_int(),
			num_prims: reader.read_ushort(),
			first_prim_id: reader.read_ushort(),
			smoothing_groups: reader.read_uint(),
		});
	}
	println!("parsed faces lump! ({current_index})");
	lump_data.push(LumpType::Faces(faces));
	
	//      ====LUMP_LIGHTING====
	current_index += 1;
	info = &lump_info[current_index];
	reader.index = info.file_offset as usize;

	let mut lightings: Vec<lumps::ColorRGBExp32> = vec![];
	while reader.index < (info.file_offset + info.length) as usize {
		lightings.push(lumps::ColorRGBExp32 {
			r: reader.read_byte(),
			g: reader.read_byte(),
			b: reader.read_byte(),
			exponent: reader.read_signed_byte(),
		});
	}
	println!("parsed lighting lump! ({current_index})");
	lump_data.push(LumpType::Lighting(lightings));
	
	//      ====LUMP_OCCLUSION====
	current_index += 1;
	info = &lump_info[current_index];
	reader.index = info.file_offset as usize;

	// this lump isnt an array :0
	let mut occluder: lumps::Occluder = lumps::Occluder {
		count: reader.read_int(),
		data: vec![],
		poly_data_count: 0,
		poly_data: vec![],
		vertex_index_count: 0,
		vertex_indices: vec![],
	};
	for _ in 0..occluder.count {
		occluder.data.push(lumps::OccluderData {
			flags: reader.read_int(),
			first_poly: reader.read_int(),
			poly_count: reader.read_int(),
			mins: reader.read_vector3(),
			maxs: reader.read_vector3(),
			area: reader.read_int(),
		})
	}
	occluder.poly_data_count = reader.read_int();
	for _ in 0..occluder.poly_data_count {
		occluder.poly_data.push(lumps::OccluderPolyData {
			first_vertex_index: reader.read_int(),
			vertex_count: reader.read_int(),
			plane_num: reader.read_int(),
		});
	}
	occluder.vertex_index_count = reader.read_int();
	for _ in 0..occluder.vertex_index_count {
		occluder.vertex_indices.push(reader.read_int());
	}

	println!("parsed occlusion lump! ({current_index})");
	lump_data.push(LumpType::Occlusion(occluder));

	//      ====LUMP_LEAFS====
	current_index += 1;
	info = &lump_info[current_index];
	reader.index = info.file_offset as usize;

	let mut leafs: Vec<lumps::Leaf> = vec![];
	while reader.index < (info.file_offset + info.length) as usize {
		leafs.push(lumps::Leaf {
			contents: reader.read_int(),
			cluster: reader.read_short(),
			area_flags: reader.read_short(),
			mins: [reader.read_short(), reader.read_short(), reader.read_short()],
			maxs: [reader.read_short(), reader.read_short(), reader.read_short()],
			first_leaf_face: reader.read_ushort(),
			num_leaf_faces: reader.read_ushort(),
			first_leaf_brushes: reader.read_ushort(),
			num_leaf_brushes: reader.read_ushort(),
			in_water: reader.read_short(),
			ambient_lighting:
				if info.version == 0 {
					Some(reader.read_compressed_light_cube())
				} else { None },
			padding: reader.read_short(),
		});
	}
	println!("parsed leafs lump! ({current_index})");
	lump_data.push(LumpType::Leafs(leafs));
	
	//      ====LUMP_FACEIDS====
	current_index += 1;
	info = &lump_info[current_index];
	reader.index = info.file_offset as usize;

	let mut faceids: Vec<lumps::FaceID> = vec![];
	while reader.index < (info.file_offset + info.length) as usize {
		faceids.push(lumps::FaceID { id: reader.read_ushort() });
	}
	println!("parsed faceids lump! ({current_index})");
	lump_data.push(LumpType::FaceIDs(faceids));

	//      ====LUMP_EDGES====
	current_index += 1;
	info = &lump_info[current_index];
	reader.index = info.file_offset as usize;

	let mut edges: Vec<lumps::Edge> = vec![];
	while reader.index < (info.file_offset + info.length) as usize {
		edges.push(lumps::Edge {
			pair: [reader.read_ushort(), reader.read_ushort()],
		});
	}
	println!("parsed edges lump! ({current_index})");
	lump_data.push(LumpType::Edges(edges));
	
	//      ====LUMP_SURFEDGES====
	current_index += 1;
	info = &lump_info[current_index];
	reader.index = info.file_offset as usize;

	let mut surfedges: Vec<i32> = vec![];
	while reader.index < (info.file_offset + info.length) as usize {
		surfedges.push(reader.read_int());
	}
	println!("parsed surfedges lump! ({current_index})");
	lump_data.push(LumpType::SurfEdges(surfedges));
	
	//      ====LUMP_MODELS====
	current_index += 1;
	info = &lump_info[current_index];
	reader.index = info.file_offset as usize;

	let mut models: Vec<lumps::Model> = vec![];
	while reader.index < (info.file_offset + info.length) as usize {
		models.push(lumps::Model {
			mins: reader.read_vector3(),
			maxs: reader.read_vector3(),
			origin: reader.read_vector3(),
			head_node: reader.read_int(),
			first_face: reader.read_int(),
			num_faces: reader.read_int(),
		});
	}
	println!("parsed models lump! ({current_index})");
	lump_data.push(LumpType::Models(models));

	// TODO: figure out the structure
	//      ====LUMP_WORLDLIGHTS====
	current_index += 1;
	info = &lump_info[current_index];
	reader.index = info.file_offset as usize;
	
	reader.skip(info.length as usize);
	lump_data.push(LumpType::None);
	println!("skipped worldlights lump! ({current_index})");
	
	// TODO: figure out the structure
	//      ====LUMP_LEAFFACES====
	current_index += 1;
	info = &lump_info[current_index];
	reader.index = info.file_offset as usize;
	
	reader.skip(info.length as usize);
	lump_data.push(LumpType::None);
	println!("skipped leaffaces lump! ({current_index})");
	
	// TODO: figure out the structure
	//      ====LUMP_LEAFBRUSHES====
	current_index += 1;
	info = &lump_info[current_index];
	reader.index = info.file_offset as usize;
	
	reader.skip(info.length as usize);
	lump_data.push(LumpType::None);
	println!("skipped leafbrushes lump! ({current_index})");
	
	//      ====LUMP_BRUSHES====
	current_index += 1;
	info = &lump_info[current_index];
	reader.index = info.file_offset as usize;

	let mut brushes: Vec<lumps::Brush> = vec![];
	while reader.index < (info.file_offset + info.length) as usize {
		brushes.push(lumps::Brush {
			first_side: reader.read_int(),
			num_sides: reader.read_int(), 
			contents: reader.read_int(),
		});
	}
	println!("parsed brushes lump! ({current_index})");
	lump_data.push(LumpType::Brushes(brushes));

	//      ====LUMP_BRUSHSIDES====
	current_index += 1;
	info = &lump_info[current_index];
	reader.index = info.file_offset as usize;

	let mut brushsides: Vec<lumps::BrushSide> = vec![];
	while reader.index < (info.file_offset + info.length) as usize {
		brushsides.push(lumps::BrushSide {
			plane_num: reader.read_ushort(),
			texinfo: reader.read_short(),
			dispinfo: reader.read_short(),
			bevel: reader.read_short(),
		});
	}
	println!("parsed brushsides lump! ({current_index})");
	lump_data.push(LumpType::BrushSides(brushsides));

	//      ====LUMP_AREAS====
	current_index += 1;
	info = &lump_info[current_index];
	reader.index = info.file_offset as usize;

	let mut areas: Vec<lumps::Area> = vec![];
	while reader.index < (info.file_offset + info.length) as usize {
		areas.push(lumps::Area {
			num_area_portals: reader.read_int(),
			first_area_portal: reader.read_int(),
		});
	}
	println!("parsed areas lump! ({current_index})");
	lump_data.push(LumpType::Areas(areas));

	//      ====LUMP_AREAPORTALS====
	current_index += 1;
	info = &lump_info[current_index];
	reader.index = info.file_offset as usize;

	let mut areaportals: Vec<lumps::AreaPortal> = vec![];
	while reader.index < (info.file_offset + info.length) as usize {
		areaportals.push(lumps::AreaPortal {
			portal_key: reader.read_ushort(),
			other_area: reader.read_ushort(),
			first_clip_portal_vert: reader.read_ushort(),
			clip_portal_verts: reader.read_ushort(),
			plane_num: reader.read_int(),
		});
	}
	println!("parsed areaportals lump! ({current_index})");
	lump_data.push(LumpType::AreaPortals(areaportals));

	// skip ones i havent done yet
	for i in current_index + 1..64 {
		reader.skip(lump_info[i].length as usize);
		lump_data.push(LumpType::None);
		println!("skipped lump with index {}!", i);
	}
}

// very very bad string parsing code
// it works so its fine
pub fn parse_entity_string(
	ent_string: String,
) -> Vec<HashMap<String, String>> {
	let mut entities: Vec<HashMap<String, String>> = vec![];
	let mut clean_strings: Vec<String> = vec![];

	for s in ent_string.split("}\n") {
		clean_strings.push(s.replace("{", ""));
	}

	let mut split_attrs: Vec<Vec<String>> = vec![];
	for s in clean_strings {
		split_attrs.push(
			s.split("\n")
			.map(|s| { s.to_owned() })
			.collect()
		);
	}

	for string in split_attrs {
		if string == vec![""] { continue; }
		let mut ent: HashMap<String, String> = HashMap::new();
		for attrs in string {
			if attrs == "" { continue; }
			let splitted: Vec<String> = attrs.split(" ")
			.map(|s| { s.trim_matches('\"').to_owned() })
			.collect();

			ent.insert(splitted[0].clone(), splitted[1].clone());
		}
		entities.push(ent);
	}

	entities
}