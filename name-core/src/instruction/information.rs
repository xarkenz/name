use crate::{
    instruction::instruction::RawInstruction,
    structs::{ExecutionStatus, Memory, Processor},
};
use std::fmt::Debug;

pub struct InstructionInformation {
    pub mnemonic: &'static str,
    pub instruction_type: InstructionType,
    pub op_code: u32,
    pub funct_code: Option<u32>,
    pub implementation: Box<
        dyn Fn(&mut Processor, &mut Memory, RawInstruction) -> Result<ExecutionStatus, String>
            + Sync
            + Send,
    >,
    pub args: &'static [ArgumentType],
    pub alt_args: Option<&'static [&'static [ArgumentType]]>,
}

impl PartialEq for InstructionInformation {
    fn eq(&self, other: &Self) -> bool {
        self.mnemonic == other.mnemonic
    }
}

impl Debug for InstructionInformation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "InstructionInformation {{
                mnemonic: {:?},
                instruction_type: {:?}
                op_code: {:?},
                funct_code {:?},
                implementation: {:?},
                args: {:?},
                alt+args: {:?}
            }}",
            self.mnemonic,
            self.instruction_type,
            self.op_code,
            self.funct_code,
            self.instruction_type,
            self.args,
            self.alt_args
        )
    }
}

impl InstructionInformation {
    pub fn lookup_code(&self) -> u32 {
        self.op_code << 6 | self.funct_code.unwrap_or(0)
    }
}

pub fn wrap_imp<Args: From<RawInstruction> + 'static>(
    f: fn(&mut Processor, &mut Memory, Args) -> Result<ExecutionStatus, String>,
) -> Box<
    dyn Fn(&mut Processor, &mut Memory, RawInstruction) -> Result<ExecutionStatus, String>
        + Sync
        + Send,
> {
    Box::new(move |processor, mem, instr| f(processor, mem, Args::from(instr)))
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
