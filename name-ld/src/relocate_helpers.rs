// This file contains the helpers for the relocate function

use name_core::{elf_def::{Elf, ElfType}, elf_utils::{create_new_elf, parse_elf_symbols}};

/// This function is responsible for adjusting the symtab link indices. 
/// It's entirely based on the .strtab offsets.
pub fn relocate_symtab_links(elf: Elf, offsets: &Vec<Vec<u32>>) -> Elf {
    let symtab_offsets: Vec<u32> = offsets[4].clone();
    let mut current_offset: u32 = 0;
    let mut current_offset_idx: usize = 0;
    let mut previous_st_name: u32 = 0;

    // For each entry in the symbol table, if the symbol has a smaller index than the previous, or it's equal (at 0), it needs to add the offset.
    // IMPORTANT (POSSIBLE EDGE CASE MUST CHECK LATER): I think this can crash at runtime with some really weird files. 
    // Note the check for offset range. It doesn't seem watertight.
    let symbol_table = parse_elf_symbols(&elf.sections[4]);
    let new_symbol_table: Vec<u8> = symbol_table.iter().map(|symbol| {
        let mut new_symbol = symbol.clone();
        if symbol.st_name <= previous_st_name {
            // ^ DEBUG POINT IS THIS
            current_offset += symtab_offsets[current_offset_idx + 1];
            current_offset_idx += 1;
        }

        previous_st_name = symbol.st_name;
        new_symbol.st_name = current_offset + symbol.st_name;

        new_symbol.to_bytes()
    }).flatten().collect();

    // Make a new ELF with the new .symtab
    let new_sections: Vec<Vec<u8>> = elf.sections.iter().enumerate().map(|(idx, section)| match idx {
        4 => new_symbol_table.clone(),
        _ => section.clone(),
    }).collect();

    return create_new_elf(new_sections, ElfType::Relocatable);
}

/// This function checks first for duplicate global symbols, 
/// then it checks for duplicate local symbols in the same local space (based on calculated offsets).
pub fn check_duplicate_symbols(_elf: Elf, _offsets: &Vec<Vec<u32>>) -> Result<Elf, String> {
    todo!();
}

pub fn relocate_text_entries(_elf: Elf, _offsets: &Vec<Vec<u32>>) -> Elf {
    todo!();
}