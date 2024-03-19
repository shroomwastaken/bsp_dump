use std::time::Instant;
use crate::{
	file_structure::{BSPFile, Header, LumpInfo},
	lumps::{self, LumpType},
	reader::Reader,
	utils::Vector3,
	specific::{
		occlusion,
		physcol_data::{self, ModelHeaders},
	},
	flags::{
		ContentsFlags,
		SurfaceFlags,
	}
};

pub fn parse_file(
	reader: &mut Reader,
) -> BSPFile {
	let mut file: BSPFile = BSPFile::new();
	let start: Instant = Instant::now();
	parse_header(reader, &mut file.header);
	parse_data_lumps(reader, &file.header.lumps, &mut file.lump_data);
	println!("\nparsed file in {:?}!\n", Instant::now().duration_since(start));
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

	let mut vis: lumps::Vis = lumps::Vis {
		num_clusters: reader.read_int(),
		byte_offsets: vec![],
	};
	for _ in 0..vis.num_clusters {
		vis.byte_offsets.push(
			[reader.read_int(), reader.read_int()]
		);
	}
	println!("parsed visibility lump! ({current_index})");
	lump_data.push(LumpType::Visibility(vis));

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
			flags: SurfaceFlags::from_bits_truncate(reader.read_uint()),
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
		lightings.push(reader.read_colorrgbexp32());
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
		occluder.data.push(occlusion::OccluderData {
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
		occluder.poly_data.push(occlusion::OccluderPolyData {
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
			contents: ContentsFlags::from_bits_truncate(reader.read_uint()),
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
			contents: ContentsFlags::from_bits_truncate(reader.read_uint()),
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

	// TODO: the next four lumps are unsused in source 2007/2009
	// i need to detect source version somehow but for now im just
	// trying to parse portal bsp so ill just skip these 4

	//      ====LUMP_UNUSED22/LUMP_UNUSED23/LUMP_UNUSED24/LUMP_UNUSED25====
	lump_data.push(LumpType::Unused22);
	lump_data.push(LumpType::Unused23);
	lump_data.push(LumpType::Unused24);
	lump_data.push(LumpType::Unused25);
	current_index += 4;
	println!("skipped lumps 22-25, they are unused");

	//      ====LUMP_DISPINFO====
	current_index += 1;
	info = &lump_info[current_index];
	reader.index = info.file_offset as usize;
	let mut dispinfos: Vec<lumps::DispInfo> = vec![];
	while reader.index < (info.file_offset + info.length) as usize {
		dispinfos.push(lumps::DispInfo {
			start_position: reader.read_vector3(),
			disp_vert_start: reader.read_int(),
			disp_tri_start: reader.read_int(),
			power: reader.read_int(),
			min_tess: reader.read_int(),
			smoothing_angle: reader.read_float(),
			contents: ContentsFlags::from_bits_truncate(reader.read_uint()),
			map_face: reader.read_ushort(),
			lightmap_alpha_start: reader.read_int(),
			lightmap_sample_position_start: reader.read_int(),
			edge_neighbors: [
				reader.read_cdispneighbor(), reader.read_cdispneighbor(),
				reader.read_cdispneighbor(), reader.read_cdispneighbor(),
			],
			corner_neighbors: [
				reader.read_cdispcornerneighbor(), reader.read_cdispcornerneighbor(),
				reader.read_cdispcornerneighbor(), reader.read_cdispcornerneighbor(),
			],
			allowed_verts: [
				reader.read_uint(), reader.read_uint(), reader.read_uint(),
				reader.read_uint(), reader.read_uint(), reader.read_uint(),
				reader.read_uint(), reader.read_uint(), reader.read_uint(),
				reader.read_uint(),
			],
		});
	}
	println!("parsed dispinfo lump! ({current_index})");
	lump_data.push(LumpType::DispInfo(dispinfos));

	//      ====LUMP_ORIGINALFACES====
	current_index += 1;
	info = &lump_info[current_index];
	reader.index = info.file_offset as usize;

	// literally the same exact structure as the faces lump
	let mut orig_faces: Vec<lumps::Face> = vec![];
	while reader.index < (info.file_offset + info.length) as usize {
		orig_faces.push(lumps::Face {
			plane_num: reader.read_ushort(),
			side: reader.read_byte(),
			on_node: reader.read_byte(),
			first_edge: reader.read_uint(),
			num_edges: reader.read_short(),
			tex_info: reader.read_short(),
			disp_info: reader.read_short(),
			surface_fog_volume_id: reader.read_short(),
			styles: reader.read_int().to_le_bytes(),
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
	lump_data.push(LumpType::OriginalFaces(orig_faces));

	//      ====LUMP_PHYDISP====
	current_index += 1;
	info = &lump_info[current_index];
	reader.index = info.file_offset as usize;

	let mut phydisps: Vec<lumps::PhyDisp> = vec![];
	while reader.index < (info.file_offset + info.length) as usize {
		phydisps.push(lumps::PhyDisp {
			num_disps: reader.read_ushort(),
		});
	}
	lump_data.push(LumpType::PhyDisp(phydisps));
	println!("parsed phydisp lump! ({current_index})");

	//      ====LUMP_PHYSCOLLIDE====
	current_index += 1;
	info = &lump_info[current_index];
	reader.index = info.file_offset as usize;

	let mut physmodels: Vec<lumps::PhysModel> = vec![];
	while reader.index < (info.file_offset + info.length) as usize {
		let mut model: lumps::PhysModel = lumps::PhysModel {
			model_index: reader.read_int(),
			data_size: reader.read_int(),
			keydata_size: reader.read_int(),
			solid_count: reader.read_int(),
			collision_data: vec![],
			key_data: vec![],
		};
		if model.model_index == -1 {
			physmodels.push(model);
			break;
		}

		for _ in 0..model.solid_count {
			let mut coll_data: physcol_data::CollisionData = physcol_data::CollisionData {
				collide_header: physcol_data::CollideHeader {
					size: reader.read_int(),
					id: reader.read_int(),
					version: reader.read_ushort(),
					model_type: reader.read_ushort(),
				},
				second_header: ModelHeaders::None,
				data: vec![],
			};
			if coll_data.collide_header.model_type == 0 {
				let surface_size: i32 = reader.read_int();
				coll_data.second_header = ModelHeaders::CompactSurfaceHeader(
					physcol_data::CompactSurfaceHeader {
						surface_size: surface_size.clone(),
						drag_axis_areas: reader.read_vector3(),
						axis_map_size: reader.read_int(),
					}
				);
				coll_data.data = reader.read_bytes(surface_size as usize);
			} else {
				// it seems theres only model types 1 and 0
				let size: i32 = reader.read_int();
				coll_data.second_header = ModelHeaders::MoppSurfaceHeader(
					physcol_data::MoppSurfaceHeader {
						size: size.clone(),
					}
				);
				coll_data.data = reader.read_bytes(size as usize);
			}
			model.collision_data.push(coll_data);
		}
		model.key_data = physcol_data::parse_keydata_string(reader.read_string());
		physmodels.push(model);
	}
	lump_data.push(LumpType::PhysCollide(physmodels));
	println!("parsed physcollide lump! ({current_index})");

	//      ====LUMP_VERTNORMALS====
	current_index += 1;
	info = &lump_info[current_index];
	reader.index = info.file_offset as usize;

	let mut vertnormals: Vec<lumps::VertexNormal> = vec![];
	while reader.index < (info.file_offset + info.length) as usize {
		vertnormals.push(lumps::VertexNormal {
			normal: reader.read_vector3()
		});
	}
	lump_data.push(LumpType::VertNormal(vertnormals));
	println!("parsed vertnormals lump! {current_index}");

	//      ====LUMP_VERTNORMALINDICES====
	current_index += 1;
	info = &lump_info[current_index];
	reader.index = info.file_offset as usize;

	let mut vertnormalindices: Vec<lumps::VertexNormalIndex> = vec![];
	while reader.index < (info.file_offset + info.length) as usize {
		vertnormalindices.push(lumps::VertexNormalIndex {
			index: reader.read_ushort(),
		});
	}
	lump_data.push(LumpType::VertNormalIndices(vertnormalindices));
	println!("parsed vertnormals lump! {current_index}");

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
) -> Vec<Vec<(String, String)>> {
	let mut entities: Vec<Vec<(String, String)>> = vec![];
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
		let mut ent: Vec<(String, String)> = vec![];
		for attrs in string {
			if attrs == "" || attrs == "\0" { continue; }
			let splitted: Vec<String> = attrs.split(" ")
			.map(|s| { s.trim_matches('\"').to_owned() })
			.collect();

			ent.push((splitted[0].clone(), splitted[1].clone()));
		}
		entities.push(ent);
	}

	entities
}
