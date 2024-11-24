// This file is just responsible for performing .text relocation. That's it.

use name_core::{
    elf_def::{Elf, Elf32Sym, ElfType, RelocationEntry, RelocationEntryType}, elf_utils::{create_new_elf, parse_elf_symbols, parse_rel_info}
};

/// Custom error type
pub enum TextRelocationError {
    UndefinedSymbol(String),
    UnimplementedRelType(RelocationEntryType),
}

/// Pretty print for that error type
impl std::fmt::Display for TextRelocationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TextRelocationError::UndefinedSymbol(name) => {
                write!(f, "Symbol {name} not found in any scope")
            },
            TextRelocationError::UnimplementedRelType(rel_type) => {
                write!(f, "Relocation type {rel_type:?} not yet implmented")
            }
        }
    }
}

/// This function will relocate the .text entries. This means resolving branch symbols and such!.
pub fn relocate_text_entries(
    adjusted_checked_elf: Elf,
    _offsets: &Vec<Vec<u32>>,
) -> Result<Elf, TextRelocationError> {
    // For each relocation entry in .rel, match on the type and perform the necessary relocation in .text.
    let mut new_text_section: Vec<u8> = adjusted_checked_elf.sections[0].clone();
    let symbol_table: Vec<Elf32Sym> = parse_elf_symbols(&adjusted_checked_elf.sections[3]);
    let string_table: Vec<u8> = adjusted_checked_elf.sections[4].clone();

    // For each relocation entry, relocate as instructed.
    let relocation_entries: Vec<RelocationEntry> = parse_rel_info(&adjusted_checked_elf.sections[2]);
    for entry in relocation_entries {
        let linked_symbol: Elf32Sym = match get_linked_symbol(&symbol_table, &string_table, entry.r_sym as usize >> 5) {
            Some(sym) => sym,
            None => return Err(TextRelocationError::UndefinedSymbol(symbol_table[entry.r_sym as usize >> 5].get_linked_name(&adjusted_checked_elf.sections[4]))),
        };

        // Match on the relocation type and perform the corresponding relocation.
        match entry.r_type {
            RelocationEntryType::Pc16 => {
                // For branch instructions:
                let symbol_address: u32 = linked_symbol.st_value;
                let pc_rel: u32 = entry.r_offset;
                let relocation_value: u32 = (((symbol_address as i32) - (pc_rel as i32)) as u32) >> 2;
                let text_offset: usize = entry.r_offset as usize;
                let old_value: u32 = u32::from_be_bytes(new_text_section[text_offset..(text_offset + 3)].try_into().unwrap());
                let new_value: u32 = old_value | relocation_value;
                new_text_section.splice(text_offset..(text_offset + 3), new_value.to_be_bytes());
            }
            RelocationEntryType::R26 => {
                // For jump instructions:
                let address_to_pack: u32 = linked_symbol.st_value >> 2;
                let text_offset: usize = entry.r_offset as usize;
                let old_value: u32 = u32::from_be_bytes(new_text_section[text_offset..(text_offset + 3)].try_into().unwrap());
                let new_value: u32 = old_value | address_to_pack;
                new_text_section.splice(text_offset..(text_offset + 3), new_value.to_be_bytes());
            },
            _ => return Err(TextRelocationError::UnimplementedRelType(entry.r_type)),
        }
    }

    // Return an executable ELF (ditch the relocation information once done with it)
    let exec_sections: Vec<Vec<u8>> = adjusted_checked_elf
        .sections
        .iter()
        .enumerate()
        .filter_map(|(idx, section)| match idx {
            0 => Some(new_text_section.clone()),
            2 => None,
            _ => Some(section.clone()),
        })
        .collect();
    Ok(create_new_elf(exec_sections, ElfType::Executable))
}


/// This function gets the correct linked symbol for a relocation entry. It looks to the local scope first by design.
fn get_linked_symbol(symtab: &Vec<Elf32Sym>, strtab: &Vec<u8>, symbol_idx: usize) -> Option<Elf32Sym> {
    match symtab[symbol_idx].st_value {
        0 => {
            let name_to_match: String = symtab[symbol_idx].get_linked_name(strtab);
            return match symtab.iter().find(|symbol| symbol.get_linked_name(strtab) == name_to_match && symbol.get_bind() == 1) {
                Some(sym) => Some(sym.clone()),
                None => None,
            };
        },
        _ => return Some(symtab[symbol_idx as usize].clone()),
    }
}
