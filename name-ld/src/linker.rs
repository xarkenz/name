/// This file contains the linker logic. If only one file was provided, it will invoke the far simpler single module linker
use name_core::elf_def::Elf;

use crate::{conformity::conformity_check, update_header::update_header};

pub fn linker(elfs: Vec<Elf>) -> Result<Elf, String> {
    // Ensure each ELF conforms to the correct standard
    match conformity_check(&elfs) {
        Ok(_) => {}
        Err(e) => panic!("{e}"),
    };

    // If just one file received, all that needs to happen is a simple value update
    if elfs.len() == 1 {
        return Ok(update_header(elfs[0].clone()));
    }

    // If more than one ELF file received, must perform actual linking.
    todo!("Linking process");

    // Update values as last step
    // let final_elf = update_header(linked_elf);

    // Ok(final_elf)
}
