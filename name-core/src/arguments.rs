use crate::instruction::RawInstruction;

#[derive(Debug, Clone)]
pub struct RArgs {
    pub rd: usize,
    pub rt: usize,
    pub rs: usize,
    pub shamt: usize,
}

impl RArgs {
    pub fn unpack(instr: RawInstruction) -> RArgs {
        let rs: u32 = (instr.raw >> 21) & 0x1F;
        let rt: u32 = (instr.raw >> 16) & 0x1F;
        let rd: u32 = (instr.raw >> 11) & 0x1F;
        let shamt: u32 = (instr.raw >> 6) & 0x1F;

        RArgs {
            rs: rs as usize,
            rt: rt as usize,
            rd: rd as usize,
            shamt: shamt as usize,
        }
    }
}

#[derive(Debug, Clone)]
pub struct IArgs {
    pub rs: usize,
    pub rt: usize,
    pub imm: u32,
}

impl IArgs {
    pub fn unpack(instr: RawInstruction) -> IArgs {
        let rs: u32 = (instr.raw >> 21) & 0x1F;
        let rt: u32 = (instr.raw >> 16) & 0x1F;
        let imm: u32 = (instr.raw & 0xFFFF) as i16 as i32 as u32;

        IArgs {
            rs: rs as usize,
            rt: rt as usize,
            imm,
        }
    }
}

#[derive(Debug, Clone)]
pub struct JArgs {
    pub imm: u32,
}

impl JArgs {
    pub fn unpack(instr: RawInstruction) -> JArgs {
        let imm: u32 = instr.raw & 0x03FFFFFF;

        JArgs { imm }
    }
}
