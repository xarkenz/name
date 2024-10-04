use crate::definitions::expandables::*;
use crate::definitions::structs::PseudoInstruction;

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
