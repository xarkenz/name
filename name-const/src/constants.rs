use crate::structs::{ArgumentType, InstructionInformation, InstructionType};

const NUM_OF_REGISTERS: usize = 32;
const NUM_OF_IMPLEMENTED_INSTRUCTIONS: usize = 1;

pub const REGISTERS: [&'static str; NUM_OF_REGISTERS] = [
    "$zero", 
    "$at", 
    "$v0", "$v1", 
    "$a0", "$a1", "$a2", "$a3",
    "$t0", "$t1", "$t2", "$t3", "$t4", "$t5", "$t6", "$t7",
    "$s0", "$s1", "$s2", "$s3", "$s4", "$s5", "$s6", "$s7",
    "$t8", "$t9", 
    "$k0", "$k1", 
    "$gp", "$sp", "$fp", 
    "$ra"
];

pub const INSTRUCTION_SET: [InstructionInformation; NUM_OF_IMPLEMENTED_INSTRUCTIONS] = [
    InstructionInformation {
        mnemonic: "add",
        instruction_type: InstructionType::RType,
        shamt: 0,
        funct: 0,
        args: &[ArgumentType::Rd, ArgumentType::Rs, ArgumentType::Rt],
    },
];