use crate::exception::constants::*;
use crate::structs::{Coprocessor0, LineInfo, Memory, Processor, ProgramState};

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
        Coprocessor0 { registers: [0; 32] }
    }
}

impl ProgramState {
    pub fn new(cpu: Processor, memory: Memory) -> Self {
        ProgramState {
            should_continue_execution: true,
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
