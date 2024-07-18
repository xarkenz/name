use crate::decode::{decode, InstructionFn};

use name_const::elf_def::{Elf, MIPS_ADDRESS_ALIGNMENT};
use name_const::elf_utils::find_target_section_index;
use name_const::structs::{Memory, Processor};

// There's some simplicity to appreciate here. This is an excellent solution.
pub fn simulate(elf: Elf) -> Result<(), String> {
    // TODO: Compliance check executable

    // Set up simulation environment
    let mut cpu: Processor = Processor::new(elf.file_header.e_entry);

    let (text, data) = extract_loadables(&elf);

    let mut memory: Memory = Memory::new(text, data);


    // Begin fetch/decode/execute cycle
    loop {
        // Fetch
        let fetched_instruction: u32 = fetch(&cpu.pc, &memory)?;
        cpu.pc += MIPS_ADDRESS_ALIGNMENT;

        if cpu.pc >= memory.text_end || cpu.pc < memory.text_start {
            break;
        }

        // Decode
        let decoded_instruction_fn: InstructionFn = decode(&fetched_instruction);

        // Execute
        let _instruction_result = decoded_instruction_fn(&mut cpu, &mut memory);
    }

    Ok(())
}

// Extract section .text and section .data from the ELF
fn extract_loadables(elf: &Elf) -> (Vec<u8>, Vec<u8>) {
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
fn fetch(fetch_address: &u32, memory: &Memory) -> Result<u32, String> {
    if fetch_address > &memory.text_end {
        return Err("Program fell off bottom.".to_string());
    } else if fetch_address < &memory.text_start {
        return Err("Program counter has reached unowned space. You, the user, have made a grave mistake.".to_string());
    }
    
    let instr_index: usize = (fetch_address - &memory.text_start) as usize;
    
    let fetched_instruction = u32::from_be_bytes(memory.text[instr_index..instr_index+4].try_into().unwrap());

    Ok(fetched_instruction)
}