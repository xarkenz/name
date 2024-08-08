use name_const::elf_def::MIPS_ADDRESS_ALIGNMENT;
use name_const::structs::{LineInfo, Memory, Processor};

use crate::decode::{decode, InstructionFn};

use crate::definitions::structs::ExecutionStatus;
use crate::fetch::fetch;
use crate::simulator_helpers::generate_err;

pub fn single_step(lineinfo: &Vec<LineInfo>, cpu: &mut Processor, memory: &mut Memory) -> Result<ExecutionStatus, String> {
    if cpu.pc > memory.text_end || cpu.pc < memory.text_start {
        return Err(generate_err(lineinfo, cpu.pc-4, "Program fell off bottom."));
    }

    // Fetch
    let fetched_instruction: u32 = fetch(&cpu.pc, &memory)?;

    cpu.pc += MIPS_ADDRESS_ALIGNMENT;

    // Decode
    let decoded_instruction_fn: InstructionFn = match decode(&fetched_instruction) {
        Ok(fun) => fun,
        Err(e) => return Err(generate_err(lineinfo, cpu.pc-4, format!("Failed instruction fetch: {}.", e).as_str() )),
    };

    // Execute
    let instruction_result = decoded_instruction_fn(cpu, memory, fetched_instruction);

    // The $0 register should never have been permanently changed. Don't let it remain changed.
    cpu.general_purpose_registers[0] = 0;

    // The instruction result contains an enum value representing whether or not "execution should stop now".
    match instruction_result {
        Ok(execution_status) => {
            return Ok(execution_status);
        },
        Err(e) => return Err(generate_err(lineinfo, cpu.pc-4, format!("{}", e).as_str() )),
    }
}