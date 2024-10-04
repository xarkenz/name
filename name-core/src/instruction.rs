use crate::arguments::{IArgs, JArgs, RArgs};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum OpFunct {
    OpCode(u32),
    Funct(u32),
}

impl OpFunct {
    pub fn opcode(self) -> Result<u32, &'static str> {
        match self {
            OpFunct::OpCode(x) => Ok(x),
            OpFunct::Funct(_) => Err("Expected opcode got funct"),
        }
    }
    pub fn funct(self) -> Result<u32, &'static str> {
        match self {
            OpFunct::Funct(x) => Ok(x),
            OpFunct::OpCode(_) => Err("Expected funct got opcode"),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct RawInstruction {
    pub raw: u32,
}

impl RawInstruction {
    pub fn new(raw: u32) -> RawInstruction {
        RawInstruction { raw }
    }

    pub fn opt_funct(self) -> OpFunct {
        let opcode = self.raw >> 26;
        if opcode == 0 {
            let funct = self.raw & 0x3F;
            OpFunct::Funct(funct)
        } else {
            OpFunct::OpCode(opcode)
        }
    }
}

#[derive(Debug, Clone)]
pub enum Instruction {
    Add(RArgs),
    Addi(IArgs),
    Addiu(IArgs),
    Addu(RArgs),
    And(RArgs),
    Andi(IArgs),
    Beq(IArgs),
    Bgtz(IArgs),
    Blez(IArgs),
    Bne(IArgs),
    J(JArgs),
    Jal(JArgs),
    Jalr(RArgs),
    Jr(RArgs),
    Lb(IArgs),
    Lui(IArgs),
    Lw(IArgs),
    Nor(RArgs),
    Nop(RArgs),
    Or(RArgs),
    Ori(IArgs),
    Sb(IArgs),
    Sll(RArgs),
    Slt(RArgs),
    Slti(IArgs),
    Sw(IArgs),
    Sltiu(IArgs),
    Sltu(RArgs),
    Srl(RArgs),
    Sub(RArgs),
    Subu(RArgs),
    Syscall(RArgs),
    Xor(RArgs),
    Xori(IArgs),
}

impl Instruction {
    pub fn from_raw_instruction(raw_instr: RawInstruction) -> Option<Instruction> {
        match raw_instr.opt_funct() {
            OpFunct::Funct(func) => {
                /*

                  ______ _    _ _   _  _____ _______
                 |  ____| |  | | \ | |/ ____|__   __|
                 | |__  | |  | |  \| | |       | |
                 |  __| | |  | | . ` | |       | |
                 | |    | |__| | |\  | |____   | |
                 |_|     \____/|_| \_|\_____|  |_|



                */
                let rargs = RArgs::unpack(raw_instr);
                match func {
                    0x00 => Some(Instruction::Sll(rargs)),
                    0x01 => None,
                    0x02 => Some(Instruction::Srl(rargs)),
                    0x03 => None,
                    0x04 => None,
                    0x05 => None,
                    0x06 => None,
                    0x07 => None,
                    0x08 => Some(Instruction::Jr(rargs)),
                    0x09 => Some(Instruction::Jalr(rargs)),
                    0x0A => None,
                    0x0B => None,
                    0x0C => Some(Instruction::Syscall(rargs)),
                    0x0D => None,
                    0x0E => None,
                    0x0F => None,
                    0x10 => None,
                    0x11 => None,
                    0x12 => None,
                    0x13 => None,
                    0x14 => None,
                    0x15 => None,
                    0x16 => None,
                    0x17 => None,
                    0x18 => None,
                    0x19 => None,
                    0x1A => None,
                    0x1B => None,
                    0x1C => None,
                    0x1D => None,
                    0x1E => None,
                    0x1F => None,
                    0x20 => Some(Instruction::Add(rargs)),
                    0x21 => Some(Instruction::Addu(rargs)),
                    0x22 => Some(Instruction::Sub(rargs)),
                    0x23 => Some(Instruction::Subu(rargs)),
                    0x24 => Some(Instruction::And(rargs)),
                    0x25 => Some(Instruction::Or(rargs)),
                    0x26 => Some(Instruction::Xor(rargs)),
                    0x27 => Some(Instruction::Nor(rargs)),
                    0x28 => None,
                    0x29 => None,
                    0x2A => Some(Instruction::Slt(rargs)),
                    0x2B => Some(Instruction::Sltu(rargs)),
                    0x2C => None,
                    0x2D => None,
                    0x2E => None,
                    0x2F => None,
                    0x30 => None,
                    0x31 => None,
                    0x32 => None,
                    0x33 => None,
                    0x34 => None,
                    0x35 => None,
                    0x36 => None,
                    0x37 => None,
                    0x38 => None,
                    0x39 => None,
                    0x3A => None,
                    0x3B => None,
                    0x3C => None,
                    0x3D => None,
                    0x3E => None,
                    0x3F => None,
                    _ => None,
                }
            }
            /*

               ____  _____   _____ ____  _____  ______
              / __ \|  __ \ / ____/ __ \|  __ \|  ____|
             | |  | | |__) | |   | |  | | |  | | |__
             | |  | |  ___/| |   | |  | | |  | |  __|
             | |__| | |    | |___| |__| | |__| | |____
              \____/|_|     \_____\____/|_____/|______|



            */
            OpFunct::OpCode(op) => match op {
                0x00 => None,
                0x01 => None,
                0x02 => Some(Instruction::J(JArgs::unpack(raw_instr))),
                0x03 => Some(Instruction::Jal(JArgs::unpack(raw_instr))),
                0x04 => Some(Instruction::Beq(IArgs::unpack(raw_instr))),
                0x05 => Some(Instruction::Bne(IArgs::unpack(raw_instr))),
                0x06 => Some(Instruction::Blez(IArgs::unpack(raw_instr))),
                0x07 => Some(Instruction::Bgtz(IArgs::unpack(raw_instr))),
                0x08 => Some(Instruction::Addi(IArgs::unpack(raw_instr))),
                0x09 => Some(Instruction::Addiu(IArgs::unpack(raw_instr))),
                0x0A => Some(Instruction::Slti(IArgs::unpack(raw_instr))),
                0x0B => Some(Instruction::Sltiu(IArgs::unpack(raw_instr))),
                0x0C => Some(Instruction::Andi(IArgs::unpack(raw_instr))),
                0x0D => Some(Instruction::Ori(IArgs::unpack(raw_instr))),
                0x0E => Some(Instruction::Xori(IArgs::unpack(raw_instr))),
                0x0F => Some(Instruction::Lui(IArgs::unpack(raw_instr))),
                0x10 => None,
                0x11 => None,
                0x12 => None,
                0x13 => None,
                0x14 => None,
                0x15 => None,
                0x16 => None,
                0x17 => None,
                0x18 => None,
                0x19 => None,
                0x1A => None,
                0x1B => None,
                0x1C => None,
                0x1D => None,
                0x1E => None,
                0x1F => None,
                0x20 => Some(Instruction::Lb(IArgs::unpack(raw_instr))),
                0x21 => None,
                0x22 => None,
                0x23 => Some(Instruction::Lw(IArgs::unpack(raw_instr))),
                0x24 => None,
                0x25 => None,
                0x26 => None,
                0x27 => None,
                0x28 => Some(Instruction::Sb(IArgs::unpack(raw_instr))),
                0x29 => None,
                0x2A => None,
                0x2B => Some(Instruction::Sw(IArgs::unpack(raw_instr))),
                0x2C => None,
                0x2D => None,
                0x2E => None,
                0x2F => None,
                0x30 => None,
                0x31 => None,
                0x32 => None,
                0x33 => None,
                0x34 => None,
                0x35 => None,
                0x36 => None,
                0x37 => None,
                0x38 => None,
                0x39 => None,
                0x3A => None,
                0x3B => None,
                0x3C => None,
                0x3D => None,
                0x3E => None,
                0x3F => None,
                _ => None,
            },
        }
    }
}
