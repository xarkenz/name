use crate::operation::{ImmediateOp, Operation, OperationError, RegimmOp, Special1Op, Special2Op};

use std::mem;

#[derive(Debug, Clone, Copy)]
#[repr(usize)]
pub enum Register {
    Zero,
    At,
    V0,
    V1,
    A0,
    A1,
    A2,
    A3,
    T0,
    T1,
    T2,
    T3,
    T4,
    T5,
    T6,
    T7,
    S0,
    S1,
    S2,
    S3,
    S4,
    S5,
    S6,
    S7,
    T8,
    T9,
    K0,
    K1,
    Gp,
    Sp,
    Fp,
    Ra,
}

impl TryFrom<u32> for Register {
    type Error = InstructionError;
    fn try_from(raw: u32) -> Result<Register, InstructionError> {
        match raw as usize {
            reg @ 0..32 => Ok(unsafe { mem::transmute(reg) }),
            _ => Err(InstructionError::InvalidRegister),
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

    pub fn opcode(self) -> u32 {
        self.raw >> 26
    }

    pub fn is_immediate(self) -> bool {
        match self.opcode() {
            0x00 | 0x01 | 0x1C => false,
            0x00..0x3F => true,
            _ => false,
        }
    }

    pub fn is_special1(self) -> bool {
        self.opcode() == 0x00
    }

    pub fn is_special2(self) -> bool {
        self.opcode() == 0x1C
    }

    pub fn is_special(self) -> bool {
        self.is_special1() || self.is_special2()
    }

    pub fn is_regimm(self) -> bool {
        self.opcode() == 0x01
    }

    pub fn operation(self) -> Result<Operation, InstructionError> {
        self.get_special_op()
            .map(|spec| match spec {
                SpecialOp::Special1(spec1) => Operation::Special1(spec1),
                SpecialOp::Special2(spec2) => Operation::Special2(spec2),
            })
            .or(self.get_regimm_op().map(|op| Operation::Regimm(op)))
            .or(self.get_immediate_op().map(|op| Operation::Immediate(op)))
    }

    pub fn get_immediate_op(self) -> Result<ImmediateOp, InstructionError> {
        ImmediateOp::try_from(self.opcode()).map_err(|e| InstructionError::Operation(e))
    }

    pub fn get_special_op(self) -> Result<SpecialOp, InstructionError> {
        if self.is_special1() {
            Ok(SpecialOp::Special1(
                Special1Op::try_from(self.raw & 0x3F)
                    .map_err(|e| InstructionError::Operation(e))?,
            ))
        } else if self.is_special2() {
            Ok(SpecialOp::Special2(
                Special2Op::try_from(self.raw & 0x3F)
                    .map_err(|e| InstructionError::Operation(e))?,
            ))
        } else {
            Err(InstructionError::NoSuchField)
        }
    }

    pub fn get_regimm_op(self) -> Result<RegimmOp, InstructionError> {
        if self.is_regimm() {
            RegimmOp::try_from(self.raw >> 16 & 0x1F).map_err(|e| InstructionError::Operation(e))
        } else {
            Err(InstructionError::NoSuchField)
        }
    }

    pub fn get_rs(self) -> Result<Register, InstructionError> {
        Register::try_from(self.raw >> 21 & 0x1F)
    }

    pub fn get_rt(self) -> Result<Register, InstructionError> {
        if self.is_regimm() {
            Err(InstructionError::NoSuchField)
        } else {
            Register::try_from(self.raw >> 16 & 0x1F)
        }
    }

    pub fn get_shamt(self) -> Result<u32, InstructionError> {
        if self.is_special() {
            Ok(self.raw >> 6 & 0x1F)
        } else {
            Err(InstructionError::NoSuchField)
        }
    }

    pub fn get_rd(self) -> Result<Register, InstructionError> {
        if self.is_regimm() | self.is_immediate() {
            Err(InstructionError::NoSuchField)
        } else {
            Register::try_from(self.raw >> 11 & 0x1F)
        }
    }

    pub fn get_immediate(self) -> Result<u32, InstructionError> {
        if self.is_special() {
            Err(InstructionError::NoSuchField)
        } else {
            Ok(self.raw & 0xFFFF)
        }
    }

    pub fn get_26_bit_address(self) -> Result<u32, InstructionError> {
        match self.get_immediate_op()? {
            ImmediateOp::J | ImmediateOp::Jal | ImmediateOp::Jalx => Ok(self.raw & 0x3FFFFFF),
            _ => Err(InstructionError::NoSuchField),
        }
    }
}

#[derive(Debug, Clone)]
pub enum InstructionError {
    Operation(OperationError),
    InvalidRegister,
    NoSuchField,
}

#[derive(Debug, Clone)]
pub struct Immediate {
    pub opcode: ImmediateOp,
    pub rs: Register,
    pub rt: Register,
    pub imm: u32,
}

impl TryFrom<RawInstruction> for Immediate {
    type Error = InstructionError;
    fn try_from(raw: RawInstruction) -> Result<Immediate, InstructionError> {
        Ok(Immediate {
            opcode: raw.get_immediate_op()?,
            rs: raw.get_rs()?,
            rt: raw.get_rt()?,
            imm: raw.get_immediate()?,
        })
    }
}

#[derive(Debug, Clone)]
pub enum SpecialOp {
    Special1(Special1Op),
    Special2(Special2Op),
}

#[derive(Debug, Clone)]
pub struct Special {
    pub special_opcode: SpecialOp,
    pub rs: Register,
    pub rt: Register,
    pub rd: Register,
    pub shamt: u32,
}

impl TryFrom<RawInstruction> for Special {
    type Error = InstructionError;
    fn try_from(raw: RawInstruction) -> Result<Special, InstructionError> {
        Ok(Special {
            special_opcode: raw.get_special_op()?,
            rs: raw.get_rs()?,
            rt: raw.get_rt()?,
            rd: raw.get_rd()?,
            shamt: raw.get_shamt()?,
        })
    }
}

#[derive(Debug, Clone)]
pub struct Regimm {
    pub regimm_op: RegimmOp,
    pub rs: Register,
    pub offset: u32,
}

impl TryFrom<RawInstruction> for Regimm {
    type Error = InstructionError;
    fn try_from(raw: RawInstruction) -> Result<Regimm, InstructionError> {
        Ok(Regimm {
            regimm_op: raw.get_regimm_op()?,
            rs: raw.get_rs()?,
            offset: raw.get_immediate()?,
        })
    }
}

#[derive(Debug, Clone)]
pub struct Jump {
    pub opcode: ImmediateOp,
    pub address: u32,
}

impl TryFrom<RawInstruction> for Jump {
    type Error = InstructionError;
    fn try_from(raw: RawInstruction) -> Result<Jump, InstructionError> {
        Ok(Jump {
            opcode: raw.get_immediate_op()?,
            address: raw.get_26_bit_address()?,
        })
    }
}

#[derive(Debug, Clone)]
pub enum Instruction {
    Imm(Immediate),
    Spec(Special),
    Regimm(Regimm),
    J(Jump),
}

impl TryFrom<RawInstruction> for Instruction {
    type Error = InstructionError;
    fn try_from(raw: RawInstruction) -> Result<Instruction, InstructionError> {
        match raw.operation()? {
            Operation::Immediate(imm) => match imm {
                ImmediateOp::J | ImmediateOp::Jal | ImmediateOp::Jalx => {
                    Ok(Instruction::J(Jump::try_from(raw)?))
                }
                _ => Ok(Instruction::Imm(Immediate::try_from(raw)?)),
            },
            Operation::Special2(_) | Operation::Special1(_) => {
                Ok(Instruction::Spec(Special::try_from(raw)?))
            }
            Operation::Regimm(_) => Ok(Instruction::Regimm(Regimm::try_from(raw)?)),
        }
    }
}
