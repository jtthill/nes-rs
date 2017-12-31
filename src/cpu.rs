#[derive(Default, Debug)]
pub struct Cpu {
    //Accumulator
    reg_a: u8,
    reg_x: u8,
    reg_y: u8,

    reg_pc: u16,
    reg_s: u8,

    //Status flag
    reg_p: u8,
}

impl Cpu {
    pub fn new() -> Self {
        let mut cpu = Cpu::default();
        cpu.reg_s = 0xFD;
        cpu.reg_p = 0x34;
		cpu
    }
}
