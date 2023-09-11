struct Mips {
    regs: [u32; 32],
    floats: [f32; 32],
    mult_hi: u32,
    mult_lo: u32,
    pc: usize,

    // Enough for six MIPS instructions.
    memory: [u32; 13]
}

struct Rtype {
    rs: usize,
    rt: usize,
    rd: usize,
    shamt: u8,
    funct: u8
}

struct Itype {
    opcode: u32,
    rs: usize,
    rt: usize,
    imm: u16
}

// struct Jtype
// struct Ftype

enum Instructions {
    R(Rtype),
    I(Itype),
    //J and F type
}

impl Mips {

    fn dispatch_r(&mut self, ins: Rtype) {
        match ins.funct {
            // Shift-left logical
            0x0 => {
                self.regs[ins.rd] = self.regs[ins.rt] << ins.shamt;
            }
            // Shift-right logical
            0x2 => {
                self.regs[ins.rd] = self.regs[ins.rt] >> ins.shamt;
            }
            // Add
            0x20 => {
                self.regs[ins.rd] = self.regs[ins.rt] + self.regs[ins.rs];
                //Todo- catch overflows
            }
            // Subtract
            0x22 => {
                //Todo- catch overflows
                self.regs[ins.rd] = self.regs[ins.rt] + self.regs[ins.rs];
            }
            // Xor
            0x26 => {
                self.regs[ins.rd] = self.regs[ins.rt] ^ self.regs[ins.rs];
            }
            _ => panic!("R-Type unimplemented instruction")
        }
    }
    fn dispatch_i(&mut self, ins: Itype) {
        match ins.opcode {
            // Or Immediate
            0xD => {
                // Rust zero-extends unsigned values when up-casting
                self.regs[ins.rt] = self.regs[ins.rs] | ins.imm as u32;
            }
            // Load Upper Immediate
            0xF => {
                self.regs[ins.rt] = (ins.imm as u32) << 16;
            }
            _ => panic!("I-type unimplemented instruction")
        }
    }

    fn decode(&self, instruction: u32) -> Instructions {
        let opcode = instruction >> 26 & 0b11111;
        match opcode {
            // R-type
            0 => {
                Instructions::R(Rtype {
                    // These are all five-bit fields
                    rs: (instruction >> 21 & 0b11111) as usize,
                    rd: (instruction >> 16 & 0b11111) as usize,
                    rt: (instruction >> 11 & 0b11111) as usize,
                    shamt: (instruction >> 6 & 0b11111) as u8,
                    // This is a six-bit field
                    funct: (instruction & 0b111111) as u8
                })
            }
            // J-type
            // 0x2 | 0x3 => {
            //     Instructions::Jtype(Jtype {

            //     })
            // }
            // I-type
            _ => {
                Instructions::I(Itype {
                    opcode,
                    rs: (instruction >> 21 & 0b11111) as usize,
                    rt: (instruction >> 16 & 0b11111) as usize,
                    imm: instruction as u16
                })
            }
        }
    }

    fn step_one(&mut self) {
        let opcode = self.memory[self.pc];
        self.pc += 1;
        let instruction = self.decode(opcode);

        match instruction {
            Instructions::R(rtype) => self.dispatch_r(rtype),
            Instructions::I(itype) => self.dispatch_i(itype)
        }
    }
}

fn main() {
    let mut mips = Mips {
        regs: [0; 32],
        floats: [0f32; 32],
        mult_hi: 0,
        mult_lo: 0,
        pc: 0,
        memory: [0x3c080032,
        0x3c090032,
        0x3c0a0032,
        0x3c0b0032,
        0x3c0c0032,
        0x3c0d0032,
        0x3c0e0032,
        0x014b4820, 0x01ae6022, 0x00108140,
        0x0017aa82, 0x03197826, 0x3c080032]
    };
    // The first instructions load a bunch of registers with 0x500000.
    for i in 0..8 {
        mips.step_one();
    }
    /*
    add $t1, $t2, $t3
    sub $t4, $t5, $t6
    sll $s0, $s0, 5
    srl $s5, $s7, 10
    xor $t7, $t8, $t9
    lui $t0, 50
    */
    for i in 0..6 {
        mips.step_one();
    }
}
