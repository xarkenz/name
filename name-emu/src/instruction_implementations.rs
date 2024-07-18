use crate::decode::{unpack_r_type, unpack_i_type, unpack_j_type};
use crate::lookup_tables::SYSCALL_TABLE;
use name_const::structs::{Processor, Memory};

const AS_TEMP: usize = 1;        // Want to access assembler temporary without remembering what register 1 is? Boy, do I have a solution for you!
const V0: usize = 2;
const RA: usize = 31;

/*

  ______ _    _ _   _  _____ _______ 
 |  ____| |  | | \ | |/ ____|__   __|
 | |__  | |  | |  \| | |       | |   
 |  __| | |  | | . ` | |       | |   
 | |    | |__| | |\  | |____   | |   
 |_|     \____/|_| \_|\_____|  |_|   
                                     
                                     

*/

// 0x00 - sll
pub fn sll(cpu: &mut Processor, _memory: &mut Memory, instruction: u32) -> Result<bool, String> {
    let (rd, _, rt, shamt) = unpack_r_type(instruction);
    cpu.general_purpose_registers[rd] = cpu.general_purpose_registers[rt] << shamt;
    Ok(false)
}

// 0x02 - srl
pub fn srl(cpu: &mut Processor, _memory: &mut Memory, instruction: u32) -> Result<bool, String> {
    let (rd, _, rt, shamt) = unpack_r_type(instruction);
    cpu.general_purpose_registers[rd] = cpu.general_purpose_registers[rt] >> shamt;
    Ok(false)
}

// 0x0C - syscall
pub fn syscall(cpu: &mut Processor, memory: &mut Memory, _instruction: u32) -> Result<bool, String> {
    let syscall_num: usize = cpu.general_purpose_registers[V0] as usize;
    match SYSCALL_TABLE[syscall_num] {
        Some(fun) => fun(cpu, memory),
        None => return Err(format!("Syscall {} is not implemented.", syscall_num)),
    }
}

// 0x20 - add
pub fn add(cpu: &mut Processor, _memory: &mut Memory, instruction: u32) -> Result<bool, String> {
    let (rd, rs, rt, _) = unpack_r_type(instruction);
    cpu.general_purpose_registers[rd] = cpu.general_purpose_registers[rs] + cpu.general_purpose_registers[rt];
    Ok(false)
}

// 0x22 - sub
pub fn sub(cpu: &mut Processor, _memory: &mut Memory, instruction: u32) -> Result<bool, String> {
    let (rd, rs, rt, _) = unpack_r_type(instruction);

    let temp: (u32, bool) = cpu.general_purpose_registers[rs].overflowing_sub(cpu.general_purpose_registers[rt]);

    cpu.general_purpose_registers[AS_TEMP] = temp.0;

    if temp.1 {
        // TODO: Implement coprocessor 0 and signal integer overflow
        return Err(format!("Integer underflow occurred in subtraction."));
    } else {
        cpu.general_purpose_registers[rd] = cpu.general_purpose_registers[AS_TEMP];
    }

    Ok(false)
}

// 0x26 - xor
pub fn xor(cpu: &mut Processor, _memory: &mut Memory, instruction: u32) -> Result<bool, String> {
    let (rd, rs, rt, _) = unpack_r_type(instruction);
    cpu.general_purpose_registers[rd] = cpu.general_purpose_registers[rs] ^ cpu.general_purpose_registers[rt];
    Ok(false)
}

/*

   ____  _____   _____ ____  _____  ______ 
  / __ \|  __ \ / ____/ __ \|  __ \|  ____|
 | |  | | |__) | |   | |  | | |  | | |__   
 | |  | |  ___/| |   | |  | | |  | |  __|  
 | |__| | |    | |___| |__| | |__| | |____ 
  \____/|_|     \_____\____/|_____/|______|
                                           
                                           

*/

// 0x02 - j
pub fn j(cpu: &mut Processor, memory: &mut Memory, instruction: u32) -> Result<bool, String> {
    let address: u32 = (unpack_j_type(instruction) << 2) | (cpu.pc & 0xF0000000);

    if address >= memory.text_end || address < memory.text_start {
        return Err(format!("Attempted to jump to unowned address 0x{:x}", address));
    } else {
        cpu.pc = address;
    }

    Ok(false)
}

// 0x03 - jal
pub fn jal(cpu: &mut Processor, memory: &mut Memory, instruction: u32) -> Result<bool, String> {
    let address: u32 = (unpack_j_type(instruction) << 2) | (cpu.pc & 0xF0000000);

    if address >= memory.text_end || address < memory.text_start {
        return Err(format!("Attempted to jump to unowned address 0x{:x}", address));
    } else {
        cpu.general_purpose_registers[RA] = cpu.pc;
        cpu.pc = address;
    }

    Ok(false)
}

// 0x04 - beq
pub fn beq(cpu: &mut Processor, memory: &mut Memory, instruction: u32) -> Result<bool, String> {
    let (rs, rt, imm) = unpack_i_type(instruction);

    // Sign extend offset
    let offset: i32 = ((imm & 0xFFFF) as i16 as i32) << 2;

    if cpu.general_purpose_registers[rs] != cpu.general_purpose_registers[rt] {
        return Ok(false)
    }

    cpu.general_purpose_registers[AS_TEMP] = (cpu.pc as i32 + offset) as u32;

    if cpu.general_purpose_registers[AS_TEMP] >= memory.text_end || cpu.general_purpose_registers[AS_TEMP] < memory.text_start {
        return Err(format!("Attempted to access unowned address 0x{:x}", cpu.general_purpose_registers[AS_TEMP]));
    }
    
    Ok(false)
}

// 0x0D - ori
pub fn ori(cpu: &mut Processor, _memory: &mut Memory, instruction: u32) -> Result<bool, String> {
    let (rs, rt, imm) = unpack_i_type(instruction);
    cpu.general_purpose_registers[rt] = cpu.general_purpose_registers[rs] | imm;
    Ok(false)
}

// 0x0F - lui
pub fn lui(cpu: &mut Processor, _memory: &mut Memory, instruction: u32) -> Result<bool, String> {
    let (_, rt, imm) = unpack_i_type(instruction);
    cpu.general_purpose_registers[rt] = imm << 16;
    Ok(false)
}

// 0x20 - lb
pub fn lb(cpu: &mut Processor, memory: &mut Memory, instruction: u32) -> Result<bool, String> {
    let (rs, rt, imm) = unpack_i_type(instruction);

    cpu.general_purpose_registers[AS_TEMP] = (cpu.general_purpose_registers[rs] as i32 + imm as i32) as u32;

    if cpu.general_purpose_registers[AS_TEMP] >= memory.data_end || cpu.general_purpose_registers[AS_TEMP] < memory.data_start {
        return Err(format!("Attempted to access unowned address 0x{:x}", cpu.general_purpose_registers[AS_TEMP]));
    } else {
        cpu.general_purpose_registers[rt] = memory.data[(cpu.general_purpose_registers[AS_TEMP] - memory.data_start) as usize] as u32;
    }

    Ok(false)
}