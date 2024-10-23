use std::{collections::HashMap, sync::LazyLock};

use name_core::{
    constants::REGISTERS,
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
            cpu.pc - 4,
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
    pub global_list_loc: usize // for the l command; the center of the output, so to speak
}

impl Breakpoint {
    pub fn new(bp_num: u16, line_num: u32, lineinfo: &Vec<LineInfo>) -> Self {
        Breakpoint {
            bp_num,
            line_num,
            address: {
                match lineinfo.iter().find(|&line| line.line_number == line_num) {
                    Some(line) => line.start_address,
                    None => {
                        eprintln!("Breakpoint not found in memory.");
                        0
                    }
                }
            },
        }
    }
    // assembler::add_label is not the solution to male loneliness
}

impl DebuggerState {
    pub fn new(breakpoints: Vec<Breakpoint>, global_bp_num: u16, global_list_loc: usize) -> Self {
        DebuggerState {
            breakpoints,
            global_bp_num,
            global_list_loc
        }
    }

    pub fn add_breakpoint(&mut self, line_num: u32, lineinfo: &Vec<LineInfo>){
        self.global_bp_num += 1;
        self.breakpoints.push(Breakpoint::new(self.global_bp_num, line_num, lineinfo));
        println!(
            "Successfully added breakpoint {} at line {}.",
            self.global_bp_num, line_num
        );
    }

    pub fn remove_breakpoint(&mut self, bp_num: u16){
        // i KNOW this can be better
        if let Some(index) = self.breakpoints.iter().position(|brpt| brpt.bp_num == bp_num) {
            let removed_element = self.breakpoints.remove(index);
            println!("Removed {:?}", removed_element);
            self.global_bp_num -= 1;
        } else {
            eprintln!("Breakpoint with bp_num {} not found", bp_num);
        }
    }
}


// This is the name debugger. Have fun...
pub fn debugger(
    lineinfo: &Vec<LineInfo>,
    memory: &mut Memory,
    cpu: &mut Processor,
) -> Result<(), String> {
    let mut db_state: DebuggerState = DebuggerState::new(
        Vec::new(), 
        0, 
        5
    );

    // static COMMANDS: &[(&str, DebugFn)] = &[
    //     ("help", )
    // ]

    println!("Welcome to the NAME debugger.");
    println!("For a list of commands, type \"help\".");

    loop {
        print!("(name-db) ");
        io::stdout().flush().expect("Failed to flush stdout");

        // take in the command and split it up into arguments
        let mut user_input = String::new();
        match io::stdin().read_line(&mut user_input) {
            Ok(_) => {}
            Err(e) => eprintln!("stdin error: {e}"),
        };
        let db_args: Vec<String> = user_input.trim().split(" ").map(|s| s.to_string()).collect();

        // run the 
        // this could be better
        match db_args[0].as_str() {
            "help" => {
                help_menu(db_args);
            }
            "q"    => return Ok(()),
            "exit" => return Ok(()),
            "quit" => return Ok(()),
            "r" => loop {
                // this is supposed to always start program execution from the beginning.
                // idk how to make it do that rn
                match single_step(lineinfo, cpu, memory, &db_state) {
                    Ok(execution_status) => match execution_status {
                        ExecutionStatus::Continue => {}
                        ExecutionStatus::Break => {
                            match lineinfo.iter().find(|&line| {
                                line.start_address == cpu.pc + MIPS_ADDRESS_ALIGNMENT
                            }) {
                                Some(line) => {
                                    println!(
                                        "Breakpoint at line {} reached.",
                                        line.line_number
                                    );
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
            }
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
            "l" => {
                // this error is terrible and i don't know why it's happening but i'll fix it in the next commit
                /*
                match list_text(lineinfo, memory, cpu, &breakpoints, &db_args, &mut global_list_loc) => {
                    Ok(_) => { continue; }
                    Err(e) => { eprintln(e); }
                };
                */
            }
            "p" => {
                if db_args.len() < 2 {
                    eprintln!(
                        "p expects a non-zero argument, received {}",
                        db_args.len() - 1
                    );
                    continue;
                }

                if db_args[1].chars().nth(0) != Some('$') {
                    eprintln!("Congrats! You discovered an unimplemented feature... or you forgot the dollar sign on your register.");
                    continue;
                }

                for register in db_args[1..].to_vec() {
                    // #[allow(unused_assignments)]  // oh boy
                    match REGISTERS.iter().position(|&x| x == register) {
                        Some(found_register) => {
                            println!(
                                "Value in register {} is {:08x}",
                                found_register, cpu.general_purpose_registers[found_register]
                            );
                        }
                        None => {
                            println!("{} is not a valid register.", db_args[1]);
                            continue;
                        }
                    }
                }
            }
            "pa" => {
                if db_args.len() != 1 {
                    // this outputs a lot so make sure the user actually meant to type pa and not pb or p or something
                    eprintln!("pa expects 0 arguments, received {}", db_args.len() - 1);
                    continue;
                }
                for register in REGISTERS {
                    let idx: usize = REGISTERS.iter().position(|&x| x == register).unwrap();
                    println!(
                        "{:>5}: {:08x}",
                        register, cpu.general_purpose_registers[idx]
                    );
                }
            }
            "m" => {
                if db_args.len() != 3 {
                    eprintln!("m expects 2 arguments, received {}", db_args.len() - 1);
                    continue;
                }

                let register = match REGISTERS.iter().position(|&x| x == db_args[1]) {
                    Some(found_register) => found_register,
                    None => {
                        eprintln!("First argument to m must be a register. (Did you include the dollar sign?)");
                        continue;
                    }
                };

                let parsed_u32 = match db_args[2].parse::<u32>() {
                    Ok(found) => found,
                    Err(e) => {
                        eprintln!("{e}");
                        continue;
                    }
                };

                let original_val = cpu.general_purpose_registers[register];
                cpu.general_purpose_registers[register] = parsed_u32;
                println!(
                    "Successfully modified value in register {} from {} to {}.",
                    db_args[1], original_val, parsed_u32
                );
            }
            "pb" => {
                println!("BP_NUM: LINE_NUM");
                for breakpoint in &db_state.breakpoints {
                    println!("{:>6}: {}", breakpoint.bp_num, breakpoint.line_num);
                }
            }
            "b" => {
                if db_args.len() != 2 {
                    eprintln!("b expects 1 argument, received {}", db_args.len() - 1);
                    continue;
                }
                
                let line_num: u32 = db_args[1]
                    .parse()
                    .expect("b takes 32-bit unsigned int as input");

                if line_num > lineinfo.len().try_into().unwrap() {
                    eprintln!("{} exceeds number of lines in program.", line_num);
                    // something like that
                }

                db_state.add_breakpoint(line_num, lineinfo);
            }
            "del" => {
                if db_args.len() != 2 {
                    eprintln!("del expects 1 argument, received {}", db_args.len() - 1);
                    continue;
                }

                let bp_num: u16 = db_args[1]
                    .parse()
                    .expect("del takes a 16-bit unsigned int as input");

                db_state.remove_breakpoint(bp_num);
            }
            _ => println!("Option not recognized. Use \"help\" to view accepted options."),
        };
    }
}
