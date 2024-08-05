use name_const::elf_utils::find_target_section_index;

use name_const::elf_def::{Elf, MIPS_ADDRESS_ALIGNMENT};
use name_const::structs::{Memory, Processor};

use crate::decode::{decode, InstructionFn};

use crate::definitions::structs::ExecutionStatus;

// Extract section .text and section .data from the ELF
pub fn extract_loadable_sections(elf: &Elf) -> (Vec<u8>, Vec<u8>) {
    // Search section header string table for '.text' and '.data'
    let text_section: Vec<u8> = match find_target_section_index(&elf.section_header_table, &elf.sections[elf.file_header.e_shstrndx as usize], ".text") {
        Some(section_index) => elf.sections[section_index].clone(),
        None => unreachable!(),
    };

    let data_section: Vec<u8> = match find_target_section_index(&elf.section_header_table, &elf.sections[elf.file_header.e_shstrndx as usize], ".data") {
        Some(section_index) => elf.sections[section_index].clone(),
        None => vec!(),
    };
    
    (text_section, data_section)
}

// Fetch the next instruction and check that it's in acceptable memory space
pub fn fetch(fetch_address: &u32, memory: &Memory) -> Result<u32, String> {
    if fetch_address > &memory.text_end {
        return Err("Program fell off bottom.".to_string());
    } else if fetch_address < &memory.text_start {
        return Err("Program counter has reached unowned address space. You, the user, have made a grave mistake.".to_string());
    }
    
    let instr_index: usize = (fetch_address - &memory.text_start) as usize;
    
    let fetched_instruction = u32::from_be_bytes( 
        match memory.text[instr_index..instr_index+4].try_into() {
            Ok(val) => val,
            Err(e) => return Err(e.to_string()),
    });

    Ok(fetched_instruction)
}

pub fn single_step(cpu: &mut Processor, memory: &mut Memory) -> Result<ExecutionStatus, String> {
        if cpu.pc > memory.text_end || cpu.pc < memory.text_start {
            return Err(format!("Program fell off bottom."));
        }

        // Fetch
        let fetched_instruction: u32 = fetch(&cpu.pc, &memory)?;

        cpu.pc += MIPS_ADDRESS_ALIGNMENT;

        // Decode
        let decoded_instruction_fn: InstructionFn = match decode(&fetched_instruction) {
            Ok(fun) => fun,
            Err(e) => return Err(format!("An error occurred when fetching instruction at address 0x{:x} (instruction {}): {}.", cpu.pc-4, (cpu.pc-4-memory.text_start)/4, e)),
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
            Err(e) => return Err(format!("An error occurred when executing instruction at address 0x{:x} (instruction #{}): {}.", cpu.pc-4, (cpu.pc-memory.text_start)/4, e)),
        }
}