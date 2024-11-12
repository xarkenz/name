/// This file contains the linker logic. If only one file was provided, it will invoke the far simpler single module linker
use name_core::elf_def::{Elf, Elf32Sym};

use crate::{
    adjust::{adjust_symbols, generate_base_addresses},
    conformity::conformity_check,
    relocate::relocate,
    symbols::collect_global_symbols,
    update_header::update_header,
};

pub fn linker(elfs: Vec<Elf>) -> Result<Elf, String> {
    // Ensure each ELF conforms to the correct standard
    conformity_check(&elfs)?;

    // If just one file received, just need to perform trivial "relocation" and a header update
    if elfs.len() == 1 {
        /*
        // Question mark used to propagate but in this instance is infallible
        let collected_globals: Vec<Symbol> = collect_global_symbols(&elfs)?;
        let collected_locals: Vec<Vec<Symbol>> = collect_local_symbols(&elfs);

        let relocated_elf: Elf = relocate(&elfs, &collected_globals, &collected_locals)?;
        return Ok(update_header(&relocated_elf));
        */

        return Ok(update_header(
            &match relocate(&vec![elfs[0].clone()], &vec![]) {
                Ok(elf) => elf,
                Err(e) => panic!("{e}"),
            },
        ));
    }

    // Else, must perform actual linking.

    // Calculate base addresses for each ELF's sections
    let base_addresses: Vec<u32> = generate_base_addresses(&elfs);

    // Adjust symbols for new base addresses
    let adjusted_symtab_elfs: Vec<Elf> = adjust_symbols(elfs, &base_addresses);

    // Construct External Symbol Dictionary (collect global symbols from each ELF).
    let esd: Vec<Elf32Sym> = collect_global_symbols(&adjusted_symtab_elfs)?;

    // Relocate everything denoted by .rel.text and .rel.data based off adjusted local symbols
    // This step also consolidates sections under the hood
    let relocated_elf: Elf = relocate(&adjusted_symtab_elfs, &esd)?;

    // Update header values as last step
    let final_elf = update_header(&relocated_elf);

    // Return constructed ELF
    return Ok(final_elf);
}
