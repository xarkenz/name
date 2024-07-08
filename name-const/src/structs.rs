#[derive(Debug, PartialEq)]
pub struct Backpatch {
    pub instruction_info: &'static InstructionInformation,
    pub arguments: Vec<LineComponent>,
    pub undiscovered_identifier: String,
    pub backpatch_address: u32,
    pub byte_offset: usize,
    pub line_number: usize,
}

#[derive(Debug)]
pub struct LineInfo {
    pub line_number: u32,
    pub content: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LineComponent {
    Mnemonic(String),
    Register(String),
    Immediate(i32),
    Identifier(String),
    Label(String),
    Directive(String),
}

#[derive(Debug, PartialEq)]
pub struct InstructionInformation {
    pub mnemonic: &'static str,
    pub instruction_type: InstructionType,
    pub opcode: Option<u32>,
    pub funct: Option<u32>,
    pub args: &'static [ArgumentType],
    pub has_optional: bool,
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
}

#[derive(Debug)]
pub struct Symbol {
    pub symbol_type: SymbolType,
    pub identifier: String,
    pub value: u32,
}

#[derive(Debug)]
pub enum SymbolType {
    Equivalence,
    Address,
}

#[derive(Debug)]
pub enum Section {
    Null,
    Text,
    Data,
}