/// This file contains the linker logic. If only one file was provided, it will invoke the far simpler single module linker
use name_core::{elf_def::Elf, structs::Symbol};

use crate::{adjust::{adjust_global_symbols, adjust_local_symbols, generate_base_addresses}, conformity::conformity_check, relocate::relocate, symbols::{collect_global_symbols, collect_local_symbols}, update_header::update_header};

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
        return Ok(update_header(&elfs[0]));
    }
    
    // Else, must perform actual linking.

    // Calculate base addresses for each ELF's sections
    let base_addresses: Vec<u32> = generate_base_addresses(&elfs);

    // Construct External Symbol Dictionary (collect global symbols from each ELF).
    let collected_globals: Vec<Symbol> = collect_global_symbols(&elfs)?;
    
    // Relocate global symbols into external symbol dictionary (Calculate proper address based on new base addresses)
    let esd: Vec<Symbol> = adjust_global_symbols(collected_globals, &base_addresses);

    // Collect local symbol tables for each ELF
    let collected_locals: Vec<Vec<Symbol>> = collect_local_symbols(&elfs);

    // Adjust local symbols for new base addresses
    let adjusted_locals: Vec<Vec<Symbol>> = adjust_local_symbols(collected_locals, &base_addresses);

    // Relocate everything denoted by .rel.text and .rel.data based off adjusted local symbols
    // This step also consolidates sections under the hood
    let relocated_elf: Elf = relocate(&elfs, &esd, &adjusted_locals)?;

    // Update header values as last step
    let final_elf = update_header(&relocated_elf);

    // Return constructed ELF
    return Ok(final_elf);
}
