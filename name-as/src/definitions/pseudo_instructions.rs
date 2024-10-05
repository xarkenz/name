use crate::definitions::structs::PseudoInstruction;
use crate::definitions::expandables::*;

pub(crate) const PSEUDO_INSTRUCTION_SET: &[PseudoInstruction] = &[
    PseudoInstruction {
        mnemonic: "li",
        expand: expand_li,
    },
    PseudoInstruction {
        mnemonic: "la",
        expand: expand_la,
    },
    PseudoInstruction {
        mnemonic: "move",
        expand: expand_move,
    },
    // this pseudoinstruction doesn't seem to be officially defined but it's w/e for now
    PseudoInstruction {
        mnemonic: "bnez",
        expand: expand_bnez,
    },
];