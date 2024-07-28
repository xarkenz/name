use super::expandables::ExpansionFn;

#[derive(Debug, PartialEq)]
pub struct Backpatch {
    pub instruction_info: &'static InstructionInformation,
    pub(crate) backpatch_type: BackpatchType,
    pub arguments: Vec<LineComponent>,
    pub undiscovered_identifier: String,
    pub backpatch_address: u32,
    pub byte_offset: usize,
    pub line_number: usize,
}

#[derive(Debug, PartialEq)]
pub(crate) enum BackpatchType {
    Standard,
    Upper,
    Lower,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LineComponent {
    Mnemonic(String),
    Register(String),
    Immediate(i32),
    Identifier(String),
    Label(String),
    Directive(String),
    DoubleQuote(String),
    Colon,
}

#[derive(Debug, PartialEq)]
pub struct InstructionInformation {
    pub mnemonic: &'static str,
    pub instruction_type: InstructionType,
    pub opcode: Option<u32>,
    pub funct: Option<u32>,
    pub args: &'static [ArgumentType],
    pub alt_args: Option< &'static [&'static [ArgumentType]] >,
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
    // Label,
}

#[derive(Debug)]
pub(crate) struct PseudoInstruction {
    pub(crate) mnemonic: &'static str,
    pub(crate) expand: ExpansionFn,
}