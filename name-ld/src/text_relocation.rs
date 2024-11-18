// This file is just responsible for performing .text relocation. That's it.

use name_core::elf_def::Elf;

/// This file will relocate the .text entries. This means resolving symbols.
pub fn relocate_text_entries(_relocated_symtab: Elf, _offsets: &Vec<Vec<u32>>) -> Elf {
    todo!()
}