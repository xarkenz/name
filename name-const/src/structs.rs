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

#[derive(Debug, Clone)]
pub enum Section {
    Null,
    Text,
    Data,
}

#[derive(Debug)]
pub struct Processor {
    pub pc: u32,
    pub general_purpose_registers: [u32; 32],
}

#[derive(Debug)]
pub struct Memory {
    pub data: Vec<u8>,
    pub text: Vec<u8>,
    pub data_start: u32,
    pub data_end: u32,
    pub text_start: u32,
    pub text_end: u32,
}

#[derive(Debug)]
pub struct LineInfo {
    pub content: String,
    pub line_number: u32,
    pub start_address: u32,
    pub end_address: u32,
}