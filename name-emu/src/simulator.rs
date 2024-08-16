use crate::definitions::structs::ExecutionStatus;
use crate::simulator_helpers::extract_loadable_sections;

use crate::debug::debug_utils::{single_step, debugger};

use name_const::elf_def::Elf;
use name_const::elf_utils::extract_lineinfo;
use name_const::structs::{LineInfo, Memory, Processor};

pub fn simulate(elf: Elf, debug: bool) -> Result<(), String> {
    // Set up simulation environment
    let mut cpu: Processor = Processor::new(elf.file_header.e_entry);

    let (data, text) = extract_loadable_sections(&elf);

    let lineinfo: Vec<LineInfo> = extract_lineinfo(&elf);

    let mut memory: Memory = Memory::new(data, text);

    if debug {
        debugger(&lineinfo, &mut memory, &mut cpu)
    } else {
        // Begin fetch/decode/execute cycle to run program normally
        loop {
            match single_step(&lineinfo, &mut cpu, &mut memory){
                Ok(execution_status) => match execution_status {
                    ExecutionStatus::Continue => {},
                    ExecutionStatus::Break => { println!{"not broke?"}; },
                    ExecutionStatus::Complete => return Ok(()),
                },
                Err(e) => return Err(e),
            };
        }
    }
}