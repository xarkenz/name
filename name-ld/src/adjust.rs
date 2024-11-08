use name_core::{elf_def::Elf, structs::Symbol};

// This file is responsible for calculating base addresses and for adjusting based off those addresses.

/// Calculate the new base addresses corresponding to each ELF
pub fn generate_base_addresses(_elfs: &Vec<Elf>) -> Vec<u32> {
    todo!("Calculate new base addresses");
}

/// Adjust global symbols based off calculated base addresses
pub fn adjust_global_symbols(_raw_globals: Vec<Symbol>, _base_addresses: &Vec<u32>) -> Vec<Symbol> {
    todo!("Adjust global symbols");
}

pub fn adjust_local_symbols(_raw_locals: Vec<Vec<Symbol>>, _base_addresses: &Vec<u32>) -> Vec<Vec<Symbol>> {
    todo!("Adjust local symbols");
}