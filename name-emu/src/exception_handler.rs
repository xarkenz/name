use name_core::{
    exception::definitions::ExceptionType,
    structs::{LineInfo, ProgramState},
};

use crate::{
    debug::debug_utils::handle_breakpoint, simulator_helpers::generate_err,
    syscall_handler::handle_syscall,
};

/// The exception handler is invoked whenever an exception has occurred.
/// Some common exceptions include breakpoints, syscalls, and arithmetic overflow.
/// It takes a mutable program state and matches on the exception type - then, it resets state if possible.
pub fn handle_exception(program_state: &mut ProgramState, lineinfo: &Vec<LineInfo>) {
    // In order to invoke this function, certain values (like exception_level == 1) are already assumed.

    // Attempt to recognize the exception that occurred
    let exception_type: ExceptionType;
    match ExceptionType::try_from(program_state.cp0.get_exc_code()) {
        Ok(exc_type) => exception_type = exc_type,
        Err(e) => panic!("{e}"),
    }

    // Retrieve necessary values
    let epc: u32 = program_state.cp0.get_epc();

    dbg!(&exception_type);

    // Match on exception type to either error out or handle appropriately
    match exception_type {
        ExceptionType::AddressExceptionLoad => {
            // TODO: Detect difference between instructions like bad lw and bad/misaligned pc
            panic!("{}", generate_err(lineinfo, epc, "Illegal address provided for load/fetch; misaligned, unreachable, or unowned address."));
        }
        ExceptionType::AddressExceptionStore => {
            panic!("{}", generate_err(lineinfo, epc, "Illegal address provided on store operation; misaligned, unreachable, or unowned address."));
        }
        ExceptionType::BusFetch => {
            panic!("{}", generate_err(
                lineinfo,
                epc,
                "Failed to interpret instruction as word; Unrecognized bytes in ELF .text space.",
            ));
        }
        ExceptionType::BusLoadStore => {
            panic!(
                "{}",
                generate_err(lineinfo, epc, "Failed to store data in given address.")
            );
        }
        ExceptionType::Syscall => {
            // Invoke the syscall handler on program state
            handle_syscall(program_state);
        }
        ExceptionType::Breakpoint => {
            // Invoke the breakpoint handler on program state and lineinfo
            handle_breakpoint(program_state, lineinfo);
        }
        ExceptionType::ReservedInstruction => {
            panic!(
                "{}",
                generate_err(
                    lineinfo,
                    epc,
                    "Unrecognized bytes in ELF at program counter.",
                )
            );
        }
        ExceptionType::CoprocessorUnusable => {
            panic!(
                "{}",
                generate_err(
                    lineinfo,
                    epc,
                    "Attempted to access a coprocessor without correct operating mode.",
                )
            );
        }
        ExceptionType::ArithmeticOverflow => {
            // TODO: Differentiate between these
            panic!(
                "{}",
                generate_err(
                    lineinfo,
                    epc,
                    "Arithmetic overflow, underflow, or divide by zero detected on instruction.",
                )
            );
        }
        ExceptionType::Trap => {
            todo!("Not sure how we want trap to work yet.");
        }
        ExceptionType::FloatingPoint => {
            // Will be more useful once cp1 is implemented
            panic!(
                "{}",
                generate_err(lineinfo, epc, "Floating point exception occurred.")
            );
        }
    }

    // If the exception did not cause a crash, reset program state to reflect that execution will continue as normal
    program_state.recover_from_exception();
}
