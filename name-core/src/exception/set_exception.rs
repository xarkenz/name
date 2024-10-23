use crate::structs::ProgramState;
use super::{definitions::ExceptionType, register_set::to_register, registers::Register};

impl ProgramState {
    /// When an exception is triggered in the MIPS architecture, 
    /// values in Coprocessor 0 registers are set to indicate the exception state to the operating system.
    /// This impl modifies the ProgramState based on a passed ExceptionType, 
    /// filling in the Status (12) and Cause (13) registers appropriately.
    pub fn set_exception(&mut self, _exception_type: ExceptionType) -> () {
        // Some values are set no matter what to indicate an exception state:
        // Extract the current operating mode. 
        self.cp0.set_exception_level(1 as u32);  // This indicates to the OS that an exception is being handled.
        self.cp0.registers[/* Exception Program Counter */ 14 ] = self.cpu.pc;  // This register contains the PC of where the exception occurred.

    }
}