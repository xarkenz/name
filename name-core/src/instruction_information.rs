use crate::operation::Operation;

#[derive(Debug, PartialEq)]
pub struct InstructionInformation {
    pub mnemonic: &'static str,
    pub instruction_type: InstructionType,
    pub operation: Operation,
    pub args: &'static [ArgumentType],
    pub alt_args: Option<&'static [&'static [ArgumentType]]>,
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
