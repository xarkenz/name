use name_core::{
    structs::{ProgramState, Register::V0},
    syscalls::{SyscallFn, SYSCALL_TABLE},
};

/// This module contains the logic for handling syscalls.
/// It is invoked by the exception handler.
pub fn handle_syscall(program_state: &mut ProgramState) -> () {
    // Retrieve syscall number from $v0
    let syscall_num: usize = program_state.cpu.general_purpose_registers[V0 as usize] as usize;

    // Attempt to retrieve syscall function for syscall number
    let syscall_fun: SyscallFn;
    match SYSCALL_TABLE[syscall_num] {
        Some(fun) => syscall_fun = fun,
        None => {
            // TODO: Use a function which sets the proper values in cp0 for us
            throw_syscall_error(format!(
                "Unrecognized syscall number {syscall_num} provided."
            ));
            // This unreachable!() signals to the compiler that every throw_syscall_error will cause an exit
            unreachable!();
        }
    }

    // Execute associated syscall
    match syscall_fun(program_state) {
        Ok(_) => (),
        Err(e) => throw_syscall_error(e),
    };
}

/// This helper is made to handle errors occurring within the syscall handler (separate from coprocessor 0).
fn throw_syscall_error(format: String) -> () {
    eprintln!("[*] Error in syscall handler:\n - {format}");
    // TODO: Replace panic with a more graceful exit
    panic!();
}
