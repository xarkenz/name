use crate::constants::structs::PseudoInstruction;
use crate::constants::expandables::*;

pub(crate) const PSEUDO_INSTRUCTION_SET: &[PseudoInstruction] = &[
    PseudoInstruction {
        mnemonic: "li",
        expand: expand_li,
    },
    PseudoInstruction {
        mnemonic: "la",
        expand: expand_la,
    },
];