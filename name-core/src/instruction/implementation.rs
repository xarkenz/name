use crate::instruction::{IArgs, JArgs, RArgs};
use crate::{
    structs::{
        ExecutionStatus, Memory, Processor,
        Register::{At, Ra, V0},
    },
    syscalls::SYSCALL_TABLE,
};

/*

  ______ _    _ _   _  _____ _______
 |  ____| |  | | \ | |/ ____|__   __|
 | |__  | |  | |  \| | |       | |
 |  __| | |  | | . ` | |       | |
 | |    | |__| | |\  | |____   | |
 |_|     \____/|_| \_|\_____|  |_|



*/

// 0x00 - sll
pub fn sll(
    cpu: &mut Processor,
    _memory: &mut Memory,
    args: RArgs,
) -> Result<ExecutionStatus, String> {
    cpu.general_purpose_registers[args.rd as usize] =
        cpu.general_purpose_registers[args.rt as usize] << args.shamt;
    Ok(ExecutionStatus::Continue)
}

// 0x02 - srl
pub fn srl(
    cpu: &mut Processor,
    _memory: &mut Memory,
    args: RArgs,
) -> Result<ExecutionStatus, String> {
    cpu.general_purpose_registers[args.rd as usize] =
        cpu.general_purpose_registers[args.rt as usize] >> args.shamt;
    Ok(ExecutionStatus::Continue)
}

// 0x08 - jr
pub fn jr(
    cpu: &mut Processor,
    memory: &mut Memory,
    args: RArgs,
) -> Result<ExecutionStatus, String> {
    if cpu.general_purpose_registers[args.rs as usize] >= memory.text_end
        || cpu.general_purpose_registers[args.rs as usize] < memory.text_start
    {
        return Err(format!(
            "Attempted to jump to unowned address 0x{:x}",
            cpu.general_purpose_registers[args.rs as usize]
        ));
    }

    cpu.pc = cpu.general_purpose_registers[args.rs as usize];

    Ok(ExecutionStatus::Continue)
}

// 0x09 - jalr
pub fn jalr(
    cpu: &mut Processor,
    memory: &mut Memory,
    args: RArgs,
) -> Result<ExecutionStatus, String> {
    let rd = match args.rd {
        0 => 31,
        x => x,
    };

    if cpu.general_purpose_registers[args.rs as usize] >= memory.text_end
        || cpu.general_purpose_registers[args.rs as usize] < memory.text_start
    {
        return Err(format!(
            "Attempted to jump to unowned address 0x{:x}",
            cpu.general_purpose_registers[args.rs as usize]
        ));
    }
    cpu.general_purpose_registers[rd as usize] = cpu.pc;
    cpu.pc = cpu.general_purpose_registers[args.rs as usize];

    Ok(ExecutionStatus::Continue)
}

// 0x0A - slti
pub fn slti(
    cpu: &mut Processor,
    _memory: &mut Memory,
    args: IArgs,
) -> Result<ExecutionStatus, String> {
    if (cpu.general_purpose_registers[args.rs as usize] as i32) < (args.imm as i32) {
        cpu.general_purpose_registers[args.rt as usize] = 1 as u32;
    } else {
        cpu.general_purpose_registers[args.rt as usize] = 0 as u32;
    }

    Ok(ExecutionStatus::Continue)
}

// 0x0B - sltiu
pub fn sltiu(
    cpu: &mut Processor,
    _memory: &mut Memory,
    args: IArgs,
) -> Result<ExecutionStatus, String> {
    if cpu.general_purpose_registers[args.rs as usize] < args.imm {
        cpu.general_purpose_registers[args.rt as usize] = 1 as u32;
    } else {
        cpu.general_purpose_registers[args.rt as usize] = 0 as u32;
    }

    Ok(ExecutionStatus::Continue)
}

// 0x0C - syscall
pub fn syscall(
    cpu: &mut Processor,
    memory: &mut Memory,
    _args: RArgs,
) -> Result<ExecutionStatus, String> {
    let syscall_num: usize = cpu.general_purpose_registers[V0 as usize] as usize;
    match SYSCALL_TABLE[syscall_num] {
        Some(fun) => fun(cpu, memory),
        None => return Err(format!("Syscall {} is not implemented", syscall_num)),
    }
}

// 0x20 - add
pub fn add(
    cpu: &mut Processor,
    _memory: &mut Memory,
    args: RArgs,
) -> Result<ExecutionStatus, String> {
    cpu.general_purpose_registers[args.rd as usize] = cpu.general_purpose_registers
        [args.rs as usize]
        + cpu.general_purpose_registers[args.rt as usize];

    // println!("Adding ${}({}) to ${}({}) and storing in ${}, now it's {}", rs, cpu.general_purpose_registers[args.rs as usize], rt, cpu.general_purpose_registers[args.rt as usize], rd, cpu.general_purpose_registers[args.rs as usize] + cpu.general_purpose_registers[args.rt as usize]);

    Ok(ExecutionStatus::Continue)
}

// 0x21 - addu
pub fn addu(
    cpu: &mut Processor,
    _memory: &mut Memory,
    args: RArgs,
) -> Result<ExecutionStatus, String> {
    // check that below works
    cpu.general_purpose_registers[args.rd as usize] = cpu.general_purpose_registers
        [args.rs as usize]
        .overflowing_add(cpu.general_purpose_registers[args.rt as usize])
        .0;
    Ok(ExecutionStatus::Continue)
}

// 0x22 - sub
pub fn sub(
    cpu: &mut Processor,
    _memory: &mut Memory,
    args: RArgs,
) -> Result<ExecutionStatus, String> {
    let temp: (u32, bool) = cpu.general_purpose_registers[args.rs as usize]
        .overflowing_sub(cpu.general_purpose_registers[args.rt as usize]);

    cpu.general_purpose_registers[At as usize] = temp.0;

    if temp.1 {
        // TODO: Implement coprocessor 0 and signal integer overflow

        return Err(format!("Integer underflow occurred in subtraction."));
    } else {
        cpu.general_purpose_registers[args.rd as usize] =
            cpu.general_purpose_registers[At as usize];
    }

    Ok(ExecutionStatus::Continue)
}

// 0x23 - subu
pub fn subu(
    cpu: &mut Processor,
    _memory: &mut Memory,
    args: RArgs,
) -> Result<ExecutionStatus, String> {
    let temp: (u32, bool) = cpu.general_purpose_registers[args.rs as usize]
        .overflowing_sub(cpu.general_purpose_registers[args.rt as usize]);

    cpu.general_purpose_registers[args.rd as usize] = temp.0;

    Ok(ExecutionStatus::Continue)
}

// 0x24 - and
pub fn and(
    cpu: &mut Processor,
    _memory: &mut Memory,
    args: RArgs,
) -> Result<ExecutionStatus, String> {
    cpu.general_purpose_registers[args.rd as usize] = cpu.general_purpose_registers
        [args.rs as usize]
        & cpu.general_purpose_registers[args.rt as usize];
    Ok(ExecutionStatus::Continue)
}

// 0x25 - or
pub fn or(
    cpu: &mut Processor,
    _memory: &mut Memory,
    args: RArgs,
) -> Result<ExecutionStatus, String> {
    cpu.general_purpose_registers[args.rd as usize] = cpu.general_purpose_registers
        [args.rs as usize]
        | cpu.general_purpose_registers[args.rt as usize];
    Ok(ExecutionStatus::Continue)
}

// 0x26 - xor
pub fn xor(
    cpu: &mut Processor,
    _memory: &mut Memory,
    args: RArgs,
) -> Result<ExecutionStatus, String> {
    cpu.general_purpose_registers[args.rd as usize] = cpu.general_purpose_registers
        [args.rs as usize]
        ^ cpu.general_purpose_registers[args.rt as usize];
    Ok(ExecutionStatus::Continue)
}

// 0x27 - nor
pub fn nor(
    cpu: &mut Processor,
    _memory: &mut Memory,
    args: RArgs,
) -> Result<ExecutionStatus, String> {
    cpu.general_purpose_registers[args.rd as usize] = !(cpu.general_purpose_registers
        [args.rs as usize]
        | cpu.general_purpose_registers[args.rt as usize]);
    Ok(ExecutionStatus::Continue)
}

// 0x2A - slt
pub fn slt(
    cpu: &mut Processor,
    _memory: &mut Memory,
    args: RArgs,
) -> Result<ExecutionStatus, String> {
    if (cpu.general_purpose_registers[args.rs as usize] as i32)
        < (cpu.general_purpose_registers[args.rt as usize] as i32)
    {
        cpu.general_purpose_registers[args.rd as usize] = 1 as u32;
    } else {
        cpu.general_purpose_registers[args.rd as usize] = 0 as u32;
    }
    Ok(ExecutionStatus::Continue)
}

// 0x2A - sltu
pub fn sltu(
    cpu: &mut Processor,
    _memory: &mut Memory,
    args: RArgs,
) -> Result<ExecutionStatus, String> {
    if cpu.general_purpose_registers[args.rs as usize]
        < cpu.general_purpose_registers[args.rt as usize]
    {
        cpu.general_purpose_registers[args.rd as usize] = 1; // check if this is kosher or if i need to do 00..001 for some reason
    } else {
        cpu.general_purpose_registers[args.rd as usize] = 0;
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
pub fn j(cpu: &mut Processor, memory: &mut Memory, args: JArgs) -> Result<ExecutionStatus, String> {
    let address: u32 = (args.address << 2) | (cpu.pc & 0xF0000000);

    if address >= memory.text_end || address < memory.text_start {
        return Err(format!(
            "Attempted to jump to unowned address 0x{:x}",
            address
        ));
    }

    cpu.pc = address;

    Ok(ExecutionStatus::Continue)
}

// 0x03 - jal
pub fn jal(
    cpu: &mut Processor,
    memory: &mut Memory,
    args: JArgs,
) -> Result<ExecutionStatus, String> {
    let address: u32 = (args.address << 2) | (cpu.pc & 0xF0000000);

    if address >= memory.text_end || address < memory.text_start {
        return Err(format!(
            "Attempted to jump to unowned address 0x{:x}",
            address
        ));
    }

    cpu.general_purpose_registers[Ra as usize] = cpu.pc;
    cpu.pc = address;

    Ok(ExecutionStatus::Continue)
}

// 0x04 - beq
pub fn beq(
    cpu: &mut Processor,
    memory: &mut Memory,
    args: IArgs,
) -> Result<ExecutionStatus, String> {
    // Sign extend offset
    let offset: i32 = ((args.imm & 0xFFFF) as i16 as i32) << 2;

    if cpu.general_purpose_registers[args.rs as usize]
        != cpu.general_purpose_registers[args.rt as usize]
    {
        return Ok(ExecutionStatus::Continue);
    }

    let temp = (cpu.pc as i32 + offset) as u32;

    if temp >= memory.text_end || temp < memory.text_start {
        return Err(format!("Attempted to access unowned address 0x{:x}", temp));
    }

    // Bro forgot the actual jump logic
    cpu.pc = temp;

    Ok(ExecutionStatus::Continue)
}

// 0x05 - bne
pub fn bne(
    cpu: &mut Processor,
    memory: &mut Memory,
    args: IArgs,
) -> Result<ExecutionStatus, String> {
    // Sign extend offset
    let offset: i32 = ((args.imm & 0xFFFF) as i16 as i32) << 2;

    if cpu.general_purpose_registers[args.rs as usize]
        == cpu.general_purpose_registers[args.rt as usize]
    {
        return Ok(ExecutionStatus::Continue);
    }

    let temp = (cpu.pc as i32 + offset) as u32;

    if temp >= memory.text_end || temp < memory.text_start {
        return Err(format!("Attempted to access unowned address 0x{:x}", temp));
    }

    // Bro once again forgot the actual jump logic
    cpu.pc = temp;

    Ok(ExecutionStatus::Continue)
}

// 0x06 - blez
pub fn blez(
    cpu: &mut Processor,
    memory: &mut Memory,
    args: IArgs,
) -> Result<ExecutionStatus, String> {
    let offset: i32 = ((args.imm & 0xFFFF) as i16 as i32) << 2;

    if (cpu.general_purpose_registers[args.rs as usize] as i32) > 0 {
        return Ok(ExecutionStatus::Continue);
    }

    let temp = (cpu.pc as i32 + offset) as u32;

    if temp >= memory.text_end || temp < memory.text_start {
        return Err(format!("Attempted to access unowned address 0x{:x}", temp));
    }

    // BRO HAS ONCE AGAIN FORGOTTEN THE ACTUAL JUMP
    cpu.pc = temp;

    Ok(ExecutionStatus::Continue)
}

// 0x07 - bgtz
pub fn bgtz(
    cpu: &mut Processor,
    memory: &mut Memory,
    args: IArgs,
) -> Result<ExecutionStatus, String> {
    // Sign extend offset
    let offset: i32 = (args.imm as i16 as i32) << 2;

    if cpu.general_purpose_registers[args.rs as usize] as i32 <= 0 {
        return Ok(ExecutionStatus::Continue);
    }

    let temp = (cpu.pc as i32 + offset) as u32;

    if temp >= memory.text_end || temp < memory.text_start {
        return Err(format!("Attempted to access unowned address 0x{:x}", temp));
    }

    cpu.pc = temp;

    Ok(ExecutionStatus::Continue)
}

// 0x08 - addi
pub fn addi(
    cpu: &mut Processor,
    _memory: &mut Memory,
    args: IArgs,
) -> Result<ExecutionStatus, String> {
    cpu.general_purpose_registers[args.rt as usize] =
        (cpu.general_purpose_registers[args.rs as usize] as i32 + (args.imm as i16 as i32)) as u32;
    Ok(ExecutionStatus::Continue)
}

// 0x09 - addiu
pub fn addiu(
    cpu: &mut Processor,
    _memory: &mut Memory,
    args: IArgs,
) -> Result<ExecutionStatus, String> {
    cpu.general_purpose_registers[args.rt as usize] = cpu.general_purpose_registers
        [args.rs as usize]
        .overflowing_add(args.imm)
        .0;
    Ok(ExecutionStatus::Continue)
}

// 0x0C - andi
pub fn andi(
    cpu: &mut Processor,
    _memory: &mut Memory,
    args: IArgs,
) -> Result<ExecutionStatus, String> {
    cpu.general_purpose_registers[args.rt as usize] =
        cpu.general_purpose_registers[args.rs as usize] & args.imm;
    Ok(ExecutionStatus::Continue)
}

// 0x0D - ori
pub fn ori(
    cpu: &mut Processor,
    _memory: &mut Memory,
    args: IArgs,
) -> Result<ExecutionStatus, String> {
    cpu.general_purpose_registers[args.rt as usize] =
        cpu.general_purpose_registers[args.rs as usize] | args.imm;
    Ok(ExecutionStatus::Continue)
}

// 0x0E - xori
pub fn xori(
    cpu: &mut Processor,
    _memory: &mut Memory,
    args: IArgs,
) -> Result<ExecutionStatus, String> {
    cpu.general_purpose_registers[args.rt as usize] =
        cpu.general_purpose_registers[args.rs as usize] ^ args.imm;
    Ok(ExecutionStatus::Continue)
}

// 0x0F - lui
pub fn lui(
    cpu: &mut Processor,
    _memory: &mut Memory,
    args: IArgs,
) -> Result<ExecutionStatus, String> {
    // SUPER DUPER PROBLEM SPOT
    cpu.general_purpose_registers[args.rt as usize] = args.imm << 16;
    Ok(ExecutionStatus::Continue)
}

// 0x20 - lb
pub fn lb(
    cpu: &mut Processor,
    memory: &mut Memory,
    args: IArgs,
) -> Result<ExecutionStatus, String> {
    cpu.general_purpose_registers[At as usize] =
        (cpu.general_purpose_registers[args.rs as usize] as i32 + args.imm as i32) as u32;

    if cpu.general_purpose_registers[At as usize] >= memory.data_end
        || cpu.general_purpose_registers[At as usize] < memory.data_start
    {
        return Err(format!(
            "Attempted to access unowned address 0x{:x}",
            cpu.general_purpose_registers[At as usize]
        ));
    } else {
        cpu.general_purpose_registers[args.rt as usize] = memory.data
            [(cpu.general_purpose_registers[At as usize] - memory.data_start) as usize]
            as u32;
    }

    Ok(ExecutionStatus::Continue)
}

// 0x23 - lw
pub fn lw(
    cpu: &mut Processor,
    memory: &mut Memory,
    args: IArgs,
) -> Result<ExecutionStatus, String> {
    let temp = (cpu.general_purpose_registers[args.rs as usize] as i32 + args.imm as i32) as u32;

    if temp % 4 != 0 {
        return Err(format!(
            "Incorrect alignment for word access at 0x{:x}",
            temp
        ));
    }

    if temp + 4 >= memory.data_end || temp < memory.data_start {
        return Err(format!(
            "Attempted to access unowned address 0x{:x} (possible alignment issue)",
            temp
        ));
    }

    let start_idx: usize = (temp - memory.data_start) as usize;
    let end_idx: usize = (start_idx + 4) as usize;

    cpu.general_purpose_registers[args.rt as usize] =
        u32::from_be_bytes(memory.data[start_idx..end_idx].try_into().unwrap());

    Ok(ExecutionStatus::Continue)
}

// 0x28 - sb
pub fn sb(
    cpu: &mut Processor,
    memory: &mut Memory,
    args: IArgs,
) -> Result<ExecutionStatus, String> {
    let temp = (cpu.general_purpose_registers[args.rs as usize] as i32 + args.imm as i32) as u32;

    if temp >= memory.data_end || temp < memory.data_start {
        return Err(format!("Attempted to access unowned address 0x{:x}", temp));
    }

    memory.data[(temp - memory.data_start) as usize] =
        cpu.general_purpose_registers[args.rt as usize] as u8;

    Ok(ExecutionStatus::Continue)
}

// 0x2b - sw
pub fn sw(
    cpu: &mut Processor,
    memory: &mut Memory,
    args: IArgs,
) -> Result<ExecutionStatus, String> {
    let temp = (cpu.general_purpose_registers[args.rs as usize] as i32 + args.imm as i32) as u32;

    if temp % 4 != 0 {
        return Err(format!(
            "Incorrect alignment for word access at 0x{:x}",
            temp
        ));
    }

    if temp + 4 >= memory.data_end || temp < memory.data_start {
        return Err(format!(
            "Attempted to access unowned address 0x{:x} (possible alignment issue)",
            temp
        ));
    }

    let start_idx: usize = (temp - memory.data_start) as usize;
    let end_idx: usize = (start_idx + 4) as usize;

    memory.data.splice(
        start_idx..end_idx,
        cpu.general_purpose_registers[args.rt as usize].to_be_bytes(),
    );

    // println!("Storing {} at 0x{:x} from ${}", cpu.general_purpose_registers[args.rt as usize], temp, rt);

    Ok(ExecutionStatus::Continue)
}
