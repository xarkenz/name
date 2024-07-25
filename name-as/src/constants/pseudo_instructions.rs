use super::structs::PseudoInstruction;

pub(crate) const PSEUDO_INSTRUCTION_SET: &[PseudoInstruction] = &[
    PseudoInstruction {
        mnemonic: "li",
        expansion: "ori     %1, %1, $zero",
    },
    PseudoInstruction {
        // NOPE this actually requires a lot of thought
        mnemonic: "la",
        expansion: "<FAILURE>"
    }
];