/*  System call functions.
 *
 *  Most of these are analogous to functions found in nearly every operating system, 
 */

use std::io::{Read, Write};

use crate::structs::{
    ProgramState,
    Register::{A0, V0},
};

use std::io;

// This macro is used to keep from having to type a bunch of stuff when implementing a new syscall.
macro_rules! make_syscall {
    ($name: ident, $ps: ident, $body: expr) => {
        pub fn $name($ps: &mut ProgramState) -> Result<(), String> {
            $body
            Ok(())
        }
    };
}

pub type SyscallFn = fn(&mut ProgramState) -> Result<(), String>;

pub const SYSCALL_TABLE: [Option<SyscallFn>; 64] = [
    None,                   // 0x00
    Some(sys_print_int),    // 0x01
    None,                   // 0x02  sys_print_float
    None,                   // 0x03  sys_print_double
    Some(sys_print_string), // 0x04
    Some(sys_read_int),     // 0x05
    None,                   // 0x06
    None,                   // 0x07
    Some(sys_read_string),  // 0x08  sys_read_string
    None,                   // 0x09  sys_alloc (SBRK)
    Some(sys_exit),         // 0x0A
    Some(sys_print_char),   // 0x0B
    Some(sys_read_char),    // 0x0C
    None,                   // 0x0D  sys_open_file
    None,                   // 0x0E  sys_read_file
    None,                   // 0x0F  sys_write_file
    None,                   // 0x10  sys_close_fie
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
make_syscall!(sys_print_int, program_state, {
    print!(
        "{}",
        program_state.cpu.general_purpose_registers[A0 as usize]
    );
});

// Syscall 4 - SysPrintString
make_syscall!(sys_print_string, program_state, {
    let mut address = program_state.cpu.general_purpose_registers[A0 as usize];
    let mut to_print: Vec<u8> = Vec::new();

    loop {
        let byte = program_state
            .memory
            .read_byte(address)
            .map_err(|_| "Failed to read byte from memory")?;

        if byte == 0 {
            break;
        }

        to_print.push(byte);
        address += 1;
    }

    let output_string: String =
        String::from_utf8(to_print).map_err(|_| "Supplied string is NOT utf-8")?;

    print!("{}", output_string);
});

// Syscall 5 - SysReadInt
make_syscall!(sys_read_int, program_state, {
    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .map_err(|_| "Failed to read from stdin")?;

    let trimmed = input_text.trim();
    match trimmed.parse::<u32>() {
        Ok(i) => {
            program_state.cpu.general_purpose_registers[V0 as usize] = i;
        }
        Err(_) => {
            return Err(format!("Failed to convert input to Int"));
        }
    }
});

// Syscall 8 - sys_read_string  -- Read a string from the keyboard one character at a time
// until we get either a \n character or run out of space.  We accept up to maxlength-1
// characters because the string is alwaays null-terminated.  If we get to the maximum
// length the \n will not be stored.
pub fn sys_read_string
(cpu: &mut Processor, memory: &mut Memory) -> Result<ExecutionStatus, String>
{
    let mut buf = [0; 1];
    let mut count = 0;
    let mut address = cpu.general_purpose_registers[A0 as usize];
    let maxlength = cpu.general_purpose_registers[A1 as usize];
    while count < maxlength
    {
    io::stdout().flush().expect("Failed to flush stdout");
    io::stdin()
        .read_exact(&mut buf)
        .expect("Failed to read from stdin");
        memory.data[address as usize] = buf[0];
        count += 1;
        address += 1;
        if buf[0] == b'\n' as u8
        {
            break;
        }
    }
    buf[0] = 0;
    memory.data[address as usize] = buf[0];
Ok(ExecutionStatus::Continue)
}

// Syscall 10 - SysExit
make_syscall!(sys_exit, program_state, {
    // Simply tell the program it should no longer execute.
    program_state.should_continue_execution = false;
});


// Syscall 11 - SysPrintChar
make_syscall!(sys_print_char, program_state, {
    match char::from_u32(program_state.cpu.general_purpose_registers[A0 as usize]) {
        Some(valid_char) => {
            print!("{}", valid_char);
        }
        None => return Err(format!("Failed to convert given character to byte value.")),
    }
});

// Syscall 12 - SysReadChar
make_syscall!(sys_read_char, program_state, {
    let mut buf = [0; 1];
    io::stdout().flush().map_err(|_| "Failed to flush stdout")?;
    io::stdin()
        .read_exact(&mut buf)
        .map_err(|_| "Failed to read from stdin")?;

    program_state.cpu.general_purpose_registers[V0 as usize] = buf[0] as u32;
});
