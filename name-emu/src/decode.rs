use name_const::structs::{Memory, Processor};

use crate::definitions::lookup_tables::{FUNCT_TABLE, OPCODE_TABLE};
use crate::definitions::structs::ExecutionStatus;

pub type InstructionFn = fn(&mut Processor, &mut Memory, u32) -> Result<ExecutionStatus, String>;

pub fn decode(instruction: &u32) -> Result<InstructionFn, String> {
    let opcode: usize = (instruction >> 26) as usize;

    match opcode {
        0 => {
            // If opcode is 0 this is an R-type instruction and needs a different lookup
            let funct: usize = (instruction & 0x3F) as usize;

            match FUNCT_TABLE[funct] {
                Some(fun) => return Ok(fun),
                None => return Err(format!("Failed to fetch R-type instruction with funct: {funct}")),
            }
        },
        _ => match OPCODE_TABLE[opcode] {
            Some(fun) => return Ok(fun),
            None => return Err(format!("Failed to decode instruction opcode: {opcode}")),
        }
    }
}

pub fn unpack_r_type(instruction: u32) -> (usize, usize, usize, usize) {
    let rs: u32 = (instruction >> 21) & 0x1F;
    let rt: u32 = (instruction >> 16) & 0x1F;
    let rd: u32 = (instruction >> 11) & 0x1F;
    let shamt: u32 = (instruction >> 6) & 0x1F;

    (rd as usize, rs as usize, rt as usize, shamt as usize)
}

pub fn unpack_i_type(instruction: u32) -> (usize, usize, u32) {
    let rs: u32 = (instruction >> 21) & 0x1F;
    let rt: u32 = (instruction >> 16) & 0x1F;
    let imm: u32 = instruction & 0xFFFF;

    (rs as usize, rt as usize, imm)
}

pub fn unpack_j_type(instruction: u32) -> u32 {
    let imm: u32 = instruction & 0x03FFFFFF;

    imm
}