// RAM
pub const INTERNAL_RAM_START: u16 = 0x0000;
pub const INTERNAL_RAM_SIZE: u16 = 0x0800;
pub const INTERNAL_RAM_END: u16 = (INTERNAL_RAM_SIZE*4) + INTERNAL_RAM_START - 1;
pub const INTERNAL_RAM_MIRROR_1_START: u16 = 0x0800;
pub const INTERNAL_RAM_MIRROR_2_START: u16 = 0x1000;
pub const INTERNAL_RAM_MIRROR_3_START: u16 = 0x1800;
pub const INTERNAL_RAM_MIRROR_SIZE: u16 = INTERNAL_RAM_SIZE;

// PPU Regs
pub const PPU_CTRL_REG: u16 = 0x2000;
pub const PPU_MASK_REG: u16 = 0x2001;
pub const PPU_STATUS_REG: u16 = 0x2002;
pub const OAM_ADDR_REG: u16 = 0x2003;
pub const OAM_DATA_REG: u16 = 0x2004;
pub const PPU_SCROLL_REG: u16 = 0x2005;
pub const PPU_ADDR_REG: u16 = 0x2006;
pub const PPU_DATA_REG: u16 = 0x2007;
pub const PPU_MIRROR_END: u16 = 0x3FFF;

// APU
pub const APU_SQ1_VOL: u16 = 0x4000;
pub const APU_SQ1_SWEEP: u16 = 0x4001;
pub const APU_SQ1_LO: u16 = 0x4002;
pub const APU_SQ1_HI: u16 = 0x4003;
pub const APU_SQ2_VOL: u16 = 0x4004;
pub const APU_SQ2_SWEEP: u16 = 0x4005;
pub const APU_SQ2_LO: u16 = 0x4006;
pub const APU_SQ2_HI: u16 = 0x4007;
pub const APU_TRI_LINEAR: u16 = 0x4008;
pub const APU_TRI_LO: u16 = 0x400A;
pub const APU_TRI_HI: u16 = 0x400B;
pub const APU_NOISE_VOL: u16 = 0x400C;
pub const APU_NOISE_LO: u16 = 0x400E;
pub const APU_NOISE_HI: u16 = 0x400F;
pub const APU_DMC_FREQ: u16 = 0x4010;
pub const APU_DMC_RAW: u16 = 0x4011;
pub const APU_DMC_START: u16 = 0x4012;
pub const APU_DMC_LEN: u16 = 0x4013;

// I/O
pub const OAM_DMA_REG: u16 = 0x4014;
pub const APU_SND_CHN: u16 = 0x4015;
pub const GAMEPAD_JOY_1: u16 = 0x4016;
pub const GAMEPAD_JOY_2: u16 = 0x4017;

// Mapper
pub const GAMEPAK_MEM_START: u16 = 0x4020;
pub const GAMEPAK_MEM_SIZE: u16 = 0xBFE0;
pub const GAMEPAK_MEM_END: u16 = GAMEPAK_MEM_START + (GAMEPAK_MEM_SIZE - 1);