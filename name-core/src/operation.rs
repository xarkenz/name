use strum_macros::FromRepr;

#[derive(Debug, Clone)]
pub enum OperationError {
    ReservedInstruction,
    InvalidInstruction,
    WrongInstructionClass,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Operation {
    Immediate(ImmediateOp),
    Special1(Special1Op),
    Special2(Special2Op),
    Regimm(RegimmOp),
}

impl Operation {
    pub fn immediate(&self) -> Result<ImmediateOp, OperationError> {
        match self {
            Operation::Immediate(x) => Ok(*x),
            _ => Err(OperationError::WrongInstructionClass),
        }
    }

    pub fn special1(&self) -> Result<Special1Op, OperationError> {
        match self {
            Operation::Special1(x) => Ok(*x),
            _ => Err(OperationError::WrongInstructionClass),
        }
    }

    pub fn special2(&self) -> Result<Special2Op, OperationError> {
        match self {
            Operation::Special2(x) => Ok(*x),
            _ => Err(OperationError::WrongInstructionClass),
        }
    }

    pub fn regimm(&self) -> Result<RegimmOp, OperationError> {
        match self {
            Operation::Regimm(x) => Ok(*x),
            _ => Err(OperationError::WrongInstructionClass),
        }
    }
}

#[derive(Debug, PartialEq, FromRepr, Clone, Copy)]
#[repr(u32)]
pub enum ImmediateOp {
    // 0x00 = OperationError::WrongInstructionClass,
    // 0x01 = OperationError::WrongInstructionClass,
    J = 0x02,
    Jal = 0x03,
    Beq = 0x04,
    Bne = 0x05,
    Blez = 0x06,
    Bgtz = 0x07,
    Addi = 0x08,
    Addiu = 0x09,
    Slti = 0x0A,
    Sltiu = 0x0B,
    Andi = 0x0C,
    Ori = 0x0D,
    Xori = 0x0E,
    Lui = 0x0F,
    // 0x10 = OperationError::ReservedInstruction,
    // 0x11 = OperationError::ReservedInstruction,
    // 0x12 = OperationError::ReservedInstruction,
    // 0x13 = OperationError::ReservedInstruction,
    Beql = 0x14,
    Bnel = 0x15,
    Blezl = 0x16,
    Bgtzl = 0x17,
    // 0x18 = OperationError::ReservedInstruction,
    // 0x19 = OperationError::ReservedInstruction,
    // 0x1A = OperationError::ReservedInstruction,
    // 0x1B = OperationError::ReservedInstruction,
    // 0x1C = OperationError::WrongInstructionClass
    Jalx = 0x1D,
    // 0x1E = OperationError::ReservedInstruction,
    // 0x1F = OperationError::ReservedInstruction,
    Lb = 0x20,
    Lh = 0x21,
    Lwl = 0x22,
    Lw = 0x23,
    Lbu = 0x24,
    Lhu = 0x25,
    Lwr = 0x26,
    // 0x27 = OperationError::ReservedInstruction,
    Sb = 0x28,
    Sh = 0x29,
    Swl = 0x2A,
    Sw = 0x2B,
    // 0x2C = OperationError::ReservedInstruction,
    // 0x2D = OperationError::ReservedInstruction,
    Swr = 0x2E,
    Cache = 0x2F,
    Ll = 0x30,
    Lwc1 = 0x31,
    Lwc2 = 0x32,
    Pref = 0x33,
    // 0x34 = OperationError::ReservedInstruction,
    Ldc1 = 0x35,
    Ldc2 = 0x36,
    // 0x37 = OperationError::ReservedInstruction,
    Sc = 0x38,
    Swc1 = 0x39,
    Swc2 = 0x3A,
    // 0x3B = OperationError::ReservedInstruction,
    // 0x3C = OperationError::ReservedInstruction,
    Sdc1 = 0x3D,
    Sdc2 = 0x3E,
    // 0x3F = OperationError::ReservedInstruction,
    // _ = OperationError::ReservedInstruction,
}

impl TryFrom<u32> for ImmediateOp {
    type Error = OperationError;
    fn try_from(raw: u32) -> Result<ImmediateOp, OperationError> {
        ImmediateOp::from_repr(raw).ok_or(match raw {
            0x00 | 0x01 | 0x1C => OperationError::WrongInstructionClass,
            0x00..0x40 => OperationError::ReservedInstruction,
            0x40.. => OperationError::InvalidInstruction,
        })
    }
}

#[derive(Debug, PartialEq, Clone, Copy, FromRepr)]
#[repr(u32)]
pub enum Special1Op {
    Sll = 0x00,
    Movci = 0x01,
    Srl = 0x02,
    Sra = 0x03,
    Sllv = 0x04,
    Lsa = 0x05,
    Srlv = 0x06,
    Srav = 0x07,
    Jr = 0x08,
    Jalr = 0x09,
    Movz = 0x0A,
    Movn = 0x0B,
    Syscall = 0x0C,
    Break = 0x0D,
    Sdbbp = 0x0E,
    Sync = 0x0F,
    Mfhi = 0x10,
    Mthi = 0x11,
    Mflo = 0x12,
    Mtlo = 0x13,
    // 0x14 = OperationError::ReservedInstruction,
    // 0x15 = OperationError::ReservedInstruction,
    // 0x16 = OperationError::ReservedInstruction,
    // 0x17 = OperationError::ReservedInstruction,
    Mult = 0x18,
    Multu = 0x19,
    Div = 0x1A,
    Divu = 0x1B,
    // 0x1c = OperationError::ReservedInstruction,
    // 0x1D = OperationError::ReservedInstruction,
    // 0x1E = OperationError::ReservedInstruction,
    // 0x1F = OperationError::ReservedInstruction,
    Add = 0x20,
    Addu = 0x21,
    Sub = 0x22,
    Subu = 0x23,
    And = 0x24,
    Or = 0x25,
    Xor = 0x26,
    Nor = 0x27,
    // 0x28 = OperationError::ReservedInstruction,
    // 0x29 = OperationError::ReservedInstruction,
    Slt = 0x2A,
    Sltu = 0x2B,
    // 0x2C = OperationError::ReservedInstruction,
    // 0x2D = OperationError::ReservedInstruction,
    // 0x2E = OperationError::ReservedInstruction,
    // 0x2F = OperationError::ReservedInstruction,
    Tge = 0x30,
    Tgeu = 0x31,
    Tlt = 0x32,
    Tltu = 0x33,
    Teq = 0x34,
    // 0x35 = OperationError::ReservedInstruction,
    Tne = 0x36,
    // 0x37 = OperationError::ReservedInstruction,
    // 0x38 = OperationError::ReservedInstruction,
    // 0x39 = OperationError::ReservedInstruction,
    // 0x3A = OperationError::ReservedInstruction,
    // 0x3B = OperationError::ReservedInstruction,
    // 0x3C = OperationError::ReservedInstruction,
    // 0x3D = OperationError::ReservedInstruction,
    // 0x3E = OperationError::ReservedInstruction,
    // 0x3F = OperationError::ReservedInstruction,
}

impl TryFrom<u32> for Special1Op {
    type Error = OperationError;
    fn try_from(raw: u32) -> Result<Special1Op, OperationError> {
        Special1Op::from_repr(raw).ok_or(match raw {
            0x00..0x40 => OperationError::ReservedInstruction,
            0x40.. => OperationError::InvalidInstruction,
        })
    }
}

#[derive(Debug, PartialEq, Clone, Copy, FromRepr)]
#[repr(u32)]
pub enum RegimmOp {
    Bltz = 0x00,
    Bgez = 0x01,
    Bltzl = 0x02,
    Bgezl = 0x03,
    // 0x04 = OperationError::ReservedInstruction,
    // 0x05 = OperationError::ReservedInstruction,
    // 0x06 = OperationError::ReservedInstruction,
    // 0x07 = OperationError::ReservedInstruction,
    Tgei = 0x08,
    Tgeiu = 0x09,
    Tlti = 0x0A,
    Tltiu = 0x0B,
    Teqi = 0x0C,
    // 0x0D = OperationError::ReservedInstruction,
    Tnei = 0x0E,
    // 0x0F = OperationError::ReservedInstruction,
    Bltzal = 0x20,
    Bgezal = 0x21,
    Bltzall = 0x22,
    Bgezall = 0x23,
    // 0x24 = OperationError::ReservedInstruction,
    // 0x25 = OperationError::ReservedInstruction,
    // 0x26 = OperationError::ReservedInstruction,
    // 0x27 = OperationError::ReservedInstruction,
    // 0x28 = OperationError::ReservedInstruction,
    // 0x29 = OperationError::ReservedInstruction,
    // 0x2A = OperationError::ReservedInstruction,
    // 0x2B = OperationError::ReservedInstruction,
    // 0x2C = OperationError::ReservedInstruction,
    // 0x2D = OperationError::ReservedInstruction,
    // 0x2E = OperationError::ReservedInstruction,
    Synci = 0x2F,
}

impl TryFrom<u32> for RegimmOp {
    type Error = OperationError;
    fn try_from(raw: u32) -> Result<RegimmOp, OperationError> {
        RegimmOp::from_repr(raw).ok_or(match raw {
            0x00..0x40 => OperationError::ReservedInstruction,
            0x40.. => OperationError::InvalidInstruction,
        })
    }
}

#[derive(Debug, PartialEq, Clone, Copy, FromRepr)]
#[repr(u32)]
pub enum Special2Op {
    Madd = 0x00,
    Maddu = 0x01,
    Mul = 0x02,
    // 0x03 = OperationError::ReservedInstruction,
    Msub = 0x04,
    Msubu = 0x05,
    // 0x06 = OperationError::ReservedInstruction,
    // 0x07 = OperationError::ReservedInstruction,
    // 0x08 = OperationError::ReservedInstruction,
    // 0x09 = OperationError::ReservedInstruction,
    // 0x0A = OperationError::ReservedInstruction,
    // 0x0B = OperationError::ReservedInstruction,
    // 0x0C = OperationError::ReservedInstruction,
    // 0x0D = OperationError::ReservedInstruction,
    // 0x0E = OperationError::ReservedInstruction,
    // 0x0F = OperationError::ReservedInstruction,
    // 0x10 = OperationError::ReservedInstruction,
    // 0x11 = OperationError::ReservedInstruction,
    // 0x12 = OperationError::ReservedInstruction,
    // 0x13 = OperationError::ReservedInstruction,
    // 0x14 = OperationError::ReservedInstruction,
    // 0x15 = OperationError::ReservedInstruction,
    // 0x16 = OperationError::ReservedInstruction,
    // 0x17 = OperationError::ReservedInstruction,
    // 0x18 = OperationError::ReservedInstruction,
    // 0x19 = OperationError::ReservedInstruction,
    // 0x1A = OperationError::ReservedInstruction,
    // 0x1B = OperationError::ReservedInstruction,
    // 0x1c = OperationError::ReservedInstruction,
    // 0x1D = OperationError::ReservedInstruction,
    // 0x1E = OperationError::ReservedInstruction,
    // 0x1F = OperationError::ReservedInstruction,
    Clz = 0x20,
    Clo = 0x21,
    // 0x22 = OperationError::ReservedInstruction,
    // 0x23 = OperationError::ReservedInstruction,
    // 0x24 = OperationError::ReservedInstruction,
    // 0x25 = OperationError::ReservedInstruction,
    // 0x26 = OperationError::ReservedInstruction,
    // 0x27 = OperationError::ReservedInstruction,
    // 0x28 = OperationError::ReservedInstruction,
    // 0x29 = OperationError::ReservedInstruction,
    // 0x2A = OperationError::ReservedInstruction,
    // 0x2B = OperationError::ReservedInstruction,
    // 0x2C = OperationError::ReservedInstruction,
    // 0x2D = OperationError::ReservedInstruction,
    // 0x2E = OperationError::ReservedInstruction,
    // 0x2F = OperationError::ReservedInstruction,
    // 0x30 = OperationError::ReservedInstruction,
    // 0x31 = OperationError::ReservedInstruction,
    // 0x32 = OperationError::ReservedInstruction,
    // 0x33 = OperationError::ReservedInstruction,
    // 0x34 = OperationError::ReservedInstruction,
    // 0x35 = OperationError::ReservedInstruction,
    // 0x36 = OperationError::ReservedInstruction,
    // 0x37 = OperationError::ReservedInstruction,
    // 0x38 = OperationError::ReservedInstruction,
    // 0x39 = OperationError::ReservedInstruction,
    // 0x3A = OperationError::ReservedInstruction,
    // 0x3B = OperationError::ReservedInstruction,
    // 0x3C = OperationError::ReservedInstruction,
    // 0x3D = OperationError::ReservedInstruction,
    // 0x3E = OperationError::ReservedInstruction,
    Sdbbp = 0x3F,
}

impl TryFrom<u32> for Special2Op {
    type Error = OperationError;
    fn try_from(raw: u32) -> Result<Special2Op, OperationError> {
        Special2Op::from_repr(raw).ok_or(match raw {
            0x00..0x40 => OperationError::ReservedInstruction,
            0x40.. => OperationError::InvalidInstruction,
        })
    }
}
