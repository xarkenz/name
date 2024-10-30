use crate::exception::definitions::ExceptionType;
use crate::instruction::{IArgs, JArgs, RArgs};
use crate::structs::{
    ProgramState,
    Register::{At, Ra},
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
pub fn sll(program_state: &mut ProgramState, args: RArgs) -> () {
    program_state.cpu.general_purpose_registers[args.rd as usize] =
        program_state.cpu.general_purpose_registers[args.rt as usize] << args.shamt;
}

// 0x02 - srl
pub fn srl(program_state: &mut ProgramState, args: RArgs) -> () {
    program_state.cpu.general_purpose_registers[args.rd as usize] =
        program_state.cpu.general_purpose_registers[args.rt as usize] >> args.shamt;
}

// 0x08 - jr
pub fn jr(program_state: &mut ProgramState, args: RArgs) -> () {
    if program_state.cpu.general_purpose_registers[args.rs as usize]
        >= program_state.memory.text_end
        || program_state.cpu.general_purpose_registers[args.rs as usize]
            < program_state.memory.text_start
    {
        // TODO: Use a function which sets the proper values in cp0 for us
        program_state.set_exception(ExceptionType::AddressExceptionLoad);
    }

    program_state.cpu.pc = program_state.cpu.general_purpose_registers[args.rs as usize];
}

// 0x09 - jalr
pub fn jalr(program_state: &mut ProgramState, args: RArgs) -> () {
    let rd = match args.rd {
        0 => 31,
        x => x,
    };

    if program_state.cpu.general_purpose_registers[args.rs as usize]
        >= program_state.memory.text_end
        || program_state.cpu.general_purpose_registers[args.rs as usize]
            < program_state.memory.text_start
    {
        // TODO: Take care of this lingering Err
        // TODO: Use a function which sets the proper values in cp0 for us
        program_state.set_exception(ExceptionType::AddressExceptionLoad);
    }
    program_state.cpu.general_purpose_registers[rd as usize] = program_state.cpu.pc;
    program_state.cpu.pc = program_state.cpu.general_purpose_registers[args.rs as usize];
}

// 0x0A - slti
pub fn slti(program_state: &mut ProgramState, args: IArgs) -> () {
    if (program_state.cpu.general_purpose_registers[args.rs as usize] as i32) < (args.imm as i32) {
        program_state.cpu.general_purpose_registers[args.rt as usize] = 1 as u32;
    } else {
        program_state.cpu.general_purpose_registers[args.rt as usize] = 0 as u32;
    }
}

// 0x0B - sltiu
pub fn sltiu(program_state: &mut ProgramState, args: IArgs) -> () {
    if program_state.cpu.general_purpose_registers[args.rs as usize] < args.imm {
        program_state.cpu.general_purpose_registers[args.rt as usize] = 1 as u32;
    } else {
        program_state.cpu.general_purpose_registers[args.rt as usize] = 0 as u32;
    }
}

// 0x0C - syscall
pub fn syscall(program_state: &mut ProgramState, _args: RArgs) -> () {
    program_state.set_exception(ExceptionType::Syscall);
}

// 0x20 - add
pub fn add(program_state: &mut ProgramState, args: RArgs) -> () {
    program_state.cpu.general_purpose_registers[args.rd as usize] =
        program_state.cpu.general_purpose_registers[args.rs as usize]
            + program_state.cpu.general_purpose_registers[args.rt as usize];

    // println!("Adding ${}({}) to ${}({}) and storing in ${}, now it's {}", rs, program_state.cpu.general_purpose_registers[args.rs as usize], rt, program_state.cpu.general_purpose_registers[args.rt as usize], rd, program_state.cpu.general_purpose_registers[args.rs as usize] + program_state.cpu.general_purpose_registers[args.rt as usize]);
}

// 0x21 - addu
pub fn addu(program_state: &mut ProgramState, args: RArgs) -> () {
    // check that below works
    program_state.cpu.general_purpose_registers[args.rd as usize] =
        program_state.cpu.general_purpose_registers[args.rs as usize]
            .overflowing_add(program_state.cpu.general_purpose_registers[args.rt as usize])
            .0;
}

// 0x22 - sub
pub fn sub(program_state: &mut ProgramState, args: RArgs) -> () {
    let temp: (u32, bool) = program_state.cpu.general_purpose_registers[args.rs as usize]
        .overflowing_sub(program_state.cpu.general_purpose_registers[args.rt as usize]);

    program_state.cpu.general_purpose_registers[At as usize] = temp.0;

    if temp.1 {
        // TODO: Use a function which sets the proper values in cp0 for us
        program_state.set_exception(ExceptionType::ArithmeticOverflow);
    } else {
        program_state.cpu.general_purpose_registers[args.rd as usize] =
            program_state.cpu.general_purpose_registers[At as usize];
    }
}

// 0x23 - subu
pub fn subu(program_state: &mut ProgramState, args: RArgs) -> () {
    let temp: (u32, bool) = program_state.cpu.general_purpose_registers[args.rs as usize]
        .overflowing_sub(program_state.cpu.general_purpose_registers[args.rt as usize]);

    program_state.cpu.general_purpose_registers[args.rd as usize] = temp.0;
}

// 0x24 - and
pub fn and(program_state: &mut ProgramState, args: RArgs) -> () {
    program_state.cpu.general_purpose_registers[args.rd as usize] =
        program_state.cpu.general_purpose_registers[args.rs as usize]
            & program_state.cpu.general_purpose_registers[args.rt as usize];
}

// 0x25 - or
pub fn or(program_state: &mut ProgramState, args: RArgs) -> () {
    program_state.cpu.general_purpose_registers[args.rd as usize] =
        program_state.cpu.general_purpose_registers[args.rs as usize]
            | program_state.cpu.general_purpose_registers[args.rt as usize];
}

// 0x26 - xor
pub fn xor(program_state: &mut ProgramState, args: RArgs) -> () {
    program_state.cpu.general_purpose_registers[args.rd as usize] =
        program_state.cpu.general_purpose_registers[args.rs as usize]
            ^ program_state.cpu.general_purpose_registers[args.rt as usize];
}

// 0x27 - nor
pub fn nor(program_state: &mut ProgramState, args: RArgs) -> () {
    program_state.cpu.general_purpose_registers[args.rd as usize] =
        !(program_state.cpu.general_purpose_registers[args.rs as usize]
            | program_state.cpu.general_purpose_registers[args.rt as usize]);
}

// 0x2A - slt
pub fn slt(program_state: &mut ProgramState, args: RArgs) -> () {
    if (program_state.cpu.general_purpose_registers[args.rs as usize] as i32)
        < (program_state.cpu.general_purpose_registers[args.rt as usize] as i32)
    {
        program_state.cpu.general_purpose_registers[args.rd as usize] = 1 as u32;
    } else {
        program_state.cpu.general_purpose_registers[args.rd as usize] = 0 as u32;
    }
}

// 0x2A - sltu
pub fn sltu(program_state: &mut ProgramState, args: RArgs) -> () {
    if program_state.cpu.general_purpose_registers[args.rs as usize]
        < program_state.cpu.general_purpose_registers[args.rt as usize]
    {
        program_state.cpu.general_purpose_registers[args.rd as usize] = 1; // check if this is kosher or if i need to do 00..001 for some reason
    } else {
        program_state.cpu.general_purpose_registers[args.rd as usize] = 0;
    }
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
pub fn j(program_state: &mut ProgramState, args: JArgs) -> () {
    let address: u32 = (args.address << 2) | (program_state.cpu.pc & 0xF0000000);

    if address >= program_state.memory.text_end || address < program_state.memory.text_start {
        // TODO: Use a function which sets the proper values in cp0 for us
        program_state.set_exception(ExceptionType::AddressExceptionLoad);
    }

    program_state.cpu.pc = address;
}

// 0x03 - jal
pub fn jal(program_state: &mut ProgramState, args: JArgs) -> () {
    let address: u32 = (args.address << 2) | (program_state.cpu.pc & 0xF0000000);

    if address >= program_state.memory.text_end || address < program_state.memory.text_start {
        // TODO: Use a function which sets the proper values in cp0 for us
        program_state.set_exception(ExceptionType::AddressExceptionLoad);
    }

    program_state.cpu.general_purpose_registers[Ra as usize] = program_state.cpu.pc;
    program_state.cpu.pc = address;
}

// 0x04 - beq
pub fn beq(program_state: &mut ProgramState, args: IArgs) -> () {
    // Sign extend offset
    let offset: i32 = ((args.imm & 0xFFFF) as i16 as i32) << 2;

    if program_state.cpu.general_purpose_registers[args.rs as usize]
        != program_state.cpu.general_purpose_registers[args.rt as usize]
    {
        return;
    }

    let temp = (program_state.cpu.pc as i32 + offset) as u32;

    if temp >= program_state.memory.text_end || temp < program_state.memory.text_start {
        // TODO: Use a function which sets the proper values in cp0 for us
        program_state.set_exception(ExceptionType::AddressExceptionLoad);
    }

    // Bro forgot the actual jump logic
    program_state.cpu.pc = temp;
}

// 0x05 - bne
pub fn bne(program_state: &mut ProgramState, args: IArgs) -> () {
    // Sign extend offset
    let offset: i32 = ((args.imm & 0xFFFF) as i16 as i32) << 2;

    if program_state.cpu.general_purpose_registers[args.rs as usize]
        == program_state.cpu.general_purpose_registers[args.rt as usize]
    {
        return;
    }

    let temp = (program_state.cpu.pc as i32 + offset) as u32;

    if temp >= program_state.memory.text_end || temp < program_state.memory.text_start {
        // TODO: Use a function which sets the proper values in cp0 for us
        program_state.set_exception(ExceptionType::AddressExceptionLoad);
    }

    // Bro once again forgot the actual jump logic
    program_state.cpu.pc = temp;
}

// 0x06 - blez
pub fn blez(program_state: &mut ProgramState, args: IArgs) -> () {
    let offset: i32 = ((args.imm & 0xFFFF) as i16 as i32) << 2;

    if (program_state.cpu.general_purpose_registers[args.rs as usize] as i32) > 0 {
        return;
    }

    let temp = (program_state.cpu.pc as i32 + offset) as u32;

    if temp >= program_state.memory.text_end || temp < program_state.memory.text_start {
        // TODO: Use a function which sets the proper values in cp0 for us
        program_state.set_exception(ExceptionType::AddressExceptionLoad);
    }

    // BRO HAS ONCE AGAIN FORGOTTEN THE ACTUAL JUMP
    program_state.cpu.pc = temp;
}

// 0x07 - bgtz
pub fn bgtz(program_state: &mut ProgramState, args: IArgs) -> () {
    // Sign extend offset
    let offset: i32 = (args.imm as i16 as i32) << 2;

    if program_state.cpu.general_purpose_registers[args.rs as usize] as i32 <= 0 {
        return;
    }

    let temp = (program_state.cpu.pc as i32 + offset) as u32;

    if temp >= program_state.memory.text_end || temp < program_state.memory.text_start {
        // TODO: Use a function which sets the proper values in cp0 for us
        program_state.set_exception(ExceptionType::AddressExceptionLoad);
    }

    program_state.cpu.pc = temp;
}

// 0x08 - addi
pub fn addi(program_state: &mut ProgramState, args: IArgs) -> () {
    program_state.cpu.general_purpose_registers[args.rt as usize] =
        (program_state.cpu.general_purpose_registers[args.rs as usize] as i32
            + (args.imm as i16 as i32)) as u32;
}

// 0x09 - addiu
pub fn addiu(program_state: &mut ProgramState, args: IArgs) -> () {
    program_state.cpu.general_purpose_registers[args.rt as usize] =
        program_state.cpu.general_purpose_registers[args.rs as usize]
            .overflowing_add(args.imm)
            .0;
}

// 0x0C - andi
pub fn andi(program_state: &mut ProgramState, args: IArgs) -> () {
    program_state.cpu.general_purpose_registers[args.rt as usize] =
        program_state.cpu.general_purpose_registers[args.rs as usize] & args.imm;
}

// 0x0D - ori
pub fn ori(program_state: &mut ProgramState, args: IArgs) -> () {
    program_state.cpu.general_purpose_registers[args.rt as usize] =
        program_state.cpu.general_purpose_registers[args.rs as usize] | args.imm;
}

// 0x0E - xori
pub fn xori(program_state: &mut ProgramState, args: IArgs) -> () {
    program_state.cpu.general_purpose_registers[args.rt as usize] =
        program_state.cpu.general_purpose_registers[args.rs as usize] ^ args.imm;
}

// 0x0F - lui
pub fn lui(program_state: &mut ProgramState, args: IArgs) -> () {
    // SUPER DUPER PROBLEM SPOT
    program_state.cpu.general_purpose_registers[args.rt as usize] = args.imm << 16;
}

// 0x20 - lb
pub fn lb(program_state: &mut ProgramState, args: IArgs) -> () {
    program_state.cpu.general_purpose_registers[At as usize] =
        (program_state.cpu.general_purpose_registers[args.rs as usize] as i32 + args.imm as i32)
            as u32;

    if program_state.cpu.general_purpose_registers[At as usize] >= program_state.memory.data_end
        || program_state.cpu.general_purpose_registers[At as usize]
            < program_state.memory.data_start
    {
        // TODO: Use a function which sets the proper values in cp0 for us
        program_state.set_exception(ExceptionType::AddressExceptionLoad);
    } else {
        program_state.cpu.general_purpose_registers[args.rt as usize] =
            program_state.memory.data[(program_state.cpu.general_purpose_registers[At as usize]
                - program_state.memory.data_start) as usize] as u32;
    }
}

// 0x23 - lw
pub fn lw(program_state: &mut ProgramState, args: IArgs) -> () {
    let temp = (program_state.cpu.general_purpose_registers[args.rs as usize] as i32
        + args.imm as i32) as u32;

    if temp % 4 != 0 {
        program_state.set_exception(ExceptionType::AddressExceptionLoad);
        return;
    }

    if temp + 4 >= program_state.memory.data_end || temp < program_state.memory.data_start {
        program_state.set_exception(ExceptionType::AddressExceptionLoad);
        return;
    }

    // Checks passed. Load word.
    let start_idx: usize = (temp - program_state.memory.data_start) as usize;
    let end_idx: usize = (start_idx + 4) as usize;

    program_state.cpu.general_purpose_registers[args.rt as usize] = u32::from_be_bytes(
        program_state.memory.data[start_idx..end_idx]
            .try_into()
            .unwrap(),
    );
}

// 0x28 - sb
pub fn sb(program_state: &mut ProgramState, args: IArgs) -> () {
    let temp = (program_state.cpu.general_purpose_registers[args.rs as usize] as i32
        + args.imm as i32) as u32;

    if temp >= program_state.memory.data_end || temp < program_state.memory.data_start {
        // TODO: Use a function which sets the proper values in cp0 for us
        program_state.set_exception(ExceptionType::AddressExceptionStore);
    }

    program_state.memory.data[(temp - program_state.memory.data_start) as usize] =
        program_state.cpu.general_purpose_registers[args.rt as usize] as u8;
}

// 0x2b - sw
pub fn sw(program_state: &mut ProgramState, args: IArgs) -> () {
    let temp = (program_state.cpu.general_purpose_registers[args.rs as usize] as i32
        + args.imm as i32) as u32;

    if temp % 4 != 0 {
        // TODO: Use a function which sets the proper values in cp0 for us
        program_state.set_exception(ExceptionType::AddressExceptionStore);
    }

    if temp + 4 >= program_state.memory.data_end || temp < program_state.memory.data_start {
        // TODO: Use a function which sets the proper values in cp0 for us
        program_state.set_exception(ExceptionType::AddressExceptionStore);
    }

    let start_idx: usize = (temp - program_state.memory.data_start) as usize;
    let end_idx: usize = (start_idx + 4) as usize;

    program_state.memory.data.splice(
        start_idx..end_idx,
        program_state.cpu.general_purpose_registers[args.rt as usize].to_be_bytes(),
    );

    // println!("Storing {} at 0x{:x} from ${}", program_state.cpu.general_purpose_registers[args.rt as usize], temp, rt);
}