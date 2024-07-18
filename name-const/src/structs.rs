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
}

#[derive(Debug)]
pub struct Symbol {
    pub symbol_type: u8,
    pub identifier: String,
    pub value: u32,
    pub size: u32,
    pub visibility: Visibility,
    pub section: Section,
}

#[derive(Debug, Default)]
pub enum Visibility {
    #[default]
    Local,
    Global,
    Weak,
}

#[derive(Debug)]
pub enum Section {
    Null,
    Text,
    Data,
}

#[derive(Debug)]
pub struct Processor {
    pub pc: u32,
    pub registers: [u32; 32],
}

pub struct Memory {
    pub text: Vec<u8>,
    pub data: Vec<u8>,
    pub text_start: u32,
    pub text_end: u32,
    pub data_start: u32,
    pub data_end: u32,
}