use crate::structs::{InstructionInformation, Memory, Processor};
use crate::elf_def::{MIPS_DATA_START_ADDR, MIPS_TEXT_START_ADDR};

impl InstructionInformation {
    pub fn get_mnemonic(&self) -> String {
        return self.mnemonic.to_string();
    }
}

impl Processor {
    pub fn new(entry: u32) -> Self {
        Processor {
            pc: entry,
            registers: [0;32],
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