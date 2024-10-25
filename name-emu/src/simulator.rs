use crate::simulator_helpers::extract_loadable_sections;

use crate::debug::debug_utils::{debugger, single_step, DebuggerState};

use name_core::elf_def::Elf;
use name_core::elf_utils::extract_lineinfo;
use name_core::structs::{ExecutionStatus, LineInfo, Memory, Processor};

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
            match single_step(
                &lineinfo,
                &mut cpu,
                &mut memory,
                &DebuggerState::new(Vec::new(), 0, 5),
            ) {
                Ok(execution_status) => match execution_status {
                    ExecutionStatus::Continue => {}
                    ExecutionStatus::Break => {
                        // assuming this behavior will be more well defined upon implementation of the extension
                        println!("Break instruction located at address {}", cpu.pc);
                        break Ok(());
                    }
                    ExecutionStatus::Complete => return Ok(()),
                },
                Err(e) => return Err(e),
            };
        }
    }
}
