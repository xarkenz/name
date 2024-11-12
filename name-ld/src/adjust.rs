use name_core::{
    elf_def::{Elf, Elf32Sym},
    elf_utils::{find_target_section_index, parse_elf_symbols},
};

// This file is responsible for calculating base addresses and for adjusting based off those addresses.

/// Calculate the new base addresses corresponding to each ELF
pub fn generate_base_addresses(elfs: &Vec<Elf>) -> Vec<u32> {
    let mut current_base_offset: u32 = 0;

    let mut base_offsets: Vec<u32> = vec![];
    for elf in elfs {
        base_offsets.push(current_base_offset);
        current_base_offset += elf.get_text_length();
    }

    return base_offsets;
}

/// Adjust global symbols based off calculated base addresses
pub fn adjust_symbols(elfs: Vec<Elf>, base_address_offsets: &Vec<u32>) -> Vec<Elf> {
    let mut collected_elfs: Vec<Elf> = vec![];

    // For each ELF:
    for (idx, elf) in elfs.into_iter().enumerate() {
        // Retrieve symbol table
        let shstrndx = elf.file_header.e_shstrndx;
        let symtab_index = match find_target_section_index(
            &elf.section_header_table,
            &elf.sections[shstrndx as usize - 1],
            ".symtab",
        ) {
            Some(i) => i,
            None => unreachable!(), // Passed checks to get here
        };

        // Parse existing symtab
        let symtab: Vec<Elf32Sym> = parse_elf_symbols(&elf.sections[symtab_index - 1]);

        // Initialize new one with reserved zero element
        let mut new_symtab: Vec<Elf32Sym> = vec![Elf32Sym {
            st_name: 0,
            st_value: 0,
            st_size: 0,
            st_info: 0,
            st_other: 0,
            st_shndx: 0,
        }];

        // Create new symtab (adjusted for this ELF's base offset)
        for symbol in symtab {
            // If the value is zero, do not adjust it. This is a placeholder value.
            match symbol.st_value {
                0 => new_symtab.push(symbol.clone()),
                _ => new_symtab.push(Elf32Sym {
                    st_name: symbol.st_name,
                    st_value: symbol.st_value + base_address_offsets[idx],
                    st_size: symbol.st_size,
                    st_info: symbol.st_info,
                    st_other: symbol.st_other,
                    st_shndx: symbol.st_shndx,
                }),
            };
        }

        // Serialize the new symbol table to bytes.
        let serialized_new_symtab: Vec<u8> = new_symtab
            .into_iter()
            .map(|sym| sym.to_bytes())
            .flatten()
            .collect();

        // Splice the new symtab into a cloned version of the ELF
        let mut cloned_elf: Elf = elf.clone();
        cloned_elf.sections[symtab_index - 1] = serialized_new_symtab;

        // Add modified ELF to collection
        collected_elfs.push(cloned_elf);
    }

    // Return all the modified ELFs
    return collected_elfs;
}
