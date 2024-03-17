use std::{
	fs,
	io::Write,
};
use crate::{
	file_structure,
	lumps::{self, LumpType},
	VERSION
};

pub fn dump_file(
	path: String,
	file: file_structure::BSPFile,
) {
	let dump_file_path: String = path.trim_end_matches(".bsp")
	.to_owned() + "-bsp_dump.txt";
	let mut dump_file: fs::File = fs::File::create(dump_file_path)
	.unwrap();

	let mut to_write: String = format!("generated by bsp_dump {VERSION}\n");
	to_write.push_str(&format!(
		"file name: {}\n\n",
		path.split("/").last().unwrap_or(""),
	));

	//      ====header dumping====
	to_write.push_str("====header====\n\n");
	to_write.push_str(&format!(
		"bsp version: {}\nmap revision: {}\n\n",
		file.header.version,
		file.header.map_revision,
	));

	for l_info in file.header.lumps {
		to_write.push_str(&format!(
			"lump {} info:\n\tfile offset: {} bytes\n\t",
			l_info.index,
			l_info.file_offset,
		));
		to_write.push_str(&format!(
			"length: {} bytes\n\tversion: {}\n\tident: {:?}\n\n",
			l_info.length,
			l_info.version,
			l_info.ident,
		));
	}

	//      ====lump dumping====
	to_write.push_str("====lumps====\n");
	
	// LUMP_ENTITIES
	to_write.push_str("\nLUMP_ENTITIES (index 0)\n");
	if let LumpType::Entities(ents) = &file.lump_data[0] {
		let mut counter: u32 = 0;
		for ent in ents {
			to_write.push_str(&format!("\t[ent{counter}]\n"));
			for pair in ent {
				to_write.push_str(&format!("\t\t{}: {}\n", pair.0, pair.1));
			}
			counter += 1;
		}
	}

	// LUMP_PLANES
	to_write.push_str("\nLUMP_PLANES (index 1)\n");
	if let LumpType::Planes(planes) = &file.lump_data[1] {
		let mut counter: u32 = 0;
		for plane in planes {
			to_write.push_str(&format!("\t[pln{counter}]\n"));
			to_write.push_str(&format!(
				"\t\tnormal: {}\n\t\tdist: {}\n\t\ttype: {}\n",
				plane.normal, plane.dist, plane.r#type, 
			));
			counter += 1;
		}
	}

	// LUMP_TEXDATA
	to_write.push_str("\nLUMP_TEXDATA (index 2)\n");
	if let LumpType::TexData(texdatas) = &file.lump_data[2] {
		let mut counter: u32 = 0;
		for texdata in texdatas {
			to_write.push_str(&format!("\t[tdata{counter}]\n"));
			// splitting these up for the lines arent long
			to_write.push_str(&format!(
				"\t\treflectivity: {}\n\t\tname_string_table_id: {}\n",
				texdata.reflectivity, texdata.name_string_table_id,
			));
			to_write.push_str(&format!(
				"\t\twidth/height: ({}, {})\n\t\tview_width/height: ({}, {})\n",
				texdata.width, texdata.height, texdata.view_width, texdata.view_height
			));
			counter += 1;
		}
	}

	// LUMP_VERTEXES
	to_write.push_str("\nLUMP_VERTEXES (index 3)\n");
	if let LumpType::Vertexes(vertexes) = &file.lump_data[3] {
		let mut counter: u32 = 0;
		for vertex in vertexes {
			to_write.push_str(&format!("\t[vtx{counter}] {vertex}\n"));
			counter += 1;
		}
	}

	// LUMP_VISIBILITY
	to_write.push_str("\nLUMP_VISIBILITY (index 4)\n====NOT FINISHED====\n");
	if let LumpType::Visibility(vis) = &file.lump_data[4] {
		let mut counter: u32 = 0;
		to_write.push_str(&format!(
			"\tnum_clusters: {}\n\tbyte_offsets:\n",
			vis.num_clusters
		));
		for offsets in &vis.byte_offsets {
			to_write.push_str(&format!(
				"\t\t[visofs{counter}] PVS: {}, PAS: {}\n",
				offsets[0], offsets[1],
			));
			counter += 1;
		}
	}

	// LUMP_NODES
	to_write.push_str("\nLUMP_VERTEXES (index 5)\n");
	if let LumpType::Nodes(nodes) = &file.lump_data[5]{
		let mut counter: u32 = 0;
		for node in nodes {
			to_write.push_str(&format!("\t[node{counter}]\n"));
			to_write.push_str(&format!(
				"\t\tplane_num: {}\n",
				node.plane_num,
			));

			let mut children_string = "\t\tchildren: ".to_string();
			children_string.push_str(&node.children[0].to_string());
			if node.children[0] > 0 {
				children_string.push_str(" (node), ");
			} else {
				children_string.push_str(&format!(
					" (leaf {}), ",
					-1 - node.children[0]
				));
			}

			children_string.push_str(&node.children[1].to_string());
			if node.children[1] > 0 {
				children_string.push_str(" (node)\n");
			} else {
				children_string.push_str(&format!(
					" (leaf {})\n",
					-1 - node.children[1]
				));
			}

			to_write.push_str(&children_string);
			to_write.push_str(&format!(
				"\t\tmins: ({}, {}, {})\n\t\tmaxs: ({}, {}, {})\n",
				node.mins[0], node.mins[1], node.mins[2],
				node.maxs[0], node.maxs[1], node.maxs[2],
			));
			to_write.push_str(&format!(
				"\t\tfirst_face: {}\n\t\tnumfaces: {}\n\t\tarea: {}\n\t\tpadding: {}\n",
				node.first_face, node.numfaces, node.area, node.padding,
			));
			counter += 1;
		}
	}

	// LUMP_TEXINFO
	to_write.push_str("\nLUMP_VERTEXES (index 6)\n");
	if let LumpType::TexInfo(texinfos) = &file.lump_data[6] {
		let mut counter: u32 = 0;
		for texinfo in texinfos {
			to_write.push_str(&format!("\t[tinfo{counter}]\n"));
			to_write.push_str(&format!(
				"\t\ttexture_vecs:\n\t\t\t{:?}\n\t\t\t{:?}\n\t\tlightmap_vecs:\n\t\t\t{:?}\n\t\t\t{:?}\n",
				texinfo.texture_vecs[0], texinfo.texture_vecs[1],
				texinfo.lightmap_vecs[0], texinfo.lightmap_vecs[1],
			));
			to_write.push_str(&format!(
				"\t\tflags: {}\n\t\ttexdata: {}\n",
				texinfo.flags, texinfo.texdata,
			));
			counter += 1;
		}
	}

	// LUMP_FACES
	to_write.push_str("\nLUMP_FACES (index 7)\n");
	if let LumpType::Faces(faces) = &file.lump_data[7] {
		let mut counter: u32 = 0;
		// this is a big one
		for face in faces {
			to_write.push_str(&format!("\t[face{counter}]\n"));
			to_write.push_str(&format!(
				"\t\tplane_num: {}\n\t\tside: {}\n\t\ton_node: {} ({})\n",
				face.plane_num, face.side, face.on_node, face.on_node == 1,
			));
			to_write.push_str(&format!(
				"\t\tfirst_surfedge: {}\n\t\tnum_edges: {}\n\t\ttexinfo: {}\n",
				face.first_edge, face.num_edges, face.tex_info,
			));
			to_write.push_str(&format!(
				"\t\tdispinfo: {}\n\t\tsurface_fog_volume_id: {}\n",
				face.disp_info, face.surface_fog_volume_id,
			));
			to_write.push_str(&format!(
				"\t\tstyles: {:?}\n\t\tlight_offset: {} (light{})\n\t\tarea: {}\n",
				face.styles, face.light_offset, face.light_offset / 4, face.area,
			));
			to_write.push_str(&format!(
				"\t\tlightmap_texture_mins: ({}, {})\n\t\tlightmap_texture_size: ({}, {})\n",
				face.lightmap_texture_mins[0], face.lightmap_texture_mins[1],
				face.lightmap_texture_size[0], face.lightmap_texture_size[1],
			));
			to_write.push_str(&format!(
				"\t\torig_face: {}\n\t\tfirst_prim_id, num_prims: {}, {}\n\t\tsmoothing_groups: {}\n",
				face.orig_face, face.first_prim_id, face.num_prims, face.smoothing_groups,
			));

			counter += 1;
		}
	}

	// LUMP_LIGHTING
	to_write.push_str("\nLUMP_LIGHTING (index 8)\n");
	if let LumpType::Lighting(lightings) = &file.lump_data[8] {
		let mut counter: u32 = 0;
		for lighting in lightings {
			to_write.push_str(&format!(
				"\t[light{counter}] r, g, b, exp: {}, {}, {}, {}\n",
				lighting.r, lighting.g, lighting.b, lighting.exponent,
			));
			counter += 1;
		}
	}

	// LUMP_OCCLUSION
	to_write.push_str("\nLUMP_OCCLUSION (index 9)\n");
	if let LumpType::Occlusion(occluder) = &file.lump_data[9] {
		to_write.push_str(&format!("\toccluder_data ({} entries)\n", occluder.count));
		for i in 0..occluder.count {
			let data: &lumps::OccluderData = &occluder.data[i as usize];
			to_write.push_str(&format!("\t\t[occdata{i}]\n"));
			to_write.push_str(&format!(
				"\t\t\tflags: {}\n\t\t\tfirst_poly: {}\n\t\t\tpoly_count: {}\n",
				data.flags, data.first_poly, data.poly_count,
			));
			to_write.push_str(&format!(
				"\t\t\tmins: {}\n\t\t\tmaxs: {}\n\t\t\tarea: {}\n",
				data.mins, data.maxs, data.area
			));
		}
		to_write.push_str(&format!("\tpoly_data ({} entries)\n", occluder.poly_data_count));
		for i in 0..occluder.poly_data_count {
			let data: &lumps::OccluderPolyData = &occluder.poly_data[i as usize];
			to_write.push_str(&format!("\t\t[polydata{i}]\n"));
			to_write.push_str(&format!(
				"\t\t\tfirst_vertex_index: {}\n\t\t\tvertex_count: {}\n\t\t\tplane_num: {}\n",
				data.first_vertex_index, data.vertex_count, data.plane_num,
			));
		}
	}

	// LUMP_LEAFS
	to_write.push_str("\nLUMP_LEAFS (index 10)\n");
	if let LumpType::Leafs(leafs) = &file.lump_data[10] {
		let mut counter: u32 = 0;
		for leaf in leafs {
			to_write.push_str(&format!("\t[leaf{counter}]\n"));
			to_write.push_str(&format!(
				"\t\tcontents: {}\n\t\tcluster: {}\n\t\tarea_flags: {}\n",
				leaf.contents, leaf.cluster, leaf.area_flags,
			));
			to_write.push_str(&format!(
				"\t\tmins: {:?}\n\t\tmaxs: {:?}\n\t\tfirst_leaf_face, num_leaf_faces: {}, {}\n",
				leaf.mins, leaf.maxs, leaf.first_leaf_face, leaf.num_leaf_faces
			));
			to_write.push_str(&format!(
				"\t\tfirst_leaf_brushes, num_leaf_brushes: {} {}\n\t\tin_water: {} ({})\n",
				leaf.first_leaf_brushes, leaf.num_leaf_brushes, leaf.in_water, leaf.in_water != -1,
			));
			if file.header.lumps[10].version == 0 {
				let amb_lighting: lumps::CompressedLightCube = leaf.ambient_lighting.unwrap();
				to_write.push_str("\t\tambient lighting:\n");
				for color in amb_lighting.color {
					to_write.push_str(&format!(
						"\t\t\tr, g, b, exp: {}, {}, {}, {}\n",
						color.r, color.g, color.b, color.exponent
					));
				}
			}
			to_write.push_str(&format!("\t\tpadding: {}\n", leaf.padding));
			counter += 1;
		}
	}

	// LUMP_FACEIDS
	to_write.push_str("\nLUMP_FACEIDS (index 11)\n");
	if file.header.lumps[11].length == 0 {
		to_write.push_str("\tempty/nonexistent\n");
	} else if let LumpType::FaceIDs(faceids) = &file.lump_data[11] {
		let mut counter: u32 = 0;
		for faceid in faceids {
			to_write.push_str(&format!("\t\t[faceid{counter}] id: {}\n", faceid.id));
			counter += 1;
		}
	}
	
	// LUMP_EDGES
	to_write.push_str("\nLUMP_EDGES (index 12)\n");
	if let LumpType::Edges(edges) = &file.lump_data[12] {
		let mut counter: u32 = 0;
		for edge in edges {
			to_write.push_str(&format!(
				"\t[edge{}] vertices: {}, {}\n",
				counter, edge.pair[0], edge.pair[1] 
			));
			counter += 1;
		}
	}
	
	// LUMP_SURFEDES
	to_write.push_str("\nLUMP_SURFEDGES (index 13)\n");
	to_write.push_str("positive number: edge referenced from 1st to 2nd vertex (->)\n");
	to_write.push_str("negative number: edge referenced from 2nd to 1st vertex (<-)\n");
	if let LumpType::SurfEdges(surfedges) = &file.lump_data[13] {
		let mut counter: u32 = 0;
		for surfedge in surfedges {
			to_write.push_str(&format!(
				"\t[sedge{}] edge: {} ({})\n",
				counter, surfedge, if *surfedge < 0 { "<-" } else { "->" }
			));
			counter += 1;
		}
	}

	// LUMP_MODELS
	to_write.push_str("\nLUMP_MODELS (index 14)\n");
	if let LumpType::Models(models) = &file.lump_data[14] {
		let mut counter: u32 = 0;
		for model in models {
			to_write.push_str(&format!("\t[model{counter}]\n"));
			to_write.push_str(&format!(
				"\t\tmins: {}\n\t\tmaxs: {}\n\t\torigin: {}\n",
				model.mins, model.maxs, model.origin,
			));
			to_write.push_str(&format!(
				"\t\thead_node: {}\n\t\tfirst_face: {}\n\t\tnum_faces: {}\n",
				model.head_node, model.first_face, model.num_faces,
			));
			counter += 1;
		}
	}

	// LUMP_WORLDLIGHTS
	to_write.push_str("\nLUMP_WORLDLIGHTS (index 15)\n");
	to_write.push_str("no information available yet!\n");

	// LUMP_LEAFFACES
	to_write.push_str("\nLUMP_WORLDLIGHTS (index 16)\n");
	to_write.push_str("no information available yet!\n");

	// LUMP_LEAFBRUSHES
	to_write.push_str("\nLUMP_LEAFBRUSHES (index 17)\n");
	to_write.push_str("no information available yet!\n");

	// LUMP_BRUSHES
	to_write.push_str("\nLUMP_BRUSHES (index 18)\n");
	if let LumpType::Brushes(brushes) = &file.lump_data[18] {
		let mut counter: u32 = 0;
		for brush in brushes {
			to_write.push_str(&format!("\t[brush{counter}]\n"));
			to_write.push_str(&format!(
				"\t\tfirst_side: {}\n\t\tnum_sides: {}\n\t\tcontents: {}\n",
				brush.first_side, brush.num_sides, brush.contents,
			));
			counter += 1;
		}
	}

	// LUMP_BRUSHSIDES
	to_write.push_str("\nLUMP_BRUSHSIDES (index 19)\n");
	if let LumpType::BrushSides(brsides) = &file.lump_data[19] {
		let mut counter: u32 = 0;
		for brside in brsides {
			to_write.push_str(&format!("\t[brside{counter}]\n"));
			to_write.push_str(&format!(
				"\t\tplane_num: {}\n\t\ttexinfo: {}\n\t\tdispinfo: {}\n\t\tbevel: {} ({})\n",
				brside.plane_num, brside.texinfo, brside.dispinfo,
				brside.bevel, brside.bevel == 1,
			));
			counter += 1;
		}
	}

	// LUMP_AREAS
	to_write.push_str("\nLUMP_AREAS (index 20)\n");
	if let LumpType::Areas(areas) = &file.lump_data[20] {
		let mut counter: u32 = 0;
		for area in areas {
			to_write.push_str(&format!("\t[area{counter}]\n"));
			to_write.push_str(&format!(
				"\t\tnum_area_portals: {}\n\t\tfirst_area_portal: {}\n",
				area.num_area_portals, area.first_area_portal,
			));
			counter += 1;
		}
	}

	// LUMP_AREAPORTALS
	to_write.push_str("\nLUMP_AREAPORTALS (index 21)\n");
	if let LumpType::AreaPortals(areaps) = &file.lump_data[21] {
		let mut counter: u32 = 0;
		for areap in areaps {
			to_write.push_str(&format!("\t[areap{counter}]\n"));
			to_write.push_str(&format!(
				"\t\tportal_key: {}\n\t\tother_area: {}\n\t\tfirst_clip_portal_vert: {}\n",
				areap.portal_key, areap.other_area, areap.first_clip_portal_vert,
			));
			to_write.push_str(&format!(
				"\t\tclip_portal_verts: {}\n\t\tplane_num: {}\n",
				areap.clip_portal_verts, areap.plane_num,
			));
			counter += 1;
		}
	}
	
	// LUMP_UNUSED22/23/24/25
	to_write.push_str("\nLUMP_UNUSED22/23/24/25\n");
	to_write.push_str("\tno data, these are unused\n");

	// LUMP_DISPINFO
	to_write.push_str("\nLUMP_DISPINFO (index 26)\n");
	if let LumpType::DispInfo(dispinfos) = &file.lump_data[26] {
		let mut counter: u32 = 0;
		// another big one
		for dispinfo in dispinfos {
			to_write.push_str(&format!("\t[dispinf{counter}]\n"));
			to_write.push_str(&format!(
				"\t\tstart_position: {}\n\t\tdisp_vert_start: {}\n\t\tdisp_tri_start: {}\n",
				dispinfo.start_position, dispinfo.disp_vert_start, dispinfo.disp_tri_start,
			));
			to_write.push_str(&format!(
				"\t\tpower: {}\n\t\tmin_tess: {}\n\t\tsmoothing_angle: {}\n",
				dispinfo.power, dispinfo.min_tess, dispinfo.smoothing_angle,
			));
			to_write.push_str(&format!(
				"\t\tcontents: {}\n\t\tmap_face: {}\n\t\tlightmap_alpha_start: {}\n",
				dispinfo.contents, dispinfo.map_face, dispinfo.lightmap_alpha_start,
			));
			to_write.push_str(&format!(
				"\t\tlightmap_sample_position_start: {}\n\t\tallowed_verts: {:?}\n",
				dispinfo.lightmap_sample_position_start, dispinfo.allowed_verts,
			));
			counter += 1;
		}
		// apparently this one can just be empty for some reason
		if counter == 0 {
			to_write.push_str("\tempty\n");
		}
	}

	// LUMP_ORIGINALFACES
	to_write.push_str("\nLUMP_ORIGINALFACES (index 27)\n");
	if let LumpType::Faces(faces) = &file.lump_data[7] {
		let mut counter: u32 = 0;
		// same exact structure as faces array
		for face in faces {
			to_write.push_str(&format!("\t[origface{counter}]\n"));
			to_write.push_str(&format!(
				"\t\tplane_num: {}\n\t\tside: {}\n\t\ton_node: {} ({})\n",
				face.plane_num, face.side, face.on_node, face.on_node == 1,
			));
			to_write.push_str(&format!(
				"\t\tfirst_surfedge: {}\n\t\tnum_edges: {}\n\t\ttexinfo: {}\n",
				face.first_edge, face.num_edges, face.tex_info,
			));
			to_write.push_str(&format!(
				"\t\tdispinfo: {}\n\t\tsurface_fog_volume_id: {}\n",
				face.disp_info, face.surface_fog_volume_id,
			));
			to_write.push_str(&format!(
				"\t\tstyles: {:?}\n\t\tlight_offset: {} (light{})\n\t\tarea: {}\n",
				face.styles, face.light_offset, face.light_offset / 4, face.area,
			));
			to_write.push_str(&format!(
				"\t\tlightmap_texture_mins: ({}, {})\n\t\tlightmap_texture_size: ({}, {})\n",
				face.lightmap_texture_mins[0], face.lightmap_texture_mins[1],
				face.lightmap_texture_size[0], face.lightmap_texture_size[1],
			));
			to_write.push_str(&format!(
				"\t\torig_face: {}\n\t\tfirst_prim_id, num_prims: {}, {}\n\t\tsmoothing_groups: {}\n",
				face.orig_face, face.first_prim_id, face.num_prims, face.smoothing_groups,
			));

			counter += 1;
		}
	}

	// done!
	println!(
		"dumping finished! wrote {} bytes",
		dump_file.write(to_write.as_bytes()).unwrap(),
	);
}