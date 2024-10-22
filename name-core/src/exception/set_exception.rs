use crate::structs::ProgramState;
use super::definitions::ExceptionType;

impl ProgramState {
    /// When an exception is triggered in the MIPS architecture, 
    /// values in Coprocessor 0 registers are set to indicate the exception state to the operating system.
    /// This impl modifies the ProgramState based on a passed ExceptionType, 
    /// filling in the Status (12) and Cause (13) registers appropriately.
    pub fn set_exception(&mut self, _exception_type: ExceptionType) -> () {
        // Some values are set no matter what to indicate an exception state:
        let _current_mode = (self.cp0.registers[/* Status [Current Mode] */ 12 ] >> 1) & 0x3;    // Extract the current operating mode. 
        self.cp0.registers[/* Status [EXL] */ 12 ] |= 1 << 1;                   // This indicates to the OS that an exception is being handled.
        self.cp0.registers[/* Exception Program Counter */ 14 ] = self.cpu.pc;  // This register contains the PC of where the exception occurred.

    }
}