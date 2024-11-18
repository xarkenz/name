// This file contains the helpers for the relocate function

use name_core::elf_def::Elf;

pub fn relocate_symtab_links(_elf: &mut Elf, _offsets: &Vec<Vec<u32>>) -> () {
    todo!();
}

pub fn check_duplicate_symbols(_elf: &Elf) -> Result<(), String> {
    todo!();
}

pub fn relocate_text_entries(_elf: &mut Elf, _offsets: &Vec<Vec<u32>>) -> () {
    todo!();
}