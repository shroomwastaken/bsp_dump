use crate::lumps::vbsp::VBSPLumpType;
use crate::lumps::goldsrc::GoldSrcLumpType;

#[derive(Debug, Clone)]
pub enum Lumps {
	VBSP(Vec<VBSPLumpType>),
	GoldSrc(Vec<GoldSrcLumpType>)
}