use crate::definitions::structs::ExecutionStatus;
use crate::simulator_helpers::{extract_loadable_sections, single_step};

use name_const::elf_def::Elf;
use name_const::structs::{Memory, Processor};

pub fn simulate(elf: Elf) -> Result<(), String> {
    // Set up simulation environment
    let mut cpu: Processor = Processor::new(elf.file_header.e_entry);

    let (text, data) = extract_loadable_sections(&elf);

    let mut memory: Memory = Memory::new(text, data);

    // Begin fetch/decode/execute cycle
    loop {
        match single_step(&mut cpu, &mut memory){
            Ok(execution_status) => match execution_status {
                ExecutionStatus::Continue => {},
                ExecutionStatus::Complete => return Ok(()),
            },
            Err(e) => return Err(e),
        };
    }
}