// This file is responsible for the construction and management of symbol tables.

use name_core::{
    elf_def::{Elf, Elf32Sym},
    elf_utils::{find_target_section_index, parse_elf_symbols},
    structs::Symbol,
};

pub fn collect_global_symbols(elfs: &Vec<Elf>) -> Result<Vec<Elf32Sym>, String> {
    let mut global_symbols: Vec<Elf32Sym> = vec![];

    for elf in elfs {
        let shstrndx: usize = elf.file_header.e_shstrndx as usize;
        let symtab_index = match find_target_section_index(
            &elf.section_header_table,
            &elf.sections[shstrndx - 1],
            ".symtab",
        ) {
            Some(i) => i,
            None => unreachable!(), // Part of conformity checks to ensure we never get here
        };

        let symtab: Vec<Elf32Sym> = parse_elf_symbols(&elf.sections[symtab_index - 1]);
        for symbol in symtab {
            if symbol.get_bind() == 1 {
                // STB_GLOBAL
                global_symbols.push(symbol.clone());
            }
        }
    }

    return Ok(global_symbols);
}

pub fn collect_local_symbols(_elfs: &Vec<Elf>) -> Vec<Vec<Symbol>> {
    todo!("Collect local symbols");
}
