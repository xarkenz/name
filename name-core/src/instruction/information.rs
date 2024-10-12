use crate::{
    instruction::instruction::Instruction,
    structs::{ExecutionStatus, Memory, Processor},
};

#[derive(Debug, PartialEq)]
pub struct InstructionInformation {
    pub mnemonic: &'static str,
    pub instruction_type: InstructionType,
    pub op_code: u32,
    pub funct_code: Option<u32>,
    pub implementation:
        fn(&mut Processor, &mut Memory, Instruction) -> Result<ExecutionStatus, String>,
    pub args: &'static [ArgumentType],
    pub alt_args: Option<&'static [&'static [ArgumentType]]>,
}

impl InstructionInformation {
    pub fn lookup_code(&self) -> u32 {
        self.op_code << 6 | self.funct_code.unwrap_or(0)
    }
}

#[derive(Debug, PartialEq)]
pub enum InstructionType {
    RType,
    IType,
    JType,
}

#[derive(Debug, PartialEq)]
pub enum ArgumentType {
    Rd,
    Rs,
    Rt,
    Immediate,
    Identifier,
    BranchLabel,
}
