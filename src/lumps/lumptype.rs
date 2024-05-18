use crate::lumps::vbsp::VBSPLumpType;
use crate::lumps::goldsrc::GoldSrcLumpType;
use crate::lumps::quake::QuakeLumpType;

#[derive(Debug, Clone)]
pub enum Lumps {
	VBSP(Vec<VBSPLumpType>),
	GoldSrc(Vec<GoldSrcLumpType>),
	Quake(Vec<QuakeLumpType>)
}