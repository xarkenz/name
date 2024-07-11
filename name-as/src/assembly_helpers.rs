use name_const::constants::REGISTERS;
use name_const::structs::{ArgumentType, LineComponent, Symbol};
use name_const::elf_utils::convert_symbol_to_elf32sym;
use name_const::elf_def::Elf32Sym;

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

// Helper function for assemble_instruction for use when multiple argument configurations are available.
// Checks argument configuration against what was passed.
pub fn arg_configuration_is_ok(passed_args: &Vec<LineComponent>, expected_args: &[ArgumentType]) -> bool {
    if passed_args.len() != expected_args.len() {
        return false;
    }

    for (passed, expected) in passed_args.iter().zip(expected_args.iter()) {
        match (passed, expected) {
            (LineComponent::Register(_), ArgumentType::Rd) |
            (LineComponent::Register(_), ArgumentType::Rs) |
            (LineComponent::Register(_), ArgumentType::Rt) |
            (LineComponent::Immediate(_), ArgumentType::Immediate) |
            (LineComponent::Identifier(_), ArgumentType::Identifier) | 
            (LineComponent::Identifier(_), ArgumentType::BranchLabel) => {},
            _ => return false,
        }
    }

    return true;
}

pub fn translate_identifier_to_address(identifier: &String, symbol_table: &Vec<Symbol>) -> Option<u32> {
    symbol_table.iter().find(|symbol| symbol.identifier == identifier.clone()).map(|symbol| symbol.value)
}

pub fn parse_register_to_u32(register: &String) -> Result<u32, String> {
    // Check the early exit
    if !register.starts_with("$") { 
        return Err("Register parse failed.".to_string());
    }

    // First, try a simple lookup on the REGISTERS constant.
    if let Some(index) = REGISTERS.iter().position(|&x| x == register){
        return Ok(index as u32);
    } else if let Ok(attempted_direct_parse) = register.chars().skip(1).collect::<String>().parse::<u32>(){
        // This line looks like wizard stuff but really I'm just removing the first char from the string by
        // using an iterator, skipping an item, and collecting everything else back together
        // This is for registers given like '$0' and '$3'
        return Ok(attempted_direct_parse);
    } else {
        return Err("Register parse failed".to_string());
    }
}

pub fn pretty_print_instruction(packed: &u32){
    println!(" - 0x{:08x}", packed);
    println!(" - 0b{:032b}", packed);
    println!();
}