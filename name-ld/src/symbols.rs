// This file is responsible for the construction and management of symbol tables.

use name_core::{elf_def::Elf, structs::Symbol};

pub fn collect_global_symbols(_elfs: &Vec<Elf>) -> Result<Vec<Symbol>, String> {
    todo!("Make global symbol gatherer");
}

pub fn collect_local_symbols(_elfs: &Vec<Elf>) -> Vec<Vec<Symbol>> {
    todo!("Collect local symbols");
}