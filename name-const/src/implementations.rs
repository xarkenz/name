use crate::structs::{InstructionInformation, LineComponent, Memory, Processor};
use crate::elf_def::{MIPS_DATA_START_ADDR, MIPS_TEXT_START_ADDR};

use std::fmt;

impl InstructionInformation {
    pub fn get_mnemonic(&self) -> String {
        return self.mnemonic.to_string();
    }
}

// I wanted .to_string() to work
impl fmt::Display for LineComponent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LineComponent::Mnemonic(m) => write!(f, "{}", m),
            LineComponent::Register(r) => write!(f, "{}", r),
            LineComponent::Immediate(i) => write!(f, "{}", i),
            LineComponent::Identifier(i) => write!(f, "{}", i),
            LineComponent::Label(l) => write!(f, "{}", l),
            LineComponent::Directive(d) => write!(f, "{}", d),
            LineComponent::DoubleQuote(d) => write!(f, "{}", d),
        }
    }
}

impl Processor {
    pub fn new(entry: u32) -> Self {
        Processor {
            pc: entry,
            general_purpose_registers: [0;32],
        }
    }
}

impl Memory {
    pub fn new(text: Vec<u8>, data: Vec<u8>) -> Self {
        let text_end = MIPS_TEXT_START_ADDR + text.len() as u32;
        let data_end = MIPS_DATA_START_ADDR + data.len() as u32;
        
        Memory {
            text,
            data,
            text_start: MIPS_TEXT_START_ADDR,
            text_end,
            data_start: MIPS_DATA_START_ADDR,
            data_end,
        }
    }
}