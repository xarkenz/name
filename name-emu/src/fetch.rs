use name_core::{exception::definitions::ExceptionType, instruction::RawInstruction, structs::ProgramState};

// Fetch the next instruction and check that it's in acceptable memory space
pub fn fetch(program_state: &mut ProgramState) -> RawInstruction {
    if program_state.cpu.pc > program_state.memory.text_end || program_state.cpu.pc < program_state.memory.text_start {
        program_state.set_exception(ExceptionType::AddressExceptionLoad);
    }

    let instr_index: usize = (program_state.cpu.pc - program_state.memory.text_start) as usize;

    let fetched_instruction =
    // This error is pretty much impossible to get with normal use. However, I still want to represent it as best as possible.
        u32::from_be_bytes(match program_state.memory.text[instr_index..instr_index + 4].try_into() {
            Ok(val) => val,
            Err(_) => {
                program_state.set_exception(ExceptionType::BusFetch);
                
                [0; 4] // Fill with 0's because it's not going to execute anyways.
            },
        });

    RawInstruction::new(fetched_instruction)
}
