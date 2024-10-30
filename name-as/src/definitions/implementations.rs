use std::fmt;

use crate::definitions::structs::LineComponent;

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
            LineComponent::Colon => write!(f, ":"),
        }
    }
}
