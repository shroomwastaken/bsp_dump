use core::fmt;

#[derive(Debug, Clone, Copy)]
pub struct Vector3 {
	pub x: f32,
	pub y: f32,
	pub z: f32,
}

impl Vector3 {
	pub fn new() -> Vector3 {
		Vector3 { x: 0.0, y: 0.0, z: 0.0 }
	}
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

pub fn int_to_gsrc_planetype(val: &i32) -> &str {
	match *val {
		0 => "PLANE_X",
		1 => "PLANE_Y",
		2 => "PLANE_Z",
		3 => "PLANE_ANYX",
		4 => "PLANE_ANYY",
		5 => "PLANE_ANYZ",
		_ => "error / undefined type"
	}
}

pub fn int_to_quake_texflag(val: &i32) -> &str {
	match *val {
		0 => "None",
		1 => "TEX_SPECIAL",
		_ => "error / undefined flag",
	}
}