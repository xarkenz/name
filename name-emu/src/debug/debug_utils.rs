use std::{collections::HashMap, sync::LazyLock};

use name_core::{
    elf_def::MIPS_ADDRESS_ALIGNMENT,
    instruction::{information::InstructionInformation, instruction_set::INSTRUCTION_SET},
    structs::{ExecutionStatus, LineInfo, Memory, Processor},
};

use crate::debug::debugger_functions::*;

static INSTRUCTION_LOOKUP: LazyLock<HashMap<u32, &'static InstructionInformation>> =
    LazyLock::new(|| {
        INSTRUCTION_SET
            .iter()
            .map(|instr| (instr.lookup_code(), instr))
            .collect()
    });

use crate::fetch::fetch;
use crate::simulator_helpers::generate_err;

use std::io::{self, Write};

pub fn single_step(
    lineinfo: &Vec<LineInfo>,
    cpu: &mut Processor,
    memory: &mut Memory,
    db_state: &DebuggerState,
) -> Result<ExecutionStatus, String> {
    // ideally, a break instruction is physically injected into the code and everything works politely from there without extra shenaniganery.
    // however, for now, passing a breakpoint vector in and looping through it will have to do
    if cpu.pc > memory.text_end || cpu.pc < memory.text_start {
        return Err(generate_err(
            lineinfo,
            cpu.pc - MIPS_ADDRESS_ALIGNMENT,
            "Program fell off bottom.",
        ));
    }

    // println!("{}", cpu.pc);

    // Fetch
    let raw_instruction = fetch(&cpu.pc, &memory)?;
    let instr_info = INSTRUCTION_LOOKUP
        .get(&raw_instruction.get_lookup())
        .ok_or(generate_err(
            lineinfo,
            cpu.pc - MIPS_ADDRESS_ALIGNMENT,
            format!("Failed instruction fetch").as_str(),
        ))?;

    cpu.pc += MIPS_ADDRESS_ALIGNMENT;

    // Execute
    let instruction_result = (instr_info.implementation)(cpu, memory, raw_instruction);

    // The $0 register should never have been permanently changed. Don't let it remain changed.
    cpu.general_purpose_registers[0] = 0;

    // check if there's a breakpoint on the line AFTER the current one (hence the + MIPS_ADDRESS_ALIGNMENT)
    for bp in &db_state.breakpoints {
        if cpu.pc + MIPS_ADDRESS_ALIGNMENT == bp.address {
            return Ok(ExecutionStatus::Break);
        }
    }

    // The instruction result contains an enum value representing whether or not "execution should stop now".
    match instruction_result {
        Ok(execution_status) => {
            return Ok(execution_status);
        }
        Err(e) => {
            return Err(generate_err(
                lineinfo,
                cpu.pc - MIPS_ADDRESS_ALIGNMENT,
                format!("{}", e).as_str(),
            ))
        }
    }
}

#[derive(Debug)]
pub struct Breakpoint {
    pub bp_num: u16, // why do you have 65535 breakpoints. do better
    pub line_num: u32,
    pub address: u32,
}

pub struct DebuggerState {
    pub breakpoints: Vec<Breakpoint>,
    pub global_bp_num: u16,
    pub global_list_loc: usize, // for the l command; the center of the output, so to speak
}

// This is the name debugger. Have fun...
pub fn debugger(
    lineinfo: &Vec<LineInfo>,
    memory: &mut Memory,
    cpu: &mut Processor,
) -> Result<(), String> {
    let mut db_state: DebuggerState = DebuggerState::new(Vec::new(), 0, 5);

    // static COMMANDS: &[(&str, DebugFn)] = &[
    //     ("help", )
    // ]
    println!("Welcome to the NAME debugger.\nFor a list of commands, type \"help\".");

    loop {
        print!("(name-db) ");
        io::stdout().flush().expect("Failed to flush stdout");

        // take in the command and split it up into arguments
        let mut user_input = String::new();
        match io::stdin().read_line(&mut user_input) {
            Ok(_) => {}
            Err(e) => eprintln!("stdin error: {e}"),
        };
        let db_args: Vec<String> = user_input
            .trim()
            .split(" ")
            .map(|s| s.to_string())
            .collect();

        // run the command the user inputsY 
        // this could be better
        match db_args[0].as_str() {
            "help" => {
                help_menu(db_args);
            }
            "q" => match quit_db() {
                // this looks absolutely ridiculous but i'm writing it this
                // way to hint at the lookup table i want to implement after
                // this juicy pull request
                Ok(()) => {
                    return Ok(());
                }
                Err(e) => {
                    eprintln!("{e}");
                }
            },
            "exit" => match quit_db() {
                Ok(()) => {
                    return Ok(());
                }
                Err(e) => {
                    eprintln!("{e}");
                }
            },
            "quit" => match quit_db() {
                Ok(()) => {
                    return Ok(());
                }
                Err(e) => {
                    eprintln!("{e}");
                }
            },
            "r" => loop {
                // this is supposed to always start program execution from the beginning.
                // idk how to make it do that rn
                match single_step(lineinfo, cpu, memory, &db_state) {
                    Ok(execution_status) => match execution_status {
                        ExecutionStatus::Continue => {}
                        ExecutionStatus::Break => {
                            match lineinfo
                                .iter()
                                .find(|&line| line.start_address == cpu.pc + MIPS_ADDRESS_ALIGNMENT)
                            {
                                Some(line) => {
                                    println!("Breakpoint at line {} reached.", line.line_number);
                                }
                                None => {
                                    eprintln!("Illegal state during single-step (lineinfo could not be located for current PC 0x{:x})", cpu.pc + MIPS_ADDRESS_ALIGNMENT);
                                }
                            }
                            break;
                        }
                        ExecutionStatus::Complete => return Ok(()),
                    },
                    Err(e) => return Err(e),
                };
            },
            "c" => loop {
                match single_step(lineinfo, cpu, memory, &db_state) {
                    Ok(execution_status) => match execution_status {
                        ExecutionStatus::Continue => {}
                        ExecutionStatus::Break => {
                            match lineinfo
                                .iter()
                                .find(|&line| line.start_address == cpu.pc + MIPS_ADDRESS_ALIGNMENT)
                            {
                                Some(line) => {
                                    println!("Breakpoint at line {} reached.", line.line_number);
                                }
                                None => {
                                    eprintln!("Illegal state during single-step (lineinfo could not be located for current PC 0x{:x})", cpu.pc + MIPS_ADDRESS_ALIGNMENT);
                                }
                            }
                            break;
                        }
                        ExecutionStatus::Complete => return Ok(()),
                    },
                    Err(e) => return Err(e),
                };
            },
            "s" => {
                // repetition bad...
                match single_step(lineinfo, cpu, memory, &db_state) {
                    Ok(execution_status) => match execution_status {
                        ExecutionStatus::Continue => {}
                        ExecutionStatus::Break => {
                            match lineinfo
                                .iter()
                                .find(|&line| line.start_address == cpu.pc + MIPS_ADDRESS_ALIGNMENT)
                            {
                                Some(line) => {
                                    println!("Breakpoint at line {} reached.", line.line_number);
                                }
                                None => {
                                    eprintln!("Illegal state during single-step (lineinfo could not be located for current PC 0x{:x})", cpu.pc + MIPS_ADDRESS_ALIGNMENT);
                                }
                            }
                        }
                        ExecutionStatus::Complete => return Ok(()),
                    },
                    Err(e) => return Err(e),
                };
            }
            "l" => match list_text(lineinfo, &mut db_state, &db_args) {
                Ok(_) => {
                    continue;
                }
                Err(e) => {
                    eprintln!("{e}");
                }
            }
            "p" => match print_register(cpu, &db_args) {
                Ok(()) => {
                    continue;
                }
                Err(e) => {
                    eprintln!("{e}");
                }
            },
            "pa" => match print_all_registers(cpu, &db_args) {
                Ok(()) => {
                    continue;
                }
                Err(e) => {
                    eprintln!("{e}");
                }
            }
            "m" => match modify_register(cpu, &db_args) {
                Ok(()) => {
                    continue;
                }
                Err(e) => {
                    eprintln!("{e}");
                }
            }
            "pb" => {
                db_state.print_all_breakpoints();
            }
            "b" => match db_state.add_breakpoint(lineinfo, &db_args) {
                Ok(_) => {
                    continue;
                }
                Err(e) => {
                    eprintln!("{e}");
                }
            }
            "del" => match db_state.remove_breakpoint(&db_args) {
                Ok(_) => {
                    continue;
                }
                Err(e) => {
                    eprintln!("{e}");
                }
            }
            _ => println!("Option not recognized. Use \"help\" to view accepted options."),
        };
    }
}
