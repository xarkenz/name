use name_const::constants::REGISTERS;
use name_const::structs::Symbol;

use crate::constants::structs::{ArgumentType, LineComponent};
use crate::constants::instructions::INSTRUCTION_SET;
use crate::constants::structs::InstructionInformation;

use std::collections::HashMap;

// Helper function for assemble_instruction for use when multiple argument configurations are available.
// Checks argument configuration against what was passed.
// Returns a boolean value representing whether the expected fields matched or not.
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

// Oft-used map operation nobody would want to repeat. Turns a symbol table entry into its address.
pub fn translate_identifier_to_address(identifier: &String, symbol_table: &Vec<Symbol>) -> Option<u32> {
    symbol_table.iter().find(|symbol| symbol.identifier == identifier.clone()).map(|symbol| symbol.value)
}

// Parse a register string like "$t0" or "$3" to u32 for packing.
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

pub fn get_mnemonics() -> Vec<String> {
    let mut mnemonics: Vec<String> = vec!();
    
    for instruction in INSTRUCTION_SET {
        mnemonics.push(instruction.get_mnemonic());
    }

    return mnemonics;
}

pub fn generate_instruction_hashmap() -> HashMap<String, &'static InstructionInformation> {
    let mut hashmap: HashMap<String, &'static InstructionInformation> = HashMap::new();

    for instruction in INSTRUCTION_SET {
        hashmap.insert(instruction.mnemonic.to_string(), &instruction);
    }

    return hashmap;
}

/*
Pretty print an instruction in the format:

 - 0xDEADBEEF
 - 0b11011110101011011011111011101111

*/
pub fn pretty_print_instruction(packed: &u32){
    println!(" - 0x{:08x}", packed);
    println!(" - 0b{:032b}", packed);
    println!();
}