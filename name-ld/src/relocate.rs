// This file is responsible for the relocation of symbols.

use name_core::{elf_def::{Elf, ElfType}, elf_utils::create_new_elf, structs::Symbol};

pub fn relocate(elfs: &Vec<Elf>, _external_symbols: &Vec<Symbol>, _internal_symbols: &Vec<Vec<Symbol>>) -> Result<Elf, String> {
    // Consolidate sections
    // .data
    let mut consolidated_data: Vec<u8> = vec!();
    for elf in elfs {
        consolidated_data.extend(
            elf.sections[0].clone()
        );
        consolidated_data.extend(
            // Zero-extend to remove alignment issues
            vec!(0; 4 - (elf.sections[0].len() % 4))
        );
    }
    
    // .text
    let consolidated_text: Vec<u8> = elfs.iter().map(|elf| elf.sections[1].clone()).flatten().collect();
    // skip .rel
    // .symtab
    let consolidated_symtab: Vec<u8> = elfs.iter().map(|elf| elf.sections[3].clone()).flatten().collect();
    // .strtab
    let consolidated_strtab: Vec<u8> = elfs.iter().map(|elf| elf.sections[4].clone()).flatten().collect();
    // .line
    let consolidated_line: Vec<u8> = elfs.iter().map(|elf| elf.sections[5].clone()).flatten().collect();

    
    if elfs.len() == 1 {
        // TODO BUT LIKE BIG TIME: perform trivial "relocation" once .rel.text and .rel.data have been introduced
        return Ok(create_new_elf(
            vec!(consolidated_data, consolidated_text, consolidated_symtab, consolidated_strtab, consolidated_line),
            ElfType::Executable
        ));
    }

    todo!("Perform consolidation and relocation");
}