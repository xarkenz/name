use name_const::structs::{Memory, Processor};

pub type InstructionFn = fn(&mut Processor, &mut Memory);

pub fn decode(_instruction: &u32) -> InstructionFn {
    todo!("Implement instruction decoding");
}