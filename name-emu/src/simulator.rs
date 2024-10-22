use crate::simulator_helpers::extract_loadable_sections;

use crate::debug::debug_utils::{debugger, single_step};

use name_core::elf_def::Elf;
use name_core::elf_utils::extract_lineinfo;
use name_core::structs::{LineInfo, Memory, Processor, ProgramState};

pub fn simulate(elf: Elf, debug: bool) -> Result<(), String> {
    // Set up simulation environment from information in ELF
    let mut cpu: Processor = Processor::new(elf.file_header.e_entry);

    let (data, text) = extract_loadable_sections(&elf);

    let lineinfo: Vec<LineInfo> = extract_lineinfo(&elf);

    let mut memory: Memory = Memory::new(data, text);

    // Create program state
    let mut program_state: ProgramState = ProgramState::new(cpu, memory);

    if debug {
        // Invoke the cli debugger if the user asked for it
        debugger(&lineinfo, &mut program_state)
    } else {
        // Begin fetch/decode/execute cycle to run program normally
        loop {
            single_step(&lineinfo, &mut program_state, &Vec::new());
            if program_state.is_exception() {
                todo!("Call on exception handler");
            }
        }
    }
}
