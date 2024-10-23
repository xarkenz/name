use crate::structs::ProgramState;
use super::{constants::*, definitions::ExceptionType, register_set::to_register, registers::Register};

impl ProgramState {
    /// When an exception is triggered in the MIPS architecture, 
    /// values in Coprocessor 0 registers are set to indicate the exception state to the operating system.
    /// This impl modifies the ProgramState based on a passed ExceptionType, 
    /// filling in the Status (12) and Cause (13) registers appropriately.
    pub fn set_exception(&mut self, _exception_type: ExceptionType) -> () {
        // Some values are set no matter what to indicate an exception state:
        //
        // The EXL bit indicates to the OS that an exception is being handled.
        self.cp0.set_exception_level(EXCEPTION_BEING_HANDLED);
        // Exceptions are handled in Kernel Mode.
        self.cp0.set_current_mode(KERNEL_MODE);
        // The EPC register contains the PC of where the exception occurred.
        // If it already contains some other value important to our flow, we do not want to overwrite the address.
        if !self.is_exception() {
            self.cp0.set_epc(self.cpu.pc);
        }

        todo!("Match on ExceptionType");
    }
}