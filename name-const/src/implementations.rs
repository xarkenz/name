use crate::structs::InstructionInformation;

impl InstructionInformation {
    pub fn get_mnemonic(&self) -> String {
        return self.mnemonic.to_string();
    }
}