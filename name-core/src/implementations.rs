use crate::elf_def::{MIPS_DATA_START_ADDR, MIPS_TEXT_START_ADDR};
use crate::exception::constants::*;
use crate::structs::{LineInfo, Memory, Processor, Coprocessor0, ProgramState};

impl Processor {
    pub fn new(entry: u32) -> Self {
        Processor {
            pc: entry,
            general_purpose_registers: [0; 32],
        }
    }
}

// TODO: Fill any default values for cp0 fields
impl Coprocessor0 {
    pub fn new() -> Self {
        Coprocessor0 {
            registers: [0; 32],
        }
    }
}

impl Memory {
    pub fn new(data: Vec<u8>, text: Vec<u8>) -> Self {
        let data_end = MIPS_DATA_START_ADDR + data.len() as u32;
        let text_end = MIPS_TEXT_START_ADDR + text.len() as u32;

        Memory {
            data,
            text,
            data_start: MIPS_DATA_START_ADDR,
            data_end,
            text_start: MIPS_TEXT_START_ADDR,
            text_end,
        }
    }

    pub fn read_byte(&self, address: u32) -> Result<u8, String> {
        // Ensure offset is within memory bounds
        if address >= self.data_end {
            return Err(format!(
                " - Address 0x{:x} is out of bounds (upper bound check failed)",
                address
            ));
        }

        // Perform address translation
        let offset = match address.checked_sub(MIPS_DATA_START_ADDR) {
            Some(offs) => offs as usize,
            None => {
                return Err(format!(
                    " - Address 0x{:x} is out of bounds (lower bound check failed)",
                    address
                ))
            }
        };

        // Read the byte from memory
        Ok(self.data[offset].clone())
    }
}

impl ProgramState {
    pub fn new(cpu: Processor, memory: Memory) -> Self {
        ProgramState {
            cpu: cpu,
            cp0: Coprocessor0::new(),
            memory: memory,
        }
    }

    pub fn is_exception(&self) -> bool {
        return self.cp0.get_exception_level() == EXCEPTION_BEING_HANDLED;
    }
}

impl LineInfo {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = self.content.as_bytes().to_vec();
        bytes.push(b'\0');

        bytes.extend_from_slice(&self.line_number.to_be_bytes());
        bytes.extend_from_slice(&self.start_address.to_be_bytes());
        bytes.extend_from_slice(&self.end_address.to_be_bytes());

        bytes
    }
}
