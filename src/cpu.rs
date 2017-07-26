#[derive(Default, Debug)]
pub struct Cpu {
    //Accumulator
    reg_a: u8,
    reg_x: u8,
    reg_y: u8,

    reg_pc: u16,
    reg_s: u8,

    //Status flag
    reg_p: u8
}

impl Cpu {
    pub fn new() -> Self {
        Cpu::default()
    }

    pub fn power_on_reset(&mut self) {
        self.reg_a = 0x0;
        self.reg_x = 0x0;
        self.reg_y = 0x0;
        self.reg_s = 0xFD;
        self.reg_p = 0x34;
    }
}