use std::{collections::HashMap, sync::LazyLock};
// use std::io::{self, Write};

// use crate::debug::debugger_methods::*;

use crate::{
    constants::MIPS_ADDRESS_ALIGNMENT,
    debug::{
        fetch::fetch,
        exception_handler::handle_exception,
    },
    exception::definitions::ExceptionType,
    instruction::{information::InstructionInformation, instruction_set::INSTRUCTION_SET, RawInstruction},
    structs::{LineInfo, OperatingSystem, ProgramState},
};

static INSTRUCTION_LOOKUP: LazyLock<HashMap<u32, &'static InstructionInformation>> =
    LazyLock::new(|| {
        INSTRUCTION_SET
            .iter()
            .map(|instr| (instr.lookup_code(), instr))
            .collect()
    });

pub fn single_step(
    _lineinfo: &Vec<LineInfo>,
    program_state: &mut ProgramState,
) -> () {
    if !program_state
        .memory
        .allows_execution_of(program_state.cpu.pc)
    {
        program_state.set_exception(ExceptionType::AddressExceptionLoad);
        return;
    }

    // check if there's a breakpoint before instruction on the line is executed
    // TODO: implement break instruction. check after fetch.

    // Fetch
    let raw_instruction = fetch(program_state);
    let instr_info = match INSTRUCTION_LOOKUP.get(&raw_instruction.get_lookup()) {
        Some(info) => info,
        None => {
            program_state.set_exception(ExceptionType::ReservedInstruction);
            return;
        }
    };

    program_state.cpu.pc += MIPS_ADDRESS_ALIGNMENT;

    // Execute the instruction; program_state is modified.
    if false
    /* Allowing for some later verbose mode */
    {
        println!("Executing {}", instr_info.mnemonic);
    }
    let _ = (instr_info.implementation)(program_state, raw_instruction);

    // The $0 register should never have been permanently changed. Don't let it remain changed.

    program_state.cpu.general_purpose_registers[0] = 0;
}

/// Executes only the next line of code. Invoked by "s" in the CLI.
// Also called by continuously_execute
pub fn db_step(
    lineinfo: &Vec<LineInfo>,
    program_state: &mut ProgramState,
    os: &mut OperatingSystem,
    debugger_state: &mut DebuggerState,
) -> Result<(), String> {
    let prev_funct_code = match program_state.memory.read_byte(program_state.cpu.pc - 1) {
        Ok(byte) => byte & 0b111111,
        Err(e) => {
            return Err(format!("{e}"))
        }
    };

    if prev_funct_code == 0b001101 {
        let bp_num = match get_bp_num(program_state) {
            Ok(idx) => idx,
            Err(e) => return Err(format!("{e}"))
        };

        let bp = match debugger_state.breakpoints.get(bp_num as usize) {
            Some(bp) => bp,
            None => {
                return Err(format!("Breakpoint {} not found in memory.", bp_num));
            }
        };

        /* This is just copy paste from single_step with edits to make it make sense for our case */

        // Fetch the instruction replaced by the breakpoint
        let raw_instruction = RawInstruction::new(bp.replaced_instruction); // lol
        let instr_info = match INSTRUCTION_LOOKUP.get(&raw_instruction.get_lookup()) {
            Some(info) => info,
            None => {
                program_state.set_exception(ExceptionType::ReservedInstruction);
                return Err(format!("Reserved instruction reached. (My code is bad so the program state has been changed as a result. Lord help us)"));
            }
        };

        // program_state.cpu.pc += MIPS_ADDRESS_ALIGNMENT;

        // Execute the instruction; program_state is modified.
        if false
        /* Allowing for some later verbose mode */
        {
            println!("Executing {}", instr_info.mnemonic);
        }
        let _ = (instr_info.implementation)(program_state, raw_instruction);

        // The $0 register should never have been permanently changed. Don't let it remain changed.
        program_state.cpu.general_purpose_registers[0] = 0;


        Ok(()) 
    } else { 
        single_step(lineinfo, program_state);
        if program_state.is_exception() {
            // todo!("Handle exception");
            // return Err("exceptionnnnnnnnn".to_string())
            handle_exception(program_state, os, lineinfo, debugger_state);
        }
        Ok(())
    }
}

fn get_bp_num(program_state: &ProgramState) -> Result<u32, String> {
    let bitmask: u32 = 0b00000011111111111111111111000000;
    let mut bp_num = 0;

    for i in 0..4 {
        let mut word_portion = match program_state.memory.read_byte(program_state.cpu.pc - MIPS_ADDRESS_ALIGNMENT + i){
            Ok(byte) => byte as u32,
            Err(e) => {
                return Err(format!("B{e}"))
            }
        };

        word_portion &= (bitmask >> (24 - 8*i)) & 0xff;

        bp_num |= word_portion << 24 - 8*i;
    }

    bp_num >>= 6;

    return Ok(bp_num);
}


#[derive(Debug)]
pub struct Breakpoint {
    // pub bp_num: u16, // why do you have 65535 breakpoints. do better
    pub line_num: u32,
    pub address: u32,
    pub replaced_instruction: u32,
}

pub struct DebuggerState {
    pub global_bp_num: usize, // point to the first available empty space in the breakpoint vector
    pub breakpoints: Vec<Breakpoint>, // indexed by bp_num
    // pub replaced_instructions: Vec<u32>, // also indexed by bp num
    pub global_list_loc: usize, // for the l command; like the center of the output
}

// pub type DebugFn = fn(&Vec<LineInfo>, &mut Memory, &mut Processor, &Vec<Breakpoint>) -> Result<(), String>;