use std::fmt;

use crate::constants::structs::{InstructionInformation, LineComponent};

impl InstructionInformation {
    pub fn get_mnemonic(&self) -> String {
        return self.mnemonic.to_string();
    }
}

// I wanted .to_string() to work
impl fmt::Display for LineComponent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LineComponent::Mnemonic(m) => write!(f, "{}", m),
            LineComponent::Register(r) => write!(f, "{}", r),
            LineComponent::Immediate(i) => write!(f, "{}", i),
            LineComponent::Identifier(i) => write!(f, "{}", i),
            LineComponent::Label(l) => write!(f, "{}", l),
            LineComponent::Directive(d) => write!(f, "{}", d),
            LineComponent::DoubleQuote(d) => write!(f, "{}", d),
        }
    }
}