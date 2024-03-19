use bitflags::bitflags;

bitflags! {
	#[derive(Debug, Clone, Copy)]
	pub struct ContentsFlags : u32 {
		const Empty              = 0x0;
		const Solid              = 0x1;
		const Window             = 0x2;
		const Aux                = 0x4;
		const Grate              = 0x8;
		const Slime              = 0x10;
		const Water              = 0x20;
		const Mist               = 0x40;
		const Opaque             = 0x80;
		const TestFogVolume      = 0x100;
		const Unused             = 0x200;
		const Unused6            = 0x400;
		const Team1              = 0x800;
		const Team2              = 0x1000;
		const IgnoreNodrawOpaque = 0x2000;
		const Moveable           = 0x4000;
		const AreaPortal         = 0x8000;
		const PlayerClip         = 0x10000;
		const MonsterClip        = 0x20000;
		const Current0           = 0x40000;
		const Current90          = 0x80000;
		const Current180         = 0x100000;
		const Current270         = 0x200000;
		const CurrentUp          = 0x400000;
		const CurrentDown        = 0x800000;
		const Origin             = 0x1000000;
		const Monster            = 0x2000000;
		const Debris             = 0x4000000;
		const Detail             = 0x8000000;
		const Translucent        = 0x10000000;
		const Ladder             = 0x20000000;
		const Hitbox             = 0x80000000;
	}	
}

bitflags! {
	#[derive(Debug, Clone, Copy)]
	pub struct SurfaceFlags : u32 {
		const Light     = 0x1;
		const Sky2D     = 0x2;
		const Sky       = 0x4;
		const Warp      = 0x8;
		const Trans     = 0x10;
		const NoPortal  = 0x20;
		const Trigger   = 0x40;
		const Nodraw    = 0x80;
		const Hint      = 0x100;
		const Skip      = 0x200;
		const NoLight   = 0x400;
		const BumpLight = 0x800;
		const NoShadows = 0x1000;
		const NoDecals  = 0x2000;
		const NoChop    = 0x4000;
		const Hitbox    = 0x8000;
	}
}

bitflags! {
	#[derive(Debug, Clone, Copy)]
	pub struct DispTriFlags : u16 {
		const TagSurface    = 0x1;
		const TagWalkable   = 0x2;
		const TagBuildable  = 0x4;
		const FlagSurfprop1 = 0x8;
		const FlagSurfprop2 = 0x10;
	}
}