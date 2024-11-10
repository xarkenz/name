use name_core::constants::MIPS_TEXT_START_ADDR;
use name_core::elf_def::{Elf, ET_EXEC};
use name_core::elf_utils::{find_global_symbol_address, parse_elf_symbols};

/// Note that this function is not fallible in the context we are using it.
/// The et_rel parameter is pre-checked.
pub fn update_header(et_rel: &Elf) -> Elf {
    let mut et_exec = et_rel.clone();

    // Update executable type (needed for name-emu to recognize file)
    et_exec.file_header.e_type = ET_EXEC;

    // Increment executable version
    et_exec.file_header.e_version += 1;

    // Update entry address
    et_exec.file_header.e_entry = MIPS_TEXT_START_ADDR;

    // If the global symbol "main" exists, that should be the entry address. It still needs to be global even in one module by convention.

    // Construct symbol table and corresponding string table from what was given
    let symbol_table = parse_elf_symbols(&et_exec.sections[2]);
    let str_table = &et_exec.sections[3];

    et_exec.file_header.e_entry = match find_global_symbol_address(&symbol_table, str_table, "main")
    {
        Some(new_address) => new_address,
        None => MIPS_TEXT_START_ADDR,
    };

    return et_exec;
}
