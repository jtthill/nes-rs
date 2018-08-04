use interconnect::Interconnect;

#[derive(Default, Debug)]
pub struct Cpu {
    //Accumulator
    reg_a: u8,
    reg_x: u8,
    reg_y: u8,

    reg_pc: u16,
    reg_s: u8,

    // Status flag
    // 7:   Negative
    // 6:   Overflow
    // 5-4: Break
    // 3:   Decimal(unused)
    // 2:   Interrupt
    // 1:   Zero
    // 0:   Carry
    reg_p: u8,

    opcode: u8,
}

#[derive(Debug)]
enum AddressingMode {
    Implicit,
    Accumulator,
    Immediate,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Relative,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Indirect,
    IndexedIndirect,
    IndirectIndexed,
}

impl Cpu {
    pub fn new() -> Self {
        let mut cpu = Cpu::default();
        cpu.reg_s = 0xFD;
        cpu.reg_p = 0x34;
        cpu.reg_pc = 0xFFFC; // Reset vector
        cpu
    }

    pub fn step(&mut self, inter: &mut Interconnect) -> u32 {
        // Do one instruction. (Possibly micro-ops?) Return the number
        // of cycles that were run in the emulation of the instruction
        self.opcode = inter.read_byte(self.reg_pc);
        self.reg_pc += 1;

        match self.opcode {
            0x69 => self.adc(inter, AddressingMode::Immediate),
            0x65 => self.adc(inter, AddressingMode::ZeroPage),
            0x75 => self.adc(inter, AddressingMode::ZeroPageX),
            0x6D => self.adc(inter, AddressingMode::Absolute),
            0x7D => self.adc(inter, AddressingMode::AbsoluteX),
            0x79 => self.adc(inter, AddressingMode::AbsoluteY),
            0x61 => self.adc(inter, AddressingMode::IndexedIndirect),
            0x71 => self.adc(inter, AddressingMode::IndirectIndexed),
            0x29 => self.and(inter, AddressingMode::Immediate),
            0x25 => self.and(inter, AddressingMode::ZeroPage),
            0x35 => self.and(inter, AddressingMode::ZeroPageX),
            0x2D => self.and(inter, AddressingMode::Absolute),
            0x3D => self.and(inter, AddressingMode::AbsoluteX),
            0x39 => self.and(inter, AddressingMode::AbsoluteY),
            0x21 => self.and(inter, AddressingMode::IndexedIndirect),
            0x31 => self.and(inter, AddressingMode::IndirectIndexed),
            0x0A => self.asl(inter, AddressingMode::Accumulator),
            0x06 => self.asl(inter, AddressingMode::ZeroPage),
            0x16 => self.asl(inter, AddressingMode::ZeroPageX),
            0x0E => self.asl(inter, AddressingMode::Absolute),
            0x1E => self.asl(inter, AddressingMode::AbsoluteX),
            0x90 => self.bcc(inter, AddressingMode::Relative),
            0xB0 => self.bcs(inter, AddressingMode::Relative),
            0xF0 => self.beq(inter, AddressingMode::Relative),
            0x24 => self.bit(inter, AddressingMode::ZeroPage),
            0x2C => self.bit(inter, AddressingMode::Absolute),
            0x30 => self.bmi(inter, AddressingMode::Relative),
            0xD0 => self.bne(inter, AddressingMode::Relative),
            0x10 => self.bpl(inter, AddressingMode::Relative),
            0x00 => self.brk(inter, AddressingMode::Implicit),
            0x50 => self.bvc(inter, AddressingMode::Relative),
            0x70 => self.bvs(inter, AddressingMode::Relative),
            0x18 => self.clc(inter, AddressingMode::Implicit),
            0xD8 => self.cld(inter, AddressingMode::Implicit),
            0x58 => self.cli(inter, AddressingMode::Implicit),
            0xB8 => self.clv(inter, AddressingMode::Implicit),
            0xC9 => self.cmp(inter, AddressingMode::Immediate),
            0xC5 => self.cmp(inter, AddressingMode::ZeroPage),
            0xD5 => self.cmp(inter, AddressingMode::ZeroPageX),
            0xCD => self.cmp(inter, AddressingMode::Absolute),
            0xDD => self.cmp(inter, AddressingMode::AbsoluteX),
            0xD9 => self.cmp(inter, AddressingMode::AbsoluteY),
            0xC1 => self.cmp(inter, AddressingMode::IndexedIndirect),
            0xD1 => self.cmp(inter, AddressingMode::IndirectIndexed),
            0xE0 => self.cpx(inter, AddressingMode::Immediate),
            0xE4 => self.cpx(inter, AddressingMode::ZeroPage),
            0xEC => self.cpx(inter, AddressingMode::Absolute),
            0xC0 => self.cpy(inter, AddressingMode::Immediate),
            0xC4 => self.cpy(inter, AddressingMode::ZeroPage),
            0xCC => self.cpy(inter, AddressingMode::Absolute),
            0xC6 => self.dec(inter, AddressingMode::ZeroPage),
            0xD6 => self.dec(inter, AddressingMode::ZeroPageX),
            0xCE => self.dec(inter, AddressingMode::Absolute),
            0xDE => self.dec(inter, AddressingMode::AbsoluteX),
            0xCA => self.dex(inter, AddressingMode::Implicit),
            0x88 => self.dey(inter, AddressingMode::Implicit),
            0x49 => self.eor(inter, AddressingMode::Immediate),
            0x45 => self.eor(inter, AddressingMode::ZeroPage),
            0x55 => self.eor(inter, AddressingMode::ZeroPageX),
            0x4D => self.eor(inter, AddressingMode::Absolute),
            0x5D => self.eor(inter, AddressingMode::AbsoluteX),
            0x59 => self.eor(inter, AddressingMode::AbsoluteY),
            0x41 => self.eor(inter, AddressingMode::IndexedIndirect),
            0x51 => self.eor(inter, AddressingMode::IndirectIndexed),
            0xE6 => self.inc(inter, AddressingMode::ZeroPage),
            0xF6 => self.inc(inter, AddressingMode::ZeroPageX),
            0xEE => self.inc(inter, AddressingMode::Absolute),
            0xFE => self.inc(inter, AddressingMode::AbsoluteX),
            0xE8 => self.inx(inter, AddressingMode::Implicit),
            0xC8 => self.iny(inter, AddressingMode::Implicit),
            0x4C => self.jmp(inter, AddressingMode::Absolute),
            0x6C => self.jmp(inter, AddressingMode::Indirect),
            0x20 => self.jsr(inter, AddressingMode::Absolute),
            0xA9 => self.lda(inter, AddressingMode::Immediate),
            0xA5 => self.lda(inter, AddressingMode::ZeroPage),
            0xB5 => self.lda(inter, AddressingMode::ZeroPageX),
            0xAD => self.lda(inter, AddressingMode::Absolute),
            0xBD => self.lda(inter, AddressingMode::AbsoluteX),
            0xB9 => self.lda(inter, AddressingMode::AbsoluteY),
            0xA1 => self.lda(inter, AddressingMode::IndexedIndirect),
            0xB1 => self.lda(inter, AddressingMode::IndirectIndexed),
            0xA2 => self.ldx(inter, AddressingMode::Immediate),
            0xA6 => self.ldx(inter, AddressingMode::ZeroPage),
            0xB6 => self.ldx(inter, AddressingMode::ZeroPageY),
            0xAE => self.ldx(inter, AddressingMode::Absolute),
            0xBE => self.ldx(inter, AddressingMode::AbsoluteY),
            0xA0 => self.ldy(inter, AddressingMode::Immediate),
            0xA4 => self.ldy(inter, AddressingMode::ZeroPage),
            0xB4 => self.ldy(inter, AddressingMode::ZeroPageX),
            0xAC => self.ldy(inter, AddressingMode::Absolute),
            0xBC => self.ldy(inter, AddressingMode::AbsoluteX),
            0x4A => self.lsr(inter, AddressingMode::Accumulator),
            0x46 => self.lsr(inter, AddressingMode::ZeroPage),
            0x56 => self.lsr(inter, AddressingMode::ZeroPageX),
            0x4E => self.lsr(inter, AddressingMode::Absolute),
            0x5E => self.lsr(inter, AddressingMode::AbsoluteX),
            0xEA => self.nop(inter, AddressingMode::Implicit),
            0x09 => self.ora(inter, AddressingMode::Immediate),
            0x05 => self.ora(inter, AddressingMode::ZeroPage),
            0x15 => self.ora(inter, AddressingMode::ZeroPageX),
            0x0D => self.ora(inter, AddressingMode::Absolute),
            0x1D => self.ora(inter, AddressingMode::AbsoluteX),
            0x19 => self.ora(inter, AddressingMode::AbsoluteY),
            0x01 => self.ora(inter, AddressingMode::IndexedIndirect),
            0x11 => self.ora(inter, AddressingMode::IndirectIndexed),
            0x48 => self.pha(inter, AddressingMode::Implicit),
            0x08 => self.php(inter, AddressingMode::Implicit),
            0x68 => self.pla(inter, AddressingMode::Implicit),
            0x28 => self.plp(inter, AddressingMode::Implicit),
            0x2A => self.rol(inter, AddressingMode::Accumulator),
            0x26 => self.rol(inter, AddressingMode::ZeroPage),
            0x36 => self.rol(inter, AddressingMode::ZeroPageX),
            0x2E => self.rol(inter, AddressingMode::Absolute),
            0x3E => self.rol(inter, AddressingMode::AbsoluteX),
            0x6A => self.ror(inter, AddressingMode::Accumulator),
            0x66 => self.ror(inter, AddressingMode::ZeroPage),
            0x76 => self.ror(inter, AddressingMode::ZeroPageX),
            0x6E => self.ror(inter, AddressingMode::Absolute),
            0x7E => self.ror(inter, AddressingMode::AbsoluteX),
            0x40 => self.rti(inter, AddressingMode::Implicit),
            0x60 => self.rts(inter, AddressingMode::Implicit),
            0xE9 => self.sbc(inter, AddressingMode::Immediate),
            0xE5 => self.sbc(inter, AddressingMode::ZeroPage),
            0xF5 => self.sbc(inter, AddressingMode::ZeroPageX),
            0xED => self.sbc(inter, AddressingMode::Absolute),
            0xFD => self.sbc(inter, AddressingMode::AbsoluteX),
            0xF9 => self.sbc(inter, AddressingMode::AbsoluteY),
            0xE1 => self.sbc(inter, AddressingMode::IndexedIndirect),
            0xF1 => self.sbc(inter, AddressingMode::IndirectIndexed),
            0x38 => self.sec(inter, AddressingMode::Implicit),
            0xF8 => self.sed(inter, AddressingMode::Implicit),
            0x78 => self.sei(inter, AddressingMode::Implicit),
            0x85 => self.sta(inter, AddressingMode::ZeroPage),
            0x95 => self.sta(inter, AddressingMode::ZeroPageX),
            0x8D => self.sta(inter, AddressingMode::Absolute),
            0x9D => self.sta(inter, AddressingMode::AbsoluteX),
            0x99 => self.sta(inter, AddressingMode::AbsoluteY),
            0x81 => self.sta(inter, AddressingMode::IndexedIndirect),
            0x91 => self.sta(inter, AddressingMode::IndirectIndexed),
            0x86 => self.stx(inter, AddressingMode::ZeroPage),
            0x96 => self.stx(inter, AddressingMode::ZeroPageY),
            0x8E => self.stx(inter, AddressingMode::Absolute),
            0x84 => self.sty(inter, AddressingMode::ZeroPage),
            0x94 => self.sty(inter, AddressingMode::ZeroPageX),
            0x8C => self.sty(inter, AddressingMode::Absolute),
            0xAA => self.tax(inter, AddressingMode::Implicit),
            0xA8 => self.tay(inter, AddressingMode::Implicit),
            0xBA => self.tsx(inter, AddressingMode::Implicit),
            0x8A => self.txa(inter, AddressingMode::Implicit),
            0x9A => self.txs(inter, AddressingMode::Implicit),
            0x98 => self.tya(inter, AddressingMode::Implicit),
            _ => panic!("Unrecognized opcode."),
        };
        0
    }
    // ADC - Add with Carry
    fn adc(&mut self, inter: &mut Interconnect, mode: AddressingMode) -> u32 {
        let mut cycles;
        let mut mem_val;
        println!("[0x{:X}] ADC using Addressing Mode {:?}", self.opcode, mode);
        match mode {
            AddressingMode::Immediate => {
                mem_val = inter.read_byte(self.reg_pc);
                self.reg_pc += 1;
                cycles = 2;
            }
            AddressingMode::ZeroPage => {
                let mem_loc = inter.read_byte(self.reg_pc);
                mem_val = inter.read_byte(mem_loc as u16);
                self.reg_pc += 1;
                cycles = 3;
            }
            AddressingMode::ZeroPageX => {
                let mem_loc = inter.read_byte(self.reg_pc);
                mem_val = inter.read_byte(mem_loc as u16 + self.reg_x as u16);
                self.reg_pc += 1;
                cycles = 4;
            }
            AddressingMode::Absolute => {
                let mem_lo = inter.read_byte(self.reg_pc);
                self.reg_pc += 1;
                let mem_hi = inter.read_byte(self.reg_pc);
                self.reg_pc += 1;
                mem_val = inter.read_byte(((mem_hi << 4) | mem_lo) as u16);
                cycles = 4;
            }
            AddressingMode::AbsoluteX => {
                let mem_lo = inter.read_byte(self.reg_pc);
                self.reg_pc += 1;
                let mem_hi = inter.read_byte(self.reg_pc);
                self.reg_pc += 1;
                mem_val = inter.read_byte(((mem_hi << 4) | mem_lo) as u16 + self.reg_x as u16);
                cycles = 4;
            }

            _ => panic!("Unrecognized addressing mode in ADC."),
        };
        cycles
    }
    // AND - Logical AND
    fn and(&mut self, inter: &mut Interconnect, mode: AddressingMode) -> u32 {
        0
    }
    // ASL - Arithmetic Shift Left
    fn asl(&mut self, inter: &mut Interconnect, mode: AddressingMode) -> u32 {
        0
    }
    // BCC - Branch if Carry Clear
    fn bcc(&mut self, inter: &mut Interconnect, mode: AddressingMode) -> u32 {
        0
    }
    // BCS - Branch if Carry Set
    fn bcs(&mut self, inter: &mut Interconnect, mode: AddressingMode) -> u32 {
        0
    }
    // BEQ - Branch if Equal
    fn beq(&mut self, inter: &mut Interconnect, mode: AddressingMode) -> u32 {
        0
    }
    // BIT - Bit Test
    fn bit(&mut self, inter: &mut Interconnect, mode: AddressingMode) -> u32 {
        0
    }
    // BMI - Branch if Minus
    fn bmi(&mut self, inter: &mut Interconnect, mode: AddressingMode) -> u32 {
        0
    }
    // BNE - Branch if Not Equal
    fn bne(&mut self, inter: &mut Interconnect, mode: AddressingMode) -> u32 {
        0
    }
    // BPL - Branch if Positive
    fn bpl(&mut self, inter: &mut Interconnect, mode: AddressingMode) -> u32 {
        0
    }
    // BRK - Force Interrupt
    fn brk(&mut self, inter: &mut Interconnect, mode: AddressingMode) -> u32 {
        0
    }
    // BVC - Branch if Overflow Clear
    fn bvc(&mut self, inter: &mut Interconnect, mode: AddressingMode) -> u32 {
        0
    }
    // BVS - Branch if Overflow Set
    fn bvs(&mut self, inter: &mut Interconnect, mode: AddressingMode) -> u32 {
        0
    }
    // CLC - Clear Carry Flag
    fn clc(&mut self, inter: &mut Interconnect, mode: AddressingMode) -> u32 {
        0
    }
    // CLD - Clear Decimal Mode
    fn cld(&mut self, inter: &mut Interconnect, mode: AddressingMode) -> u32 {
        0
    }
    // CLI - Clear Interrupt Disable
    fn cli(&mut self, inter: &mut Interconnect, mode: AddressingMode) -> u32 {
        0
    }
    // CLV - Clear Overflow Flag
    fn clv(&mut self, inter: &mut Interconnect, mode: AddressingMode) -> u32 {
        0
    }
    // CMP - Compare
    fn cmp(&mut self, inter: &mut Interconnect, mode: AddressingMode) -> u32 {
        0
    }
    // CPX - Compare X Register
    fn cpx(&mut self, inter: &mut Interconnect, mode: AddressingMode) -> u32 {
        0
    }
    // CPY - Compare Y Register
    fn cpy(&mut self, inter: &mut Interconnect, mode: AddressingMode) -> u32 {
        0
    }
    // DEC - Decrement Memory
    fn dec(&mut self, inter: &mut Interconnect, mode: AddressingMode) -> u32 {
        0
    }
    // DEX - Decrement X Register
    fn dex(&mut self, inter: &mut Interconnect, mode: AddressingMode) -> u32 {
        0
    }
    // DEY - Decrement Y Register
    fn dey(&mut self, inter: &mut Interconnect, mode: AddressingMode) -> u32 {
        0
    }
    // EOR - Exclusive OR
    fn eor(&mut self, inter: &mut Interconnect, mode: AddressingMode) -> u32 {
        0
    }
    // INC - Increment Memory
    fn inc(&mut self, inter: &mut Interconnect, mode: AddressingMode) -> u32 {
        0
    }
    // INX - Increment X Register
    fn inx(&mut self, inter: &mut Interconnect, mode: AddressingMode) -> u32 {
        0
    }
    // INY - Increment Y Register
    fn iny(&mut self, inter: &mut Interconnect, mode: AddressingMode) -> u32 {
        0
    }
    // JMP - Jump
    fn jmp(&mut self, inter: &mut Interconnect, mode: AddressingMode) -> u32 {
        0
    }
    // JSR - Jump to Subroutine
    fn jsr(&mut self, inter: &mut Interconnect, mode: AddressingMode) -> u32 {
        0
    }
    // LDA - Load Accumulator
    fn lda(&mut self, inter: &mut Interconnect, mode: AddressingMode) -> u32 {
        0
    }
    // LDX - Load X Register
    fn ldx(&mut self, inter: &mut Interconnect, mode: AddressingMode) -> u32 {
        0
    }
    // LDY - Load Y Register
    fn ldy(&mut self, inter: &mut Interconnect, mode: AddressingMode) -> u32 {
        0
    }
    // LSR - Logical Shift Right
    fn lsr(&mut self, inter: &mut Interconnect, mode: AddressingMode) -> u32 {
        0
    }
    // NOP - No Operation
    fn nop(&mut self, inter: &mut Interconnect, mode: AddressingMode) -> u32 {
        0
    }
    // ORA - Logical Inclusive OR
    fn ora(&mut self, inter: &mut Interconnect, mode: AddressingMode) -> u32 {
        0
    }
    // PHA - Push Accumulator
    fn pha(&mut self, inter: &mut Interconnect, mode: AddressingMode) -> u32 {
        0
    }
    // PHP - Push Processor Status
    fn php(&mut self, inter: &mut Interconnect, mode: AddressingMode) -> u32 {
        0
    }
    // PLA - Pull Accumulator
    fn pla(&mut self, inter: &mut Interconnect, mode: AddressingMode) -> u32 {
        0
    }
    // PLP - Pull Processor Status
    fn plp(&mut self, inter: &mut Interconnect, mode: AddressingMode) -> u32 {
        0
    }
    // ROL - Rotate Left
    fn rol(&mut self, inter: &mut Interconnect, mode: AddressingMode) -> u32 {
        0
    }
    // ROR - Rotate Right
    fn ror(&mut self, inter: &mut Interconnect, mode: AddressingMode) -> u32 {
        0
    }
    // RTI - Return from Interrupt
    fn rti(&mut self, inter: &mut Interconnect, mode: AddressingMode) -> u32 {
        0
    }
    // RTS - Return from Subroutine
    fn rts(&mut self, inter: &mut Interconnect, mode: AddressingMode) -> u32 {
        0
    }
    // SBC - Subtract with Carry
    fn sbc(&mut self, inter: &mut Interconnect, mode: AddressingMode) -> u32 {
        0
    }
    // SEC - Set Carry Flag
    fn sec(&mut self, inter: &mut Interconnect, mode: AddressingMode) -> u32 {
        0
    }
    // SED - Set Decimal Flag
    fn sed(&mut self, inter: &mut Interconnect, mode: AddressingMode) -> u32 {
        0
    }
    // SEI - Set Interrupt Disable
    fn sei(&mut self, inter: &mut Interconnect, mode: AddressingMode) -> u32 {
        0
    }
    // STA - Store Accumulator
    fn sta(&mut self, inter: &mut Interconnect, mode: AddressingMode) -> u32 {
        0
    }
    // STX - Store X Register
    fn stx(&mut self, inter: &mut Interconnect, mode: AddressingMode) -> u32 {
        0
    }
    // STY - Store Y Register
    fn sty(&mut self, inter: &mut Interconnect, mode: AddressingMode) -> u32 {
        0
    }
    // TAX - Transfer Accumulator to X
    fn tax(&mut self, inter: &mut Interconnect, mode: AddressingMode) -> u32 {
        0
    }
    // TAY - Transfer Accumulator to Y
    fn tay(&mut self, inter: &mut Interconnect, mode: AddressingMode) -> u32 {
        0
    }
    // TSX - Transfer Stack POinter to X
    fn tsx(&mut self, inter: &mut Interconnect, mode: AddressingMode) -> u32 {
        0
    }
    // TXA - Transfer X to Accumulator
    fn txa(&mut self, inter: &mut Interconnect, mode: AddressingMode) -> u32 {
        0
    }
    // TXS - Transfer X to Stack Pointer
    fn txs(&mut self, inter: &mut Interconnect, mode: AddressingMode) -> u32 {
        0
    }
    // TYA - Transfer Y to Accumulator
    fn tya(&mut self, inter: &mut Interconnect, mode: AddressingMode) -> u32 {
        0
    }
}
