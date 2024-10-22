use crate::structs::ProgramState;
use super::definitions::ExceptionType;

impl ProgramState {
    pub fn set_exception(&mut self, _exception_type: ExceptionType) -> () {
        ()
    }
}