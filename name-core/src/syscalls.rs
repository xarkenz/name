use std::io::{Read, Write};

use crate::structs::{
    ExecutionStatus, Memory, Processor,
    Register::{A0, V0},
};

use std::io;

pub type SyscallFn = fn(&mut Processor, &mut Memory) -> Result<ExecutionStatus, String>;

pub const SYSCALL_TABLE: [Option<SyscallFn>; 64] = [
    None,                   // 0x00
    Some(sys_print_int),    // 0x01
    None,                   // 0x02
    None,                   // 0x03
    Some(sys_print_string), // 0x04
    Some(sys_read_int),     // 0x05
    None,                   // 0x06
    None,                   // 0x07
    None,                   // 0x08
    None,                   // 0x09
    Some(sys_exit),         // 0x0A
    Some(sys_print_char),   // 0x0B
    Some(sys_read_char),    // 0x0C
    None,                   // 0x0D
    None,                   // 0x0E
    None,                   // 0x0F
    None,                   // 0x10
    None,                   // 0x11
    None,                   // 0x12
    None,                   // 0x13
    None,                   // 0x14
    None,                   // 0x15
    None,                   // 0x16
    None,                   // 0x17
    None,                   // 0x18
    None,                   // 0x19
    None,                   // 0x1A
    None,                   // 0x1B
    None,                   // 0x1C
    None,                   // 0x1D
    None,                   // 0x1E
    None,                   // 0x1F
    None,                   // 0x20
    None,                   // 0x21
    None,                   // 0x22
    None,                   // 0x23
    None,                   // 0x24
    None,                   // 0x25
    None,                   // 0x26
    None,                   // 0x27
    None,                   // 0x28
    None,                   // 0x29
    None,                   // 0x2A
    None,                   // 0x2B
    None,                   // 0x2C
    None,                   // 0x2D
    None,                   // 0x2E
    None,                   // 0x2F
    None,                   // 0x30
    None,                   // 0x31
    None,                   // 0x32
    None,                   // 0x33
    None,                   // 0x34
    None,                   // 0x35
    None,                   // 0x36
    None,                   // 0x37
    None,                   // 0x38
    None,                   // 0x39
    None,                   // 0x3A
    None,                   // 0x3B
    None,                   // 0x3C
    None,                   // 0x3D
    None,                   // 0x3E
    None,                   // 0x3F
];

// Syscall 1 - SysPrintInt
pub fn sys_print_int(cpu: &mut Processor, _memory: &mut Memory) -> Result<ExecutionStatus, String> {
    print!("{}", cpu.general_purpose_registers[A0 as usize]);
    Ok(ExecutionStatus::Continue)
}

// Syscall 4 - SysPrintString
pub fn sys_print_string(
    cpu: &mut Processor,
    memory: &mut Memory,
) -> Result<ExecutionStatus, String> {
    let mut address = cpu.general_purpose_registers[A0 as usize];
    let mut to_print: Vec<u8> = Vec::new();

    loop {
        let byte = match memory.read_byte(address) {
            Ok(b) => b,
            Err(e) => return Err(e),
        };

        if byte == 0 {
            break;
        }

        to_print.push(byte);
        address += 1;
    }

    let output_string: String =
        String::from_utf8(to_print).map_err(|e| format!("UTF-8 conversion error: {}", e))?;

    print!("{}", output_string);

    Ok(ExecutionStatus::Continue)
}

// Syscall 5 - SysReadInt
pub fn sys_read_int(cpu: &mut Processor, _memory: &mut Memory) -> Result<ExecutionStatus, String> {
    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("Failed to read from stdin");

    let trimmed = input_text.trim();
    match trimmed.parse::<u32>() {
        Ok(i) => {
            cpu.general_purpose_registers[V0 as usize] = i;
            Ok(ExecutionStatus::Continue)
        }
        Err(..) => {
            // eprintln!("{} is not an integer.\nRead failed", trimmed);
            Err(format!("{} is not an integer.\nRead failed", trimmed))
        }
    }
}

// Syscall 10 - SysExit
pub fn sys_exit(_cpu: &mut Processor, _memory: &mut Memory) -> Result<ExecutionStatus, String> {
    return Ok(ExecutionStatus::Complete);
}

// Syscall 11 - SysPrintChar
pub fn sys_print_char(
    cpu: &mut Processor,
    _memory: &mut Memory,
) -> Result<ExecutionStatus, String> {
    match char::from_u32(cpu.general_purpose_registers[A0 as usize]) {
        Some(valid_char) => {
            print!("{}", valid_char);
            Ok(ExecutionStatus::Continue)
        }
        None => Err("Value in register $a0 could not be evaluated to a char.".to_string()),
    }
}

// Syscall 12 - SysReadChar
pub fn sys_read_char(cpu: &mut Processor, _memory: &mut Memory) -> Result<ExecutionStatus, String> {
    let mut buf = [0; 1];
    io::stdout().flush().expect("Failed to flush stdout");
    io::stdin()
        .read_exact(&mut buf)
        .expect("Failed to read from stdin");

    cpu.general_purpose_registers[V0 as usize] = buf[0] as u32;
    Ok(ExecutionStatus::Continue)
}
