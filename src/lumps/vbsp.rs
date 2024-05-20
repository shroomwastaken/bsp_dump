// all of the information about the lump structure was taken mostly from the wiki:
// https://developer.valvesoftware.com/wiki/BSP_(Source)
// and from other sources, too:
// https://pysourcesdk.github.io/ValveBSP/datastructures.html

use crate:: {
	utils::Vector3,
	specific::{
		cdisp,
		physcol_data,
		occlusion,
		gamelump,
	},
	flags::{
		ContentsFlags,
		SurfaceFlags,
		DispTriFlags,
	},
};

#[derive(Debug, Clone)]
#[allow(unused)]
pub enum VBSPLumpType {
	None,
	Entities(Vec<Vec<(String, String)>>),
	Planes(Vec<Plane>),
	TexData(Vec<TexData>),
	Vertices(Vec<Vector3>),
	Visibility(Vis),
	Nodes(Vec<Node>),
	TexInfo(Vec<TexInfo>),
	Faces(Vec<Face>),
	Lighting(Vec<ColorRGBExp32>),
	Occlusion(Occluder),
	Leaves(Vec<Leaf>),
	FaceIDs(Vec<FaceID>),
	Edges(Vec<Edge>),
	// vector of indices into Edges
	// abs(number) is the index
	// if number is positive, the edge is defined from 1st to 2nd vertex
	// if number is negative, the edge is defined from 2nd to 1st vertex
	SurfEdges(Vec<i32>),
	Models(Vec<Model>),
	WorldLights,
	LeafFaces,
	LeafBrushes,
	Brushes(Vec<Brush>),
	BrushSides(Vec<BrushSide>),
	Areas(Vec<Area>),
	AreaPortals(Vec<AreaPortal>),
	Portals,
	Unused22,
	PropCollision,
	Clusters,
	Unused23,
	PropHulls,
	PortalVerts,
	Unused24,
	PropHullVerts,
	ClusterPortals,
	Unused25,
	PropTrips,
	DispInfo(Vec<DispInfo>),
	OriginalFaces(Vec<Face>),
	PhyDisp(Vec<PhyDisp>),
	PhysCollide(Vec<PhysModel>),
	VertNormal(Vec<VertexNormal>),
	VertNormalIndices(Vec<VertexNormalIndex>),
	DispLightmapAlphas,
	DispVerts(Vec<DispVert>),
	DispLightmapSamplePositions(Vec<DispLightmapSamplePosition>),
	// lump count, lump data
	GameLump(GameLump),
	LeafWaterData,
	Primitives(Vec<Primitive>),
	PrimVerts(Vec<PrimVert>),
	PrimIndices(Vec<PrimIndex>),
	// this is literally just a zip archive of files lmao
	PakFile(PakFile),
	ClipPortalVerts(Vec<ClipPortalVert>),
	Cubemaps(Vec<CubemapSample>),
	TexDataStringData(Vec<TexDataStringData>),
	TexDataStringTable(Vec<TexDataStringTable>),
	Overlays(Vec<Overlay>),
	LeafMinDistToWater(Vec<LeafMinDistToWater>),
	FaceMacroTextureInfo(Vec<FaceMacroTextureInfo>),
	DispTris(Vec<DispTriFlags>),
	PhysCollideSurface,
	PropBlob,
	WaterOverlays,
	LightMapPages,
	LeafAmbientIndexHDR(Vec<LeafAmbientIndex>),
	LightmapPageInfos,
	LeafAmbientIndex(Vec<LeafAmbientIndex>),
	LightingHDR,
	WorldLightsHDR,
	LeafAmbientLightingHDR(Vec<LeafAmbientLighting>),
	LeafAmbientLighting(Vec<LeafAmbientLighting>),
	XZipPakFile,
	FacesHDR,
	MapFlags,
	OverlayFades,
	OverlaySystemLevels,
	PhysLevel,
	DispMultibend,
}


#[derive(Debug, Clone, Copy)]
pub struct Edge {
	// pair of vertex indices,
	// a straight line between two vertices is an edge
	pub pair: [u16; 2]
}

#[derive(Debug, Clone, Copy)]
pub struct Plane {
	pub normal: Vector3, // normal vector
	pub dist: f32, // distance from origin
	pub r#type: i32, // plane axis identifier
}

#[derive(Debug, Clone, Copy)]
pub struct Face {
	pub plane_num: u16, // the plane number
	pub side: u8, // faces opposite to the nodes plane direction
	pub on_node: u8, // 1 if on node, 0 if in leaf
	pub first_edge: u32, // index into surfedges,
	pub num_edges: i16, // number of surfedges
	pub tex_info: i16, // texture info
	pub disp_info: i16, // displacement info
	pub surface_fog_volume_id: i16, // not even the wiki knows what this is
	pub styles: [u8; 4], // switchable lighting info
	pub light_offset: i32, // offset into lightmap lump
	pub area: f32, // face area in units squared
	pub lightmap_texture_mins: [i32; 2], // both of these are "in luxels"
	pub lightmap_texture_size: [i32; 2], // texture lighting info
	pub orig_face: i32, // original face this was split from
	pub num_prims: u16, // primitives
	pub first_prim_id: u16,
	pub smoothing_groups: u32, // lightmap smoothing group
}

#[derive(Debug, Clone, Copy)]
pub struct Node {
	pub plane_num: i32, // index into plane array

	// if positive these are node indices,
	// if negative, the value (-1-child) is an index into the leaf array
	// for example value -100 would be leaf 99
	pub children: [i32; 2],

	pub mins: [i16; 3], // these are rough coordinates of
	pub maxs: [i16; 3], // the bounding box this node has

	pub first_face: u16, // indices into face array which show
	pub numfaces: u16,   // which faces are contained in this node, 0 if none
	pub area: i16, // map area of this node
	pub padding: i16 // we dont need this
}

#[derive(Debug, Clone, Copy)]
pub struct Leaf {
	pub contents: ContentsFlags, // flags, same as in brush lump
	pub cluster: i16,
	pub area_flags: i16, // bitfield, area takes 9 bits, flags takes 7

	pub mins: [i16; 3], // these are rough coordinates of
	pub maxs: [i16; 3], // the bounding box this leaf has

	pub first_leaf_face: u16, // these are indices into leaffaces array
	pub num_leaf_faces: u16,

	pub first_leaf_brushes: u16, // these are indices into leafbrushes array
	pub num_leaf_brushes: u16,

	pub in_water: i16, // -1 if not in water

	pub ambient_lighting: Option<CompressedLightCube>, // only in lump version 0

	pub padding: i16,
}

#[derive(Debug, Clone, Copy)]
pub struct FaceID {
	pub id: u16,
}

#[derive(Debug, Clone, Copy)]
pub struct TexInfo {
	pub texture_vecs: [[f32; 4]; 2],
	pub lightmap_vecs: [[f32; 4]; 2],
	pub flags: SurfaceFlags,
	pub texdata: i32, // index into texdata array
}

#[derive(Debug, Clone, Copy)]
pub struct TexData {
	pub reflectivity: Vector3,
	pub name_string_table_id: i32, // index into TexdataStringTable array
	pub width: i32,
	pub height: i32,
	pub view_width: i32,
	pub view_height: i32,
}

#[derive(Debug, Clone, Copy)]
pub struct Model {
	pub mins: Vector3, // bounding box
	pub maxs: Vector3,

	pub origin: Vector3,

	pub head_node: i32, // index into node array

	pub first_face: i32, // indices into face array
	pub num_faces: i32,
}

#[derive(Debug, Clone, Copy)]
pub struct WorldLight {

}

#[derive(Debug, Clone, Copy)]
pub struct Brush {
	pub first_side: i32, // index into brushside array
	pub num_sides: i32, // firstside and the next numsides make up all the sides in the brush
	pub contents: ContentsFlags,
}

#[derive(Debug, Clone, Copy)]
pub struct BrushSide {
	pub plane_num: u16, // index into planes array
	pub texinfo: i16, // index into texinfo array
	pub dispinfo: i16, // index into dispinfo array

	// why is this an i16 lmao
	pub bevel: i16, // 1 if side is a bevel plane
}

#[derive(Debug, Clone, Copy)]
pub struct Area {
	pub num_area_portals: i32, // first_area_portal + num_area_portals portals make up the area (?)
	pub first_area_portal: i32, // index into areaportals array (?)
}

#[derive(Debug, Clone, Copy)]
pub struct AreaPortal {
	// Entities have a key called portalnumber (and in vbsp a variable
	// called areaportalnum) which is used to bind them
	// to the area portals by comparing with this value.
	pub portal_key: u16,

	pub other_area: u16, // the area this portal looks into
	pub first_clip_portal_vert: u16, // portal geometry
	pub clip_portal_verts: u16,
	pub plane_num: i32,
}

#[derive(Debug, Clone)]
pub struct Vis {
	pub num_clusters: i32,

	// byte offsets into pvs and pas arrays from the start of this lump for every cluster
	// this is of length num_clusters but i can't define it like that :(
	// so ill have to use a vec
	pub byte_offsets: Vec<[i32; 2]>,

	// here ill store the decompressed PVS and PAS data
	// that is the clusters visible and audible from each cluster
	pub cluster_data: [Vec<Vec<bool>>; 2]
}

#[derive(Debug, Clone)]
pub struct GameLump {
	pub header: gamelump::GameLumpHeader,
	pub data: Vec<gamelump::GameLumpData>
}

#[derive(Debug, Clone, Copy)]
pub struct DispInfo {
	pub start_position: Vector3,
	pub disp_vert_start: i32,
	pub disp_tri_start: i32,
	pub power: i32,
	pub min_tess: i32,
	pub smoothing_angle: f32,
	pub contents: ContentsFlags,
	// unsigned short in sdk2013 but it seems to be stored as u32 in hl2 maps
	pub map_face: u32,
	pub lightmap_alpha_start: i32,
	pub lightmap_sample_position_start: i32,
	pub edge_neighbors: [cdisp::CDispNeighbor; 4],
	pub corner_neighbors: [cdisp::CDispCornerNeighbors; 4],
	// unsigned long in sdk2013 but it seems a lot of them are -1 in hl2 maps
	pub allowed_verts: [i32; 10],
}

#[derive(Debug, Clone, Copy)]
pub struct DispVert {
	pub vec: Vector3, // normalized vector of the offset of each displacement vertex from its original (flat) position
	pub dist: f32, // distance the offset has taken place
	pub alpha: f32, // alpha-blending of the texture at that vertex
}

#[derive(Debug, Clone, Copy)]
pub struct CubemapSample {
	pub origin:[i32; 3],
	pub size: i32,
}

#[derive(Debug, Clone, Copy)]
pub struct Overlay {
	pub id: i32,
	pub texinfo: i16,
	pub face_count_and_render_order: u16,
	pub faces: [i32; 64],

	// ???
	pub u: [f32; 2],
	pub v: [f32; 2],
	pub uv_points: [Vector3; 4],

	pub origin: Vector3,
	pub basis_normal: Vector3,
}

#[derive(Debug, Clone, Copy)]
pub struct ColorRGBExp32 {
	pub r: u8, pub g: u8, pub b: u8,
	pub exponent: i8,
}

#[derive(Debug, Clone, Copy)]
pub struct CompressedLightCube {
	pub color: [ColorRGBExp32; 6],
}

#[derive(Debug, Clone, Copy)]
pub struct LeafAmbientLighting {
	pub cube: CompressedLightCube,
	pub x: u8, pub y: u8, pub z: u8,
	pub padding: u8
}

#[derive(Debug, Clone, Copy)]
pub struct LeafAmbientIndex {
	pub ambient_sample_count: u16,
	pub first_ambient_sample: u16,
}

#[derive(Debug, Clone)]
pub struct Occluder {
	pub count: i32,
	pub data: Vec<occlusion::OccluderData>, // of length count
	pub poly_data_count: i32,
	pub poly_data: Vec<occlusion::OccluderPolyData>, // of length poly_data_count
	pub vertex_index_count: i32,
	pub vertex_indices: Vec<i32>, // of length vertex_index_count
}

#[derive(Debug, Clone)]
pub struct PhysModel {
	pub model_index: i32,
	pub data_size: i32, // size of collision data section
	pub keydata_size: i32, // size of text section
	pub solid_count: i32, // number of collision data sections
	pub collision_data: Vec<physcol_data::CollisionData>,
	// key {same format as entity string}
	pub key_data: Vec<(String, Vec<(String, String)>)>,
}

#[derive(Debug, Clone, Copy)]
pub struct PhyDisp {
	pub num_disps: u16,
	// data_size: Vec<u16>
	// this is commented out in the sdk so idk
}

// i just want everything to be a struct ok
#[derive(Debug, Clone, Copy)]
pub struct VertexNormal {
	pub normal: Vector3,
}

// and i mean everything
#[derive(Debug, Clone, Copy)]
pub struct VertexNormalIndex {
	pub index: u16
}

#[derive(Debug, Clone, Copy)]
pub struct DispLightmapSamplePosition {
	pub unknown: u8, // no clue
}

#[derive(Debug, Clone, Copy)]
pub struct Primitive {
	// this is an unsigned char in 2013 sdk
	// but its an unsigned short in portal maps
	// TODO: check an hl2 map
	pub r#type: u16,

	pub first_index: u16,
	pub num_indices: u16,
	pub first_vertex: u16,
	pub num_vertices: u16,
}

#[derive(Debug, Clone, Copy)]
pub struct PrimVert {
	pub pos: Vector3,
}

#[derive(Debug, Clone, Copy)]
pub struct PrimIndex {
	pub index: u16,
}

#[derive(Debug, Clone)]
pub struct PakFile {
	pub bytes: Vec<u8>
}

#[derive(Debug, Clone)]
pub struct ClipPortalVert {
	pub vec: Vector3,
}

#[derive(Debug, Clone)]
pub struct TexDataStringData {
	pub val: String,
	// ill store the offset so that i can use it with
	// texdatastringtable values
	pub offset: usize,
}

#[derive(Debug, Clone)]
pub struct TexDataStringTable {
	// type is actually unknown this just makes the most sense
	pub offset: u32,
}

#[derive(Debug, Clone)]
pub struct LeafMinDistToWater {
	pub dist: i32,
}

#[derive(Debug, Clone)]
pub struct FaceMacroTextureInfo {
	// seemingly indices into lump 44
	// but then theyre also -1 a lot of the time so idk
	pub index: i32
}