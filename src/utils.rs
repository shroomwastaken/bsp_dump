use core::fmt;

#[derive(Debug, Clone, Copy)]
pub struct Vector3 {
	pub x: f32,
	pub y: f32,
	pub z: f32,
}

impl fmt::Display for Vector3 {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "({}, {}, {})", self.x, self.y, self.z)
	}
}

// copied from
// https://github.com/shroomwastaken/iipdp/blob/master/src/structs/utils.rs#L322
pub fn bitflags_to_string<B: bitflags::Flags>(names: bitflags::iter::IterNames<B>) -> String {
    let mut flag_str = "".to_string();
    for name in names {
        flag_str.push_str(name.0);
        flag_str.push_str(" | ");
    }
    if flag_str == "" {
        flag_str = "None".to_string();
    } else {
        flag_str = flag_str[..flag_str.len() - 3].to_string();
    }
    
    return flag_str;
}