use std::time::Instant;
use crate::{
	file_structure::{BSPFile, BSPVersion, Header, LumpInfo}, 
	flags::{self, ContentsFlags, SurfaceFlags},
	lumps::{
		vbsp::{self, VBSPLumpType},
		goldsrc::{self, GoldSrcLumpType},
		lumptype::Lumps
	},
	reader::Reader,
	specific::{
		gamelump, occlusion, physcol_data::{self, ModelHeaders}, vis::decompress_vis
	},
	utils::{Vector3, parse_entity_string},
	VBSP_MAGIC, IBSP_MAGIC, GOLDSRC_MAGIC
};

pub fn parse_file(
	reader: &mut Reader,
) -> BSPFile {
	let start: Instant = Instant::now();
	let mut header: Header = Header::new();
	parse_header(reader, &mut header);
	let mut file: BSPFile = BSPFile::new(header);
	if let Lumps::VBSP(ld) = &mut file.lump_data {
		parse_vbsp_data_lumps(reader, &file.header.lumps, ld);
	} else if let Lumps::GoldSrc(ld) = &mut file.lump_data {
		parse_goldsrc_data_lumps(reader, &file.header.lumps, ld);
	}
	println!("\nparsed file in {:?}!\n", Instant::now().duration_since(start));
	file
}

pub fn parse_header(
	reader: &mut Reader,
	header: &mut Header,
) {
	header.ident = reader.read_int();

	if header.ident == VBSP_MAGIC {
		header.bspver = BSPVersion::VBSP;
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
	} else if header.ident == GOLDSRC_MAGIC {
		header.bspver = BSPVersion::GoldSrc;
		for i in 0..15 {
			header.lumps[i].file_offset = reader.read_uint();
			header.lumps[i].length = reader.read_uint();
			header.lumps[i].index = i as u8;
		}
	} else {
		println!("invalid file header! exiting...");
		std::process::exit(0);
	}

	println!("parsed header!");
}

pub fn parse_vbsp_data_lumps(
	reader: &mut Reader,
	lump_info: &[LumpInfo; 64],
	lump_data: &mut Vec<VBSPLumpType>,
) {
	let mut current_index: usize = 0;
	let mut info: &LumpInfo = &lump_info[current_index];

	//      ====LUMP_ENTITIES====
	reader.index = info.file_offset as usize;
	let ent_string: String = reader.read_string();
	lump_data.push(VBSPLumpType::Entities(parse_entity_string(ent_string)));
	println!("parsed entities lump! ({current_index})");

	//      ====LUMP_PLANES====
	current_index += 1;
	info = &lump_info[current_index];
	reader.index = info.file_offset as usize;

	let mut planes: Vec<vbsp::Plane> = vec![];
	while reader.index < (info.file_offset + info.length) as usize {
		planes.push(vbsp::Plane {
			normal: reader.read_vector3(),
			dist: reader.read_float(),
			r#type: reader.read_int(),
		});
	}
	println!("parsed planes lump! ({current_index})");
	lump_data.push(VBSPLumpType::Planes(planes));

	//      ====LUMP_TEXDATA====
	current_index += 1;
	info = &lump_info[current_index];
	reader.index = info.file_offset as usize;

	let mut texdata: Vec<vbsp::TexData> = vec![];
	while reader.index < (info.file_offset + info.length) as usize {
		texdata.push(vbsp::TexData {
			reflectivity: reader.read_vector3(),
			name_string_table_id: reader.read_int(),
			width: reader.read_int(),
			height: reader.read_int(),
			view_width: reader.read_int(),
			view_height: reader.read_int(),
		});
	}
	println!("parsed texdata lump! ({current_index})");
	lump_data.push(VBSPLumpType::TexData(texdata));

	//      ====LUMP_VERTEXES====
	current_index += 1;
	info = &lump_info[current_index];
	reader.index = info.file_offset as usize;

	let mut vertexes: Vec<Vector3> = vec![];
	while reader.index < (info.file_offset + info.length) as usize {
		vertexes.push(reader.read_vector3());
	}
	println!("parsed vertexes lump! ({current_index})");
	lump_data.push(VBSPLumpType::Vertexes(vertexes));

	//      ====LUMP_VISIBILITY====
	current_index += 1;
	info = &lump_info[current_index];
	reader.index = info.file_offset as usize;

	let mut vis: vbsp::Vis = vbsp::Vis {
		num_clusters: reader.read_int(),
		byte_offsets: vec![],
		cluster_data: [vec![], vec![]],
	};
	for _ in 0..vis.num_clusters {
		let pvs_ofs: i32 = reader.read_int();
		let pas_ofs: i32 = reader.read_int();
		vis.cluster_data[0].push(
			decompress_vis(
				&reader.bytes[info.file_offset as usize + pvs_ofs as usize..],
				&vis.num_clusters,
			)
		);
		vis.cluster_data[1].push(
			decompress_vis(
				&reader.bytes[info.file_offset as usize + pas_ofs as usize..],
				&vis.num_clusters,
			)
		);
		vis.byte_offsets.push(
			[pvs_ofs, pas_ofs]
		);
	}
	println!("parsed and decompressed visibility lump! ({current_index})");
	lump_data.push(VBSPLumpType::Visibility(vis));

	//      ====LUMP_NODES====
	current_index += 1;
	info = &lump_info[current_index];
	reader.index = info.file_offset as usize;

	let mut nodes: Vec<vbsp::Node> = vec![];
	while reader.index < (info.file_offset + info.length) as usize {
		nodes.push(vbsp::Node {
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
	lump_data.push(VBSPLumpType::Nodes(nodes));

	//      ====LUMP_TEXINFOS====
	current_index += 1;
	info = &lump_info[current_index];
	reader.index = info.file_offset as usize;

	let mut texinfos: Vec<vbsp::TexInfo> = vec![];
	while reader.index < (info.file_offset + info.length) as usize {
		texinfos.push(vbsp::TexInfo {
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
	lump_data.push(VBSPLumpType::TexInfo(texinfos));

	//      ====LUMP_FACES====
	current_index += 1;
	info = &lump_info[current_index];
	reader.index = info.file_offset as usize;

	let mut faces: Vec<vbsp::Face> = vec![];
	while reader.index < (info.file_offset + info.length) as usize {
		faces.push(vbsp::Face {
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
	lump_data.push(VBSPLumpType::Faces(faces));

	//      ====LUMP_LIGHTING====
	current_index += 1;
	info = &lump_info[current_index];
	reader.index = info.file_offset as usize;

	let mut lightings: Vec<vbsp::ColorRGBExp32> = vec![];
	while reader.index < (info.file_offset + info.length) as usize {
		lightings.push(reader.read_colorrgbexp32());
	}
	println!("parsed lighting lump! ({current_index})");
	lump_data.push(VBSPLumpType::Lighting(lightings));

	//      ====LUMP_OCCLUSION====
	current_index += 1;
	info = &lump_info[current_index];
	reader.index = info.file_offset as usize;

	// this lump isnt an array :0
	let mut occluder: vbsp::Occluder = vbsp::Occluder {
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
	lump_data.push(VBSPLumpType::Occlusion(occluder));

	//      ====LUMP_LEAFS====
	current_index += 1;
	info = &lump_info[current_index];
	reader.index = info.file_offset as usize;

	let mut leafs: Vec<vbsp::Leaf> = vec![];
	while reader.index < (info.file_offset + info.length) as usize {
		leafs.push(vbsp::Leaf {
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
	lump_data.push(VBSPLumpType::Leafs(leafs));

	//      ====LUMP_FACEIDS====
	current_index += 1;
	info = &lump_info[current_index];
	reader.index = info.file_offset as usize;

	let mut faceids: Vec<vbsp::FaceID> = vec![];
	while reader.index < (info.file_offset + info.length) as usize {
		faceids.push(vbsp::FaceID { id: reader.read_ushort() });
	}
	println!("parsed faceids lump! ({current_index})");
	lump_data.push(VBSPLumpType::FaceIDs(faceids));

	//      ====LUMP_EDGES====
	current_index += 1;
	info = &lump_info[current_index];
	reader.index = info.file_offset as usize;

	let mut edges: Vec<vbsp::Edge> = vec![];
	while reader.index < (info.file_offset + info.length) as usize {
		edges.push(vbsp::Edge {
			pair: [reader.read_ushort(), reader.read_ushort()],
		});
	}
	println!("parsed edges lump! ({current_index})");
	lump_data.push(VBSPLumpType::Edges(edges));

	//      ====LUMP_SURFEDGES====
	current_index += 1;
	info = &lump_info[current_index];
	reader.index = info.file_offset as usize;

	let mut surfedges: Vec<i32> = vec![];
	while reader.index < (info.file_offset + info.length) as usize {
		surfedges.push(reader.read_int());
	}
	println!("parsed surfedges lump! ({current_index})");
	lump_data.push(VBSPLumpType::SurfEdges(surfedges));

	//      ====LUMP_MODELS====
	current_index += 1;
	info = &lump_info[current_index];
	reader.index = info.file_offset as usize;

	let mut models: Vec<vbsp::Model> = vec![];
	while reader.index < (info.file_offset + info.length) as usize {
		models.push(vbsp::Model {
			mins: reader.read_vector3(),
			maxs: reader.read_vector3(),
			origin: reader.read_vector3(),
			head_node: reader.read_int(),
			first_face: reader.read_int(),
			num_faces: reader.read_int(),
		});
	}
	println!("parsed models lump! ({current_index})");
	lump_data.push(VBSPLumpType::Models(models));

	// TODO: figure out the structure
	//      ====LUMP_WORLDLIGHTS====
	current_index += 1;
	info = &lump_info[current_index];
	reader.index = info.file_offset as usize;

	reader.skip(info.length as usize);
	lump_data.push(VBSPLumpType::None);
	println!("skipped worldlights lump! ({current_index})");

	// TODO: figure out the structure
	//      ====LUMP_LEAFFACES====
	current_index += 1;
	info = &lump_info[current_index];
	reader.index = info.file_offset as usize;

	reader.skip(info.length as usize);
	lump_data.push(VBSPLumpType::None);
	println!("skipped leaffaces lump! ({current_index})");

	// TODO: figure out the structure
	//      ====LUMP_LEAFBRUSHES====
	current_index += 1;
	info = &lump_info[current_index];
	reader.index = info.file_offset as usize;

	reader.skip(info.length as usize);
	lump_data.push(VBSPLumpType::None);
	println!("skipped leafbrushes lump! ({current_index})");

	//      ====LUMP_BRUSHES====
	current_index += 1;
	info = &lump_info[current_index];
	reader.index = info.file_offset as usize;

	let mut brushes: Vec<vbsp::Brush> = vec![];
	while reader.index < (info.file_offset + info.length) as usize {
		brushes.push(vbsp::Brush {
			first_side: reader.read_int(),
			num_sides: reader.read_int(),
			contents: ContentsFlags::from_bits_truncate(reader.read_uint()),
		});
	}
	println!("parsed brushes lump! ({current_index})");
	lump_data.push(VBSPLumpType::Brushes(brushes));

	//      ====LUMP_BRUSHSIDES====
	current_index += 1;
	info = &lump_info[current_index];
	reader.index = info.file_offset as usize;

	let mut brushsides: Vec<vbsp::BrushSide> = vec![];
	while reader.index < (info.file_offset + info.length) as usize {
		brushsides.push(vbsp::BrushSide {
			plane_num: reader.read_ushort(),
			texinfo: reader.read_short(),
			dispinfo: reader.read_short(),
			bevel: reader.read_short(),
		});
	}
	println!("parsed brushsides lump! ({current_index})");
	lump_data.push(VBSPLumpType::BrushSides(brushsides));

	//      ====LUMP_AREAS====
	current_index += 1;
	info = &lump_info[current_index];
	reader.index = info.file_offset as usize;

	let mut areas: Vec<vbsp::Area> = vec![];
	while reader.index < (info.file_offset + info.length) as usize {
		areas.push(vbsp::Area {
			num_area_portals: reader.read_int(),
			first_area_portal: reader.read_int(),
		});
	}
	println!("parsed areas lump! ({current_index})");
	lump_data.push(VBSPLumpType::Areas(areas));

	//      ====LUMP_AREAPORTALS====
	current_index += 1;
	info = &lump_info[current_index];
	reader.index = info.file_offset as usize;

	let mut areaportals: Vec<vbsp::AreaPortal> = vec![];
	while reader.index < (info.file_offset + info.length) as usize {
		areaportals.push(vbsp::AreaPortal {
			portal_key: reader.read_ushort(),
			other_area: reader.read_ushort(),
			first_clip_portal_vert: reader.read_ushort(),
			clip_portal_verts: reader.read_ushort(),
			plane_num: reader.read_int(),
		});
	}
	println!("parsed areaportals lump! ({current_index})");
	lump_data.push(VBSPLumpType::AreaPortals(areaportals));

	// TODO: the next four lumps are unsused in source 2007/2009
	// i need to detect source version somehow but for now im just
	// trying to parse portal bsp so ill just skip these 4

	//      ====LUMP_UNUSED22/LUMP_UNUSED23/LUMP_UNUSED24/LUMP_UNUSED25====
	lump_data.push(VBSPLumpType::Unused22);
	lump_data.push(VBSPLumpType::Unused23);
	lump_data.push(VBSPLumpType::Unused24);
	lump_data.push(VBSPLumpType::Unused25);
	current_index += 4;
	println!("skipped lumps 22-25, they are unused");

	//      ====LUMP_DISPINFO====
	current_index += 1;
	info = &lump_info[current_index];
	reader.index = info.file_offset as usize;
	let mut dispinfos: Vec<vbsp::DispInfo> = vec![];
	while reader.index < (info.file_offset + info.length) as usize {
		dispinfos.push(vbsp::DispInfo {
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
	lump_data.push(VBSPLumpType::DispInfo(dispinfos));

	//      ====LUMP_ORIGINALFACES====
	current_index += 1;
	info = &lump_info[current_index];
	reader.index = info.file_offset as usize;

	// literally the same exact structure as the faces lump
	let mut orig_faces: Vec<vbsp::Face> = vec![];
	while reader.index < (info.file_offset + info.length) as usize {
		orig_faces.push(vbsp::Face {
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
	lump_data.push(VBSPLumpType::OriginalFaces(orig_faces));

	//      ====LUMP_PHYDISP====
	current_index += 1;
	info = &lump_info[current_index];
	reader.index = info.file_offset as usize;

	let mut phydisps: Vec<vbsp::PhyDisp> = vec![];
	while reader.index < (info.file_offset + info.length) as usize {
		phydisps.push(vbsp::PhyDisp {
			num_disps: reader.read_ushort(),
		});
	}
	lump_data.push(VBSPLumpType::PhyDisp(phydisps));
	println!("parsed phydisp lump! ({current_index})");

	//      ====LUMP_PHYSCOLLIDE====
	current_index += 1;
	info = &lump_info[current_index];
	reader.index = info.file_offset as usize;

	let mut physmodels: Vec<vbsp::PhysModel> = vec![];
	while reader.index < (info.file_offset + info.length) as usize {
		let mut model: vbsp::PhysModel = vbsp::PhysModel {
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
	lump_data.push(VBSPLumpType::PhysCollide(physmodels));
	println!("parsed physcollide lump! ({current_index})");

	//      ====LUMP_VERTNORMALS====
	current_index += 1;
	info = &lump_info[current_index];
	reader.index = info.file_offset as usize;

	let mut vertnormals: Vec<vbsp::VertexNormal> = vec![];
	while reader.index < (info.file_offset + info.length) as usize {
		vertnormals.push(vbsp::VertexNormal {
			normal: reader.read_vector3()
		});
	}
	lump_data.push(VBSPLumpType::VertNormal(vertnormals));
	println!("parsed vertnormals lump! ({current_index})");

	//      ====LUMP_VERTNORMALINDICES====
	current_index += 1;
	info = &lump_info[current_index];
	reader.index = info.file_offset as usize;

	let mut vertnormalindices: Vec<vbsp::VertexNormalIndex> = vec![];
	while reader.index < (info.file_offset + info.length) as usize {
		vertnormalindices.push(vbsp::VertexNormalIndex {
			index: reader.read_ushort(),
		});
	}
	lump_data.push(VBSPLumpType::VertNormalIndices(vertnormalindices));
	println!("parsed vertnormals lump! ({current_index})");

	//      ====LUMP_DISPLIGHTMAPALPHAS====
	current_index += 1;
	info = &lump_info[current_index];
	reader.index = info.file_offset as usize;

	// the structure for this one is unknown
	lump_data.push(VBSPLumpType::None);
	println!("skipped displightmapalphas lump! ({current_index})");

	//      ====LUMP_DISPVERTS====
	current_index += 1;
	info = &lump_info[current_index];
	reader.index = info.file_offset as usize;

	let mut dispverts: Vec<vbsp::DispVert> = vec![];
	while reader.index < (info.file_offset + info.length) as usize {
		dispverts.push(vbsp::DispVert {
			vec: reader.read_vector3(),
			dist: reader.read_float(),
			alpha: reader.read_float(),
		});
	}
	lump_data.push(VBSPLumpType::DispVerts(dispverts));
	println!("parsed dispverts lump! ({current_index})");

	//      ====LUMP_DISP_LIGHTMAP_SAMPLE_POSITIONS====
	current_index += 1;
	info = &lump_info[current_index];
	reader.index = info.file_offset as usize;

	let mut dlsp: Vec<vbsp::DispLightmapSamplePosition> = vec![];
	while reader.index < (info.file_offset + info.length) as usize {
		dlsp.push(vbsp::DispLightmapSamplePosition { unknown: reader.read_byte() });
	}
	lump_data.push(VBSPLumpType::DispLightmapSamplePositions(dlsp));
	println!("parsed displightmapsamplepositions lump! ({current_index})");

	//      ====LUMP_GAME_LUMP====
	// the most annoying lump of them all (probably)
	// for now ill only read the headers
	current_index += 1;
	info = &lump_info[current_index];
	reader.index = info.file_offset as usize;

	let mut gamelump: vbsp::GameLump = vbsp::GameLump {
		header: gamelump::GameLumpHeader {
			lump_count: reader.read_int(),
			game_lump_info: vec![],
		},
		data: vec![],
	};
	for _ in 0..gamelump.header.lump_count {
		gamelump.header.game_lump_info.push(gamelump::GameLumpInfo {
			id: reader.read_int(),
			flags: reader.read_ushort(),
			version: reader.read_ushort(),
			file_offset: reader.read_int(),
			file_length: reader.read_int(),
		});
	}
	// ill only read the two static props headers for now
	for g_lump_info in &gamelump.header.game_lump_info {
		reader.index = g_lump_info.file_offset as usize;
		if &g_lump_info.id.to_be_bytes() != b"sprp" { continue }
		let mut dict: gamelump::StaticPropDictLump = gamelump::StaticPropDictLump {
			dict_entries: reader.read_int(),
			names: vec![],
		};
		for _ in 0..dict.dict_entries {
			// uhhhh the way im reading strings at the moment is confusing
			// ill just remove the null byte manually
			// all of these names are null-padded to 128 bytes for some reason
			let mut name: String = reader.read_string();
			reader.index += 128 - name.len();
			name = name.as_str()[..name.len() - 1].to_string();
			dict.names.push(name);
		}
		let mut leafs: gamelump::StaticPropLeafLump = gamelump::StaticPropLeafLump {
			leaf_entries: reader.read_int(),
			leafs: vec![],
		};
		for _ in 0..leafs.leaf_entries { leafs.leafs.push(reader.read_ushort()) }
		gamelump.data.push(gamelump::GameLumpData::StaticProps(
			gamelump::StaticProps {
				dict,
				leafs,
				num_entries: reader.read_int(),
			}
		));
	}
	lump_data.push(VBSPLumpType::GameLump(gamelump));
	println!("parsed gamelump headers! ({current_index})");
	// TODO: sprp, dprp and all the other fun stuff

	//      ====LUMP_LEAFWATERDATA====
	current_index += 1;
	lump_data.push(VBSPLumpType::None);
	println!("skipped leafwaterdata lump! ({current_index})");

	//      ====LUMP_PRIMITIVES====
	current_index += 1;
	info = &lump_info[current_index];
	reader.index = info.file_offset as usize;

	let mut prims: Vec<vbsp::Primitive> = vec![];

	while reader.index < (info.file_offset + info.length) as usize {
		prims.push(vbsp::Primitive {
			// TODO: check an hl2 map
			// cause it could be a u8 apparently
			r#type: reader.read_ushort(),
			first_index: reader.read_ushort(),
			num_indices: reader.read_ushort(),
			first_vertex: reader.read_ushort(),
			num_vertices: reader.read_ushort(),
		});
	}
	lump_data.push(VBSPLumpType::Primitives(prims));
	println!("parsed primitives lump! ({current_index})");

	//      ====LUMP_PRIMVERTS====
	current_index += 1;
	info = &lump_info[current_index];
	reader.index = info.file_offset as usize;

	let mut primverts: Vec<vbsp::PrimVert> = vec![];
	while reader.index < (info.file_offset + info.length) as usize {
		primverts.push(vbsp::PrimVert { pos: reader.read_vector3() });
	}
	lump_data.push(VBSPLumpType::PrimVerts(primverts));
	println!("parsed primverts lump! ({current_index})");

	//      ====LUMP_PRIMINDICES====
	current_index += 1;
	info = &lump_info[current_index];
	reader.index = info.file_offset as usize;

	let mut primindices: Vec<vbsp::PrimIndex> = vec![];
	while reader.index < (info.file_offset + info.length) as usize {
		primindices.push(vbsp::PrimIndex { index: reader.read_ushort() })
	}
	lump_data.push(VBSPLumpType::PrimIndices(primindices));
	println!("parsed primindices lump! ({current_index})");

	//      ====LUMP_PAKFILE====
	current_index += 1;
	info = &lump_info[current_index];
	reader.index = info.file_offset as usize;

	let pakfile: vbsp::PakFile = vbsp::PakFile {
		bytes: reader.read_bytes(info.length as usize),
	};
	lump_data.push(VBSPLumpType::PakFile(pakfile));
	println!("parsed pakfile lump! ({current_index})");

	//      ====LUMP_CLIPPORTALVERTS====
	current_index += 1;
	info = &lump_info[current_index];
	reader.index = info.file_offset as usize;
	
	let mut clip_portal_verts: Vec<vbsp::ClipPortalVert> = vec![];
	while reader.index < (info.file_offset + info.length) as usize {
		clip_portal_verts.push(vbsp::ClipPortalVert { vec: reader.read_vector3() });
	}
	lump_data.push(VBSPLumpType::ClipPortalVerts(clip_portal_verts));
	println!("parsed clipportalverts lump! ({current_index})");

	//      ====LUMP_CUBEMAPS====
	current_index += 1;
	info = &lump_info[current_index];
	reader.index = info.file_offset as usize;

	let mut cubemaps: Vec<vbsp::CubemapSample> = vec![];
	while reader.index < (info.file_offset + info.length) as usize {
		cubemaps.push(vbsp::CubemapSample {
			origin: [reader.read_int(), reader.read_int(), reader.read_int()],
			size: reader.read_int(),
		});
	}
	lump_data.push(VBSPLumpType::Cubemaps(cubemaps));
	println!("parsed cubemaps lump! ({current_index})");

	//      ====LUMP_TEXDATASTRINGDATA====
	current_index += 1;
	info = &lump_info[current_index];
	reader.index = info.file_offset as usize;
	
	let mut texdatastringdata: Vec<vbsp::TexDataStringData> = vec![];
	while reader.index < (info.file_offset + info.length) as usize {
		texdatastringdata.push(vbsp::TexDataStringData {
			offset: reader.index - info.file_offset as usize,
			val: reader.read_string(),
		})
	}
	lump_data.push(VBSPLumpType::TexDataStringData(texdatastringdata));
	println!("parsed texdatastringdata lump! ({current_index})");

	//      ====LUMP_TEXDATASTRINGTABLE====
	current_index += 1;
	info = &lump_info[current_index];
	reader.index = info.file_offset as usize;

	let mut texdatastringtable: Vec<vbsp::TexDataStringTable> = vec![];
	while reader.index < (info.file_offset + info.length) as usize {
		texdatastringtable.push(vbsp::TexDataStringTable { offset: reader.read_uint() })
	}
	lump_data.push(VBSPLumpType::TexDataStringTable(texdatastringtable));
	println!("parsed texdatastringtable lump! ({current_index})");

	//      ====LUMP_OVERLAYS====
	current_index += 1;
	info = &lump_info[current_index];
	reader.index = info.file_offset as usize;

	let mut overlays: Vec<vbsp::Overlay> = vec![];
	while reader.index < (info.file_offset + info.length) as usize {
		let mut overlay: vbsp::Overlay = vbsp::Overlay {
			id: reader.read_int(),
			texinfo: reader.read_short(),
			face_count_and_render_order: reader.read_ushort(),
			faces: [0; 64],
			u: [0.0; 2],
			v: [0.0; 2],
			uv_points: [Vector3::new(); 4],
			origin: Vector3::new(),
			basis_normal: Vector3::new(),
		};
		// i should make a method for reading slices
		for i in 0..64 { overlay.faces[i] = reader.read_int(); }
		overlay.u = [reader.read_float(), reader.read_float()];
		overlay.v = [reader.read_float(), reader.read_float()];
		overlay.uv_points = [
			reader.read_vector3(), reader.read_vector3(),
			reader.read_vector3(), reader.read_vector3(),
		];
		overlay.origin = reader.read_vector3();
		overlay.basis_normal = reader.read_vector3();
		overlays.push(overlay)
	}
	lump_data.push(VBSPLumpType::Overlays(overlays));
	println!("parsed overlays lump! ({current_index})");

	//      ====LUMP_LEAFMINDISTTOWATER====
	current_index += 1;
	info = &lump_info[current_index];
	reader.index = info.file_offset as usize;

	let mut dists: Vec<vbsp::LeafMinDistToWater> = vec![];
	while reader.index < (info.file_offset + info.length) as usize {
		dists.push(vbsp::LeafMinDistToWater { dist: reader.read_int(), });
	}
	lump_data.push(VBSPLumpType::LeafMinDistToWater(dists));
	println!("parsed leafmindisttowater lump! ({current_index})");

	//      ====LUMP_FACEMACROTEXTUREINFO====
	current_index += 1;
	info = &lump_info[current_index];
	reader.index = info.file_offset as usize;

	let mut inds: Vec<vbsp::FaceMacroTextureInfo> = vec![];
	while reader.index < (info.file_offset + info.length) as usize {
		inds.push(vbsp::FaceMacroTextureInfo { index: reader.read_int(), });
	}
	lump_data.push(VBSPLumpType::FaceMacroTextureInfo(inds));
	println!("parsed facemacrotextureinfo lump! ({current_index})");

	//      ====LUMP_DISPTRIS====
	current_index += 1;
	info = &lump_info[current_index];
	reader.index = info.file_offset as usize;

	let mut tris: Vec<flags::DispTriFlags> = vec![];
	while reader.index < (info.file_offset + info.length) as usize {
		tris.push(flags::DispTriFlags::from_bits_truncate(reader.read_ushort()));
	}
	lump_data.push(VBSPLumpType::DispTris(tris));
	println!("parsed disptris lump! ({current_index})");

	//      ====LUMP_PHYSCOLLISESURFACE====
	println!("skipped physcollide lump! ({current_index})");

	// skip ones i havent done yet
	for i in current_index + 1..64 {
		lump_data.push(VBSPLumpType::None);
		println!("skipped lump with index {}!", i);
	}
}

pub fn parse_goldsrc_data_lumps(
	reader: &mut Reader,
	lump_info: &[LumpInfo; 64],
	lump_data: &mut Vec<GoldSrcLumpType>,
) {

}
