use name_const::elf_utils::convert_symbol_to_elf32sym;
use name_const::elf_def::Elf32Sym;

// Extract the symbol table to .symtab and .strtab sections. Needed to properly write to ELF files.
pub fn extract_symbol_table_to_sections(symbol_table: Vec<name_const::structs::Symbol>) -> (Vec<u8>, Vec<u8>) {
    let mut symtab: Vec<u8> = Elf32Sym{ st_name: 0, st_value: 0, st_size: 0, st_info: 0, st_other: 0, st_shndx: 0 }.to_bytes();
    let mut strtab: Vec<u8> = vec![0];

    let mut strtab_index: u32 = 1;

    for symbol in symbol_table {
        symtab.extend(convert_symbol_to_elf32sym(&symbol, strtab_index).to_bytes());

        strtab.extend_from_slice(&symbol.identifier.as_bytes());
        strtab.extend_from_slice(b"\0");

        strtab_index += (symbol.identifier.len() + 1) as u32;
    }

    return (symtab, strtab);
}