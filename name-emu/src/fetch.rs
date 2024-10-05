use name_core::{instruction::RawInstruction, structs::Memory};

// Fetch the next instruction and check that it's in acceptable memory space
pub fn fetch(fetch_address: &u32, memory: &Memory) -> Result<RawInstruction, String> {
    if fetch_address > &memory.text_end {
        return Err("Program fell off bottom.".to_string());
    } else if fetch_address < &memory.text_start {
        return Err("Program counter has reached unowned address space. You, the user, have made a grave mistake.".to_string());
    }

    let instr_index: usize = (fetch_address - &memory.text_start) as usize;

    let fetched_instruction =
        u32::from_be_bytes(match memory.text[instr_index..instr_index + 4].try_into() {
            Ok(val) => val,
            Err(e) => return Err(e.to_string()),
        });

    Ok(RawInstruction::new(fetched_instruction))
}
