use crate::structs::{ArgumentType, InstructionInformation, InstructionType};

const NUM_OF_REGISTERS: usize = 32;

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

pub const INSTRUCTION_SET: &[InstructionInformation] = &[
    InstructionInformation {
        mnemonic: "add",
        instruction_type: InstructionType::RType,
        opcode: None,
        funct: Some(32),
        args: &[ArgumentType::Rd, ArgumentType::Rs, ArgumentType::Rt],
        alt_args: None,
    },
    InstructionInformation {
        mnemonic: "addi",
        instruction_type: InstructionType::IType,
        opcode: Some(8),
        funct: None,
        args: &[ArgumentType::Rt, ArgumentType::Rs, ArgumentType::Immediate],
        alt_args: None,
    },
    InstructionInformation {
        mnemonic: "beq",
        instruction_type: InstructionType::IType,
        opcode: Some(4),
        funct: None,
        args: &[ArgumentType::Rs, ArgumentType::Rt, ArgumentType::BranchLabel],
        alt_args: None,
    },
    InstructionInformation {
        mnemonic: "j",
        instruction_type: InstructionType::JType,
        opcode: Some(2),
        funct: None,
        args: &[ArgumentType::BranchLabel],
        alt_args: None,
    },
    InstructionInformation {
        mnemonic: "jal",
        instruction_type: InstructionType::JType,
        opcode: Some(3),
        funct: None,
        args: &[ArgumentType::BranchLabel],
        alt_args: None,
    },
    InstructionInformation {
        mnemonic: "lb",
        instruction_type: InstructionType::IType,
        opcode: Some(32),
        funct: None,
        args: &[ArgumentType::Rt, ArgumentType::Immediate, ArgumentType::Rs],
        alt_args: Some(&[
            &[ArgumentType::Rt, ArgumentType::Rs],
            &[ArgumentType::Rt, ArgumentType::Identifier, ArgumentType::Rs],
        ]),
    },
    InstructionInformation {
        mnemonic: "lui",
        instruction_type: InstructionType::IType,
        opcode: Some(15),
        funct: None,
        args: &[ArgumentType::Rt, ArgumentType::Immediate],
        alt_args: None,
    },
    InstructionInformation {
        mnemonic: "ori",
        instruction_type: InstructionType::IType,
        opcode: Some(13),
        funct: None,
        args: &[ArgumentType::Rs, ArgumentType::Rt, ArgumentType::Immediate],
        alt_args: None,
    },
    InstructionInformation {
        mnemonic: "sll",
        instruction_type: InstructionType::RType,
        opcode: None,
        funct: Some(0),
        args: &[ArgumentType::Rd, ArgumentType::Rt, ArgumentType::Immediate],
        alt_args: None,
    },
    InstructionInformation {
        mnemonic: "srl",
        instruction_type: InstructionType::RType,
        opcode: None,
        funct: Some(2),
        args: &[ArgumentType::Rd, ArgumentType::Rt, ArgumentType::Immediate],
        alt_args: None,
    },
    InstructionInformation {
        mnemonic: "sub",
        instruction_type: InstructionType::RType,
        opcode: None,
        funct: Some(34),
        args: &[ArgumentType::Rd, ArgumentType::Rs, ArgumentType::Rt],
        alt_args: None,
    },
    InstructionInformation {
        mnemonic: "syscall",
        instruction_type: InstructionType::RType,
        opcode: None,
        funct: Some(12),
        args: &[],
        alt_args: None,
    },
    InstructionInformation {
        mnemonic: "xor",
        instruction_type: InstructionType::RType,
        opcode: None,
        funct: Some(38),
        args: &[ArgumentType::Rd, ArgumentType::Rs, ArgumentType::Rt],
        alt_args: None,
    },
];