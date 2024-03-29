// https://github.com/ValveSoftware/source-sdk-2013/blob/master/sp/src/utils/common/bsplib.cpp#L1417
// https://www.flipcode.com/archives/Quake_2_BSP_File_Format.shtml

// returns a vec of bools where true means the cluster is visible
// and false means it isnt
pub fn decompress_vis(inp: &[u8], num_clusters: &i32) -> Vec<bool> {
	let mut res: Vec<bool> = vec![false; *num_clusters as usize];
	let mut c: usize = 0;
	let mut v: usize = 0;
	while c < *num_clusters as usize{
		if inp[v] == 0 {
			v += 1;
			c += 8 * inp[v] as usize;
		} else {
			// TODO: ugly code fix it
			let mut bit: u8 = 1;
			while bit != 0 {
				res[c] = (inp[v] & bit) != 0;
				bit <<= 2; c += 1;
				if c >= *num_clusters as usize {
					println!("warning overrun");
					break;
				}
			}
		}
		v += 1;
	}

	res
}
