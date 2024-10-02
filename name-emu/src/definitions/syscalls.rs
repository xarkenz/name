use name_const::structs::{Memory, Processor};
use std::io::{Read, Write};

use crate::definitions::structs::ExecutionStatus;

use std::io;

const A0: usize = 4;
const V0: usize = 2;

pub type SyscallFn = fn(&mut Processor, &mut Memory) -> Result<ExecutionStatus, String>;

// Syscall 1 - SysPrintInt
pub fn sys_print_int(cpu: &mut Processor, _memory: &mut Memory) -> Result<ExecutionStatus, String> {
    print!("{}", cpu.general_purpose_registers[A0]);
    Ok(ExecutionStatus::Continue)
}

// Syscall 4 - SysPrintString
pub fn sys_print_string(cpu: &mut Processor, memory: &mut Memory) -> Result<ExecutionStatus, String> {
    let mut address = cpu.general_purpose_registers[A0];
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
            cpu.general_purpose_registers[V0] = i;
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
pub fn sys_print_char(cpu: &mut Processor, _memory: &mut Memory) -> Result<ExecutionStatus, String> {
    match char::from_u32(cpu.general_purpose_registers[A0]){
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

    cpu.general_purpose_registers[V0] = buf[0] as u32;
    Ok(ExecutionStatus::Continue)
}
