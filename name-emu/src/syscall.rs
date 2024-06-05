// This file implements the system calls used by MARS.
// The system call number is stored in $v0, which is register 2.

use crate::mips::Mips;
use crate::exception::{ExecutionErrors, ExecutionEvents};
// use std::io::Stdin; // Lmao unused

const LAST_AVAILABLE_SYSCALL: u32 = 11;

// If you're curious what this is, take a look right below.
type SyscallFn = fn(&mut Mips) -> Result<(), ExecutionErrors>;

// This is the syscall lookup table. It consists of a set of function pointers which are used to actually execute the syscalls in question.
// Each function has the type SyscallFn, declared above.
// We use LAST_AVAILABLE_SYSCALL + 1 since arrays are zero-indexed.
const SYSCALL_LOOKUP_TABLE: [SyscallFn; (LAST_AVAILABLE_SYSCALL as usize) + 1] = [
    nonimpl, // 0
    sys_print_int, // 1
    nonimpl, // 2
    nonimpl, // 3
    sys_print_string, // 4
    nonimpl, // 5
    nonimpl, // 6
    nonimpl, // 7
    nonimpl, // 8
    nonimpl, // 9
    sys_exit, // 10
    sys_print_char, // 11
];

// For now, code is prefaced with '_' as it is unused
// This is insanely simple. Just perform a very simple lookup that calls the right function pointer.
pub(crate) fn syscall(mips: &mut Mips, _code: u32) -> Result<(), ExecutionErrors> {
    if mips.regs[2] < 1 {
        return Err(ExecutionErrors::SyscallInvalidSyscallNumber);
    } else if mips.regs[2] > LAST_AVAILABLE_SYSCALL {
        nonimpl(mips)?;
        panic!();
    } else {
        SYSCALL_LOOKUP_TABLE[mips.regs[2] as usize](mips)?;
        return Ok(());
    }
}

// Function implementations for each of the syscalls

// Applies to syscall 0 (since arrays are zero-indexed) and anything not yet implemented.
fn nonimpl(_mips: &mut Mips) -> Result<(), ExecutionErrors>{
    println!("The syscall you attempted to use has not yet been implemented.");
    return Err(ExecutionErrors::SyscallInvalidSyscallNumber);
}

// Syscall 1 - Print the value in $a0 ($4) to the screen.
fn sys_print_int(mips: &mut Mips) -> Result<(), ExecutionErrors>{
    print!("{}", mips.regs[4]);
    return Ok(())
}

// Syscall 2
// Syscall 3
// Syscall 4 - Print string; writes the null-terminated value pointed to by $a0 ($4) to screen.
fn sys_print_string(mips: &mut Mips) -> Result<(), ExecutionErrors>{
    let mut str_vec = vec![];
            let mut i = 0;
            loop {
                let read_char = mips.read_b(mips.regs[4] + i)?;
                if read_char == 0 {
                    break;
                }
                str_vec.push(read_char as char);
                i += 1;
            };
            print!("{}", str_vec.iter().collect::<String>());
            Ok(())
}
// Syscall 4
// Syscall 5
// Syscall 6
// Syscall 7
// Syscall 8
// Syscall 9
// Syscall 10 - System exit (raises a program complete execution event)
fn sys_exit(_mips: &mut Mips) -> Result<(), ExecutionErrors>{
    return Err(ExecutionErrors::Event{ event: ExecutionEvents::ProgramComplete });
}
// Syscall 11 - Print char: writes the char in $a0 ($4) to the screen.
fn sys_print_char(mips: &mut Mips) -> Result<(), ExecutionErrors>{
    if let Some(c) = std::char::from_u32(mips.regs[4]) {
        print!("{}", c);
    }
    Ok(())
}