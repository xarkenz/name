// This file is responsible for the relocation of symbols.

use name_core::{elf_def::Elf, structs::Symbol};

pub fn relocate(elfs: &Vec<Elf>, _external_symbols: &Vec<Symbol>, _internal_symbols: &Vec<Vec<Symbol>>) -> Result<Elf, String> {
    if elfs.len() == 1 {
        // TODO BUT LIKE BIG TIME: perform trivial "relocation" once .rel.text and .rel.data have been introduced
        return Ok(elfs[0].clone());
    }

    todo!("Perform consolidation and relocation");
}