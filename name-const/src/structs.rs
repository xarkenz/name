#[derive(Debug)]
pub struct LineInfo {
    pub line_number: u32,
    pub content: String,
}

#[derive(Debug)]
pub struct LineComponent {
    pub component_type: ComponentType,
    pub content: String,
}

#[derive(Debug)]
pub enum ComponentType {
    Mnemonic,
    Register,
    Immediate,
    Identifier,
    Label,
    Directive,
}

#[derive(Debug)]
pub struct InstructionInformation {
    pub mnemonic: &'static str,
    pub instruction_type: InstructionType,
    pub opcode: Option<u32>,
    pub funct: Option<u32>,
    pub args: &'static [ArgumentType],
}

#[derive(Debug)]
pub enum InstructionType {
    RType,
    IType,
    JType,
}

#[derive(Debug)]
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
    pub value: Option<u32>,
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