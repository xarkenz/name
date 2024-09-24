use crate::decode::{unpack_r_type, unpack_i_type, unpack_j_type};
use crate::definitions::lookup_tables::SYSCALL_TABLE;
use clap::builder::{Str, StringValueParser};
use name_const::structs::{Processor, Memory};

use crate::definitions::structs::ExecutionStatus;

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
pub fn sll(cpu: &mut Processor, _memory: &mut Memory, instruction: u32) -> Result<ExecutionStatus, String> {
    let (rd, _, rt, shamt) = unpack_r_type(instruction);
    cpu.general_purpose_registers[rd] = cpu.general_purpose_registers[rt] << shamt;
    Ok(ExecutionStatus::Continue)
}

// 0x02 - srl
pub fn srl(cpu: &mut Processor, _memory: &mut Memory, instruction: u32) -> Result<ExecutionStatus, String> {
    let (rd, _, rt, shamt) = unpack_r_type(instruction);
    cpu.general_purpose_registers[rd] = cpu.general_purpose_registers[rt] >> shamt;
    Ok(ExecutionStatus::Continue)
}

// 0x08 - jr
pub fn jr(cpu: &mut Processor, memory: &mut Memory, instruction: u32) -> Result<ExecutionStatus, String> {
    let (rs, _, _, _) = unpack_r_type(instruction);

    if cpu.general_purpose_registers[rs] >= memory.text_end || cpu.general_purpose_registers[rs] < memory.text_start {
        return Err(format!("Attempted to jump to unowned address 0x{:x}", cpu.general_purpose_registers[rs]));
    }
    
    cpu.pc = cpu.general_purpose_registers[rs];

    Ok(ExecutionStatus::Continue)
}

// 0x09 - jalr
pub fn jalr(cpu: &mut Processor, memory: &mut Memory, instruction: u32) -> Result<ExecutionStatus, String> {
    let (rs, _, _, _) = unpack_r_type(instruction);
    if cpu.general_purpose_registers[rs] >= memory.text_end || cpu.general_purpose_registers[rs] < memory.text_start {
        return Err(format!("Attempted to jump to unowned address 0x{:x}", cpu.general_purpose_registers[rs]));
    }
    cpu.general_purpose_registers[RA] = cpu.pc;
    cpu.pc = cpu.general_purpose_registers[rs];

    Ok(ExecutionStatus::Continue)
}

// 0x0C - syscall
pub fn syscall(cpu: &mut Processor, memory: &mut Memory, _instruction: u32) -> Result<ExecutionStatus, String> {
    let syscall_num: usize = cpu.general_purpose_registers[V0] as usize;
    match SYSCALL_TABLE[syscall_num] {
        Some(fun) => fun(cpu, memory),
        None => return Err(format!("Syscall {} is not implemented", syscall_num)),
    }
}

// 0x20 - add
pub fn add(cpu: &mut Processor, _memory: &mut Memory, instruction: u32) -> Result<ExecutionStatus, String> {
    let (rd, rs, rt, _) = unpack_r_type(instruction);
    cpu.general_purpose_registers[rd] = cpu.general_purpose_registers[rs] + cpu.general_purpose_registers[rt];
    
    // println!("Adding ${}({}) to ${}({}) and storing in ${}, now it's {}", rs, cpu.general_purpose_registers[rs], rt, cpu.general_purpose_registers[rt], rd, cpu.general_purpose_registers[rs] + cpu.general_purpose_registers[rt]);
    
    Ok(ExecutionStatus::Continue)
}

// 0x21 - addu
pub fn addu(cpu: &mut Processor, _memory: &mut Memory, instruction: u32) -> Result<ExecutionStatus, String> {
    let (rd, rs, rt, _) = unpack_r_type(instruction);
    // check that below works
    cpu.general_purpose_registers[rd] = cpu.general_purpose_registers[rs].overflowing_add(cpu.general_purpose_registers[rt]).0;
    Ok(ExecutionStatus::Continue)
}

// 0x22 - sub
pub fn sub(cpu: &mut Processor, _memory: &mut Memory, instruction: u32) -> Result<ExecutionStatus, String> {
    let (rd, rs, rt, _) = unpack_r_type(instruction);

    let temp: (u32, bool) = cpu.general_purpose_registers[rs].overflowing_sub(cpu.general_purpose_registers[rt]);

    cpu.general_purpose_registers[AS_TEMP] = temp.0;

    if temp.1 {
        // TODO: Implement coprocessor 0 and signal integer overflow
        
        return Err(format!("Integer underflow occurred in subtraction."));
    } else {
        cpu.general_purpose_registers[rd] = cpu.general_purpose_registers[AS_TEMP];
    }

    Ok(ExecutionStatus::Continue)
}

// 0x23 - subu
/* pub fn subu(cpu: &mut Processor, _memory: &mut Memory, instruction: u32) -> Result<ExecutionStatus, String> {
    let (rd, rs, rt, _) = unpack_r_type(instruction);

    let temp: (u32, bool) = cpu.general_purpose_registers[rs].overflowing_sub(cpu.general_purpose_registers[rt]);

    cpu.general_purpose_registers[AS_TEMP] = temp.0;

    if temp.1 {
        // TODO: Implement coprocessor 0 and signal integer overflow
        return Err(format!("Integer underflow occurred in subtraction."));
    } else {
        cpu.general_purpose_registers[rd] = cpu.general_purpose_registers[AS_TEMP];
    }

    Ok(ExecutionStatus::Continue)
} */

// 0x24 - and
pub fn and(cpu: &mut Processor, _memory: &mut Memory, instruction: u32) -> Result<ExecutionStatus, String>{
    let (rd, rs, rt, _) = unpack_r_type(instruction);
    cpu.general_purpose_registers[rd] = cpu.general_purpose_registers[rs] & cpu.general_purpose_registers[rt];
    Ok(ExecutionStatus::Continue)
}

// 0x25 - or
pub fn or(cpu: &mut Processor, _memory: &mut Memory, instruction: u32) -> Result<ExecutionStatus, String> {
    let (rd, rs, rt, _) = unpack_r_type(instruction);
    cpu.general_purpose_registers[rd] = cpu.general_purpose_registers[rs] | cpu.general_purpose_registers[rt];
    Ok(ExecutionStatus::Continue)
}

// 0x26 - xor
pub fn xor(cpu: &mut Processor, _memory: &mut Memory, instruction: u32) -> Result<ExecutionStatus, String> {
    let (rd, rs, rt, _) = unpack_r_type(instruction);
    cpu.general_purpose_registers[rd] = cpu.general_purpose_registers[rs] ^ cpu.general_purpose_registers[rt];
    Ok(ExecutionStatus::Continue)
}

// 0x27 - nor
pub fn nor(cpu: &mut Processor, _memory: &mut Memory, instruction: u32) -> Result<ExecutionStatus, String>{
    let (rd, rs, rt, _) = unpack_r_type(instruction);
    cpu.general_purpose_registers[rd] = !(cpu.general_purpose_registers[rs] | cpu.general_purpose_registers[rt]);
    Ok(ExecutionStatus::Continue)
}

// 0x2A - slt
pub fn slt(cpu: &mut Processor, _memory: &mut Memory, instruction: u32) -> Result<ExecutionStatus, String>{
    let (rd, rs, rt, _) = unpack_r_type(instruction);
    if cpu.general_purpose_registers[rs] < cpu.general_purpose_registers[rt] {
        cpu.general_purpose_registers[rd] = 1; // check if this is kosher or if i need to do 00..001 for some reason
    } else {
        cpu.general_purpose_registers[rd] = 0;
    }
    Ok(ExecutionStatus::Continue)
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
pub fn j(cpu: &mut Processor, memory: &mut Memory, instruction: u32) -> Result<ExecutionStatus, String> {
    let address: u32 = (unpack_j_type(instruction) << 2) | (cpu.pc & 0xF0000000);

    if address >= memory.text_end || address < memory.text_start {
        return Err(format!("Attempted to jump to unowned address 0x{:x}", address));
    }

    cpu.pc = address;

    Ok(ExecutionStatus::Continue)
}

// 0x03 - jal
pub fn jal(cpu: &mut Processor, memory: &mut Memory, instruction: u32) -> Result<ExecutionStatus, String> {
    let address: u32 = (unpack_j_type(instruction) << 2) | (cpu.pc & 0xF0000000);

    if address >= memory.text_end || address < memory.text_start {
        return Err(format!("Attempted to jump to unowned address 0x{:x}", address));
    }
    
    cpu.general_purpose_registers[RA] = cpu.pc;
    cpu.pc = address;

    Ok(ExecutionStatus::Continue)
}

// 0x04 - beq
pub fn beq(cpu: &mut Processor, memory: &mut Memory, instruction: u32) -> Result<ExecutionStatus, String> {
    let (rs, rt, imm) = unpack_i_type(instruction);

    // Sign extend offset
    let offset: i32 = ((imm & 0xFFFF) as i16 as i32) << 2;

    if cpu.general_purpose_registers[rs] != cpu.general_purpose_registers[rt] {
        return Ok(ExecutionStatus::Continue)
    }

    cpu.general_purpose_registers[AS_TEMP] = (cpu.pc as i32 + offset) as u32;

    if cpu.general_purpose_registers[AS_TEMP] >= memory.text_end || cpu.general_purpose_registers[AS_TEMP] < memory.text_start {
        return Err(format!("Attempted to access unowned address 0x{:x}", cpu.general_purpose_registers[AS_TEMP]));
    }
    
    Ok(ExecutionStatus::Continue)
}

// 0x05 - bne
pub fn bne(cpu: &mut Processor, memory: &mut Memory, instruction: u32) -> Result<ExecutionStatus, String> {
    let (rs, rt, imm) = unpack_i_type(instruction);

    // Sign extend offset
    let offset: i32 = ((imm & 0xFFFF) as i16 as i32) << 2;

    if cpu.general_purpose_registers[rs] == cpu.general_purpose_registers[rt] {
        return Ok(ExecutionStatus::Continue)
    }
    
    let temp = (cpu.pc as i32 + offset) as u32;

    if temp >= memory.text_end || temp < memory.text_start {
        return Err(format!("Attempted to access unowned address 0x{:x}", temp));
    }
    
    Ok(ExecutionStatus::Continue)
}

// 0x06 - blez
pub fn blez(cpu: &mut Processor, memory: &mut Memory, instruction: u32) -> Result<ExecutionStatus, String> {
    let (rs, _, imm) = unpack_i_type(instruction);

    let offset: i32 = ((imm & 0xFFFF) as i16 as i32) << 2;

    if cpu.general_purpose_registers[rs] > 0 {
        return Ok(ExecutionStatus::Continue)
    }

    let temp = (cpu.pc as i32 + offset) as u32;

    if temp >= memory.text_end || temp < memory.text_start {
        return Err(format!("Attempted to access unowned address 0x{:x}", temp));
    }

    Ok(ExecutionStatus::Continue)
}

// 0x07 - bgtz
pub fn bgtz(cpu: &mut Processor, memory: &mut Memory, instruction: u32) -> Result<ExecutionStatus, String> {
    let (rs, _, imm) = unpack_i_type(instruction);

    // Sign extend offset
    let offset: i32 = ((imm) as i16 as i32) << 2;

    if cpu.general_purpose_registers[rs] <= 0 {
        return Ok(ExecutionStatus::Continue)
    }
    
    let temp = (cpu.pc as i32 + offset) as u32;

    if temp >= memory.text_end || temp < memory.text_start {
        return Err(format!("Attempted to access unowned address 0x{:x}", temp));
    }

    cpu.pc = temp;

    // println!("Jumping to 0x{:x}", temp);
    
    Ok(ExecutionStatus::Continue)
}

// 0x08 - addi
pub fn addi(cpu: &mut Processor, _memory: &mut Memory, instruction: u32) -> Result<ExecutionStatus, String>{
    let (rs, rt, imm) = unpack_i_type(instruction);
    cpu.general_purpose_registers[rt] = (cpu.general_purpose_registers[rs] as i32 + (imm as i16 as i32)) as u32;
    // println!("Adding {} to {} and storing in ${}, now it's {}", (imm as i16 as i32), cpu.general_purpose_registers[rs], rt, cpu.general_purpose_registers[rt]);
    Ok(ExecutionStatus::Continue)
}


// 0x09 - addiu
pub fn addiu(cpu: &mut Processor, _memory: &mut Memory, instruction: u32) -> Result<ExecutionStatus, String>{
    let (rs, rt, imm) = unpack_i_type(instruction);
    cpu.general_purpose_registers[rt] = cpu.general_purpose_registers[rs].overflowing_add(imm).0;
    Ok(ExecutionStatus::Continue)
}


// 0x0C - andi
pub fn andi(cpu: &mut Processor, _memory: &mut Memory, instruction: u32) -> Result<ExecutionStatus, String> {
    let (rs, rt, imm) = unpack_i_type(instruction);
    cpu.general_purpose_registers[rt] = cpu.general_purpose_registers[rs] & imm;
    Ok(ExecutionStatus::Continue)
}

// 0x0D - ori
pub fn ori(cpu: &mut Processor, _memory: &mut Memory, instruction: u32) -> Result<ExecutionStatus, String> {
    let (rs, rt, imm) = unpack_i_type(instruction);
    cpu.general_purpose_registers[rt] = cpu.general_purpose_registers[rs] | imm;
    Ok(ExecutionStatus::Continue)
}

// 0x0F - lui
pub fn lui(cpu: &mut Processor, _memory: &mut Memory, instruction: u32) -> Result<ExecutionStatus, String> {
    let (_, rt, imm) = unpack_i_type(instruction);
    cpu.general_purpose_registers[rt] = imm << 16;
    Ok(ExecutionStatus::Continue)
}

// 0x20 - lb
pub fn lb(cpu: &mut Processor, memory: &mut Memory, instruction: u32) -> Result<ExecutionStatus, String> {
    let (rs, rt, imm) = unpack_i_type(instruction);

    cpu.general_purpose_registers[AS_TEMP] = (cpu.general_purpose_registers[rs] as i32 + imm as i32) as u32;

    if cpu.general_purpose_registers[AS_TEMP] >= memory.data_end || cpu.general_purpose_registers[AS_TEMP] < memory.data_start {
        return Err(format!("Attempted to access unowned address 0x{:x}", cpu.general_purpose_registers[AS_TEMP]));
    } else {
        cpu.general_purpose_registers[rt] = memory.data[(cpu.general_purpose_registers[AS_TEMP] - memory.data_start) as usize] as u32;
    }

    Ok(ExecutionStatus::Continue)
}

// 0x23 - lw
pub fn lw(cpu: &mut Processor, memory: &mut Memory, instruction: u32) -> Result<ExecutionStatus, String> {
    let (rs, rt, imm) = unpack_i_type(instruction);

    let temp = (cpu.general_purpose_registers[rs] as i32 + imm as i32) as u32;

    if temp % 4 != 0 {
        return Err(format!("Incorrect alignment for word access at 0x{:x}", temp));
    }

    if temp+4 >= memory.data_end || temp < memory.data_start {
        return Err(format!("Attempted to access unowned address 0x{:x} (possible alignment issue)", temp));
    }

    let start_idx: usize = (temp - memory.data_start) as usize;
    let end_idx: usize = (start_idx + 4) as usize;

    cpu.general_purpose_registers[rt] = u32::from_be_bytes(memory.data[start_idx..end_idx].try_into().unwrap());

    // println!("Loading {} from 0x{:x} into ${}", cpu.general_purpose_registers[rt], temp, rt);

    Ok(ExecutionStatus::Continue)
}

// 0x2b - sw
pub fn sw(cpu: &mut Processor, memory: &mut Memory, instruction: u32) -> Result<ExecutionStatus, String> {
    let (rs, rt, imm) = unpack_i_type(instruction);

    let temp = (cpu.general_purpose_registers[rs] as i32 + imm as i32) as u32;

    if temp % 4 != 0 {
        return Err(format!("Incorrect alignment for word access at 0x{:x}", temp));
    }

    if temp+4 >= memory.data_end || temp < memory.data_start {
        return Err(format!("Attempted to access unowned address 0x{:x} (possible alignment issue)", temp));
    }

    let start_idx: usize = (temp - memory.data_start) as usize;
    let end_idx: usize = (start_idx + 4) as usize;

    memory.data.splice(start_idx..end_idx, cpu.general_purpose_registers[rt].to_be_bytes());

    // println!("Storing {} at 0x{:x} from ${}", cpu.general_purpose_registers[rt], temp, rt);

    Ok(ExecutionStatus::Continue)
}