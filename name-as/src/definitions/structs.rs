use super::expandables::ExpansionFn;
use name_core::instruction::information::InstructionInformation;

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

#[derive(Debug)]
pub(crate) struct PseudoInstruction {
    pub(crate) mnemonic: &'static str,
    pub(crate) expand: ExpansionFn,
}
