/// This file contains the linker logic. If only one file was provided, it will invoke the far simpler single module linker
use name_core::elf_def::Elf;

use crate::conformity::conformity_check;

pub fn linker(elfs: Vec<Elf>) -> Result<Elf, String> {
    // Ensure each ELF conforms to the correct standard
    conformity_check(&elfs)?;

    todo!("Linker linker linker");
    // Let's architect with comments!
}