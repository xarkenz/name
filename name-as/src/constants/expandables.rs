use std::fmt;

use crate::constants::structs::PseudoInstruction;

pub(crate) trait Expandable: fmt::Debug {
    fn expand(&self, input: &str) -> String;
}

impl Expandable for PseudoInstruction {
    fn expand(&self, input: &str) -> String {
        return String::new();
    }
}