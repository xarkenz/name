use crate::constants::INSTRUCTION_SET;
use crate::structs::InstructionInformation;

use std::collections::HashMap;

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