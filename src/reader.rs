use crate::utils::{
	Vector3,
	CDispCornerNeighbors,
	CDispSubNeighbor,
	CDispNeighbor,
};
use crate::lumps;

pub struct Reader {
	pub bytes: Vec<u8>,
	pub index: usize,
}

impl Reader {
	pub fn new(
		bytes: Vec<u8>,
	) -> Reader {
		Reader { bytes, index: 0 }
	}

	pub fn read_bytes(
		&mut self,
		amount: usize,
	) -> Vec<u8> {
		self.index += amount;
		self.bytes[self.index - amount..self.index].to_vec()
	}

	// assume all strings are null terminated
	// errors out if bytes are invalid utf8
	pub fn read_string(
		&mut self
	) -> String {
		let next_null: usize = find_next_null_byte(&self.bytes[self.index..]);
		String::from_utf8(self.read_bytes(next_null)).unwrap()
	}

	pub fn read_int(
		&mut self,
	) -> i32 {
		i32::from_le_bytes(
			self.read_bytes(4)
			.try_into()
			.unwrap()
		)
	}

	pub fn read_short(
		&mut self,
	) -> i16 {
		i16::from_le_bytes(
			self.read_bytes(2)
			.try_into()
			.unwrap()
		)
	}
	
	pub fn read_uint(
		&mut self,
	) -> u32 {
		u32::from_le_bytes(
			self.read_bytes(4)
			.try_into()
			.unwrap()
		)
	}
	
	pub fn read_ushort(
		&mut self,
	) -> u16 {
		u16::from_le_bytes(
			self.read_bytes(2)
			.try_into()
			.unwrap()
		)
	}

	pub fn read_float(
		&mut self,
	) -> f32 {
		f32::from_le_bytes(
			self.read_bytes(4)
			.try_into()
			.unwrap()
		)
	}
	
	pub fn read_vector3(
		&mut self,
	) -> Vector3 {
		Vector3 {
			x: self.read_float(),
			y: self.read_float(),
			z: self.read_float(),
		}
	}

	pub fn skip(
		&mut self,
		amount: usize,
	) {
		self.index += amount;
	}

	// very unnecessary these are just shortcuts
	pub fn read_byte(
		&mut self,
	) -> u8 {
		self.read_bytes(1)[0]
	}

	pub fn read_signed_byte(
		&mut self,
	) -> i8 {
		self.read_bytes(1)[0] as i8
	}

	pub fn read_colorrgbexp32(
		&mut self,
	) -> lumps::ColorRGBExp32 {
		lumps::ColorRGBExp32 {
			r: self.read_byte(),
			g: self.read_byte(),
			b: self.read_byte(),
			exponent: self.read_signed_byte(),
		}
	}

	pub fn read_compressed_light_cube(
		&mut self,
	) -> lumps::CompressedLightCube {
		lumps::CompressedLightCube {
			color: [
				self.read_colorrgbexp32(), self.read_colorrgbexp32(),
				self.read_colorrgbexp32(), self.read_colorrgbexp32(),
				self.read_colorrgbexp32(), self.read_colorrgbexp32(),
			]
		}
	}

	pub fn read_cdispsubneighbor(
		&mut self,
	) -> CDispSubNeighbor {
		CDispSubNeighbor {
			neighbor: self.read_ushort(),
			neighbor_orientation: self.read_byte(),
			span: self.read_byte(),
			neighbor_span: self.read_byte(),
			padding: self.read_byte(),
		}
	}

	pub fn read_cdispneighbor(
		&mut self,
	) -> CDispNeighbor {
		CDispNeighbor {
			sub_neighbors: [
				self.read_cdispsubneighbor(),
				self.read_cdispsubneighbor(),
			],
		}
	}

	pub fn read_cdispcornerneighbor(
		&mut self,
	) -> CDispCornerNeighbors {
		CDispCornerNeighbors {
			neighbors: [
				self.read_ushort(), self.read_ushort(),
				self.read_ushort(), self.read_ushort(),
			],
			num_neighbors: self.read_byte(),
			padding: self.read_byte(),
		}
	}
}

// returns index of first found null byte in slice
// errors out if byte not found
fn find_next_null_byte(
	bytes: &[u8],
) -> usize {
	bytes.iter()
	.position(|x| *x == 0)
	.unwrap()
}