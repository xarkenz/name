use name_const::structs::{Processor, Memory};

use crate::definitions::structs::ExecutionStatus;

const A0: usize = 4;

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

    let output_string: String = String::from_utf8(to_print).map_err(|e| format!("UTF-8 conversion error: {}", e))?;

    print!("{}", output_string);

    Ok(ExecutionStatus::Continue)
}

// Syscall 10 - SysExit
pub fn sys_exit(_cpu: &mut Processor, _memory: &mut Memory) -> Result<ExecutionStatus, String> {
    return Ok(ExecutionStatus::Complete);
}