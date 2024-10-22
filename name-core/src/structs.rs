#[derive(Debug)]
pub struct Symbol {
    pub symbol_type: u8,
    pub identifier: String,
    pub value: u32,
    pub size: u32,
    pub visibility: Visibility,
    pub section: Section,
}

#[derive(Debug)]
pub struct Processor {
    pub pc: u32,
    pub general_purpose_registers: [u32; 32],
}

#[derive(Debug)]
pub struct Coprocessor0 {
    pub registers: [u32; 32],
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
pub struct ProgramState {
    pub cpu: Processor,
    pub cp0: Coprocessor0,
    pub memory: Memory,
}

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
pub struct LineInfo {
    pub content: String,
    pub line_number: u32,
    pub start_address: u32,
    pub end_address: u32,
}
