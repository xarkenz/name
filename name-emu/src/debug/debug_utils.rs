use name_const::elf_def::MIPS_ADDRESS_ALIGNMENT;
use name_const::structs::{LineInfo, Memory, Processor};
// use name_const::constants::REGISTERS; // use for p command

use crate::decode::{decode, InstructionFn};

use crate::definitions::structs::ExecutionStatus;
use crate::fetch::fetch;
use crate::simulator_helpers::generate_err;

use std::io::{self, Write};


pub fn single_step(lineinfo: &Vec<LineInfo>, cpu: &mut Processor, memory: &mut Memory) -> Result<ExecutionStatus, String> {
    if cpu.pc > memory.text_end || cpu.pc < memory.text_start {
        return Err(generate_err(lineinfo, cpu.pc-4, "Program fell off bottom."));
    }

    // Fetch
    let fetched_instruction: u32 = fetch(&cpu.pc, &memory)?;

    cpu.pc += MIPS_ADDRESS_ALIGNMENT;

    // Decode
    let decoded_instruction_fn: InstructionFn = match decode(&fetched_instruction) {
        Ok(fun) => fun,
        Err(e) => return Err(generate_err(lineinfo, cpu.pc-4, format!("Failed instruction fetch: {}.", e).as_str() )),
    };

    // Execute
    let instruction_result = decoded_instruction_fn(cpu, memory, fetched_instruction);

    // The $0 register should never have been permanently changed. Don't let it remain changed.
    cpu.general_purpose_registers[0] = 0;

    // The instruction result contains an enum value representing whether or not "execution should stop now".
    match instruction_result {
        Ok(execution_status) => {
            return Ok(execution_status);
        },
        Err(e) => return Err(generate_err(lineinfo, cpu.pc-4, format!("{}", e).as_str() )),
    }
}

// do we really need to put this another file riiiiight now
#[derive(Debug)]
pub struct Breakpoint {
    pub bp_num: u16, // why do you have 65535 breakpoints. do better
    pub line_num: u32,
}

impl Breakpoint {
    pub fn new(bp_num: u16, line_num: u32) -> Self {
        Breakpoint {
            bp_num,
            line_num,
        }
    }
}


pub fn debugger(lineinfo: &Vec<LineInfo>, memory: &mut Memory, cpu: &mut Processor) -> Result<(), String> {
    let mut breakpoints: Vec<Breakpoint> = Vec::new();
    let mut global_bp_num: u16 = 0;  // probably a better way to do this

    println!("Welcome to the NAME debugger.");
    println!("For a list of commands, type \"help\".");

    // i have not written real rust before. please forgive me
    loop {
        print!("(name-db) ");
        io::stdout().flush().expect("Failed to flush stdout");

        let mut user_input = String::new();
        let stdin = io::stdin();
        match stdin.read_line(&mut user_input) {
            Ok(_) => {},
            Err(e) => println!("Error: {e}"),
        };
        let user_input: Vec<&str> = user_input.trim().split(" ").collect(); 

        // this could be better mayb
        match user_input[0] {
            "help" => { help_menu(); },
            "q" => return Ok(()),
            "r" => {
                loop {
                    match single_step(lineinfo, cpu, memory){
                        Ok(execution_status) => match execution_status {
                            ExecutionStatus::Continue => {},
                            ExecutionStatus::Break => { break; },
                            ExecutionStatus::Complete => return Ok(()),
                        },
                        Err(e) => return Err(e),
                    };
                }
            },
            "l" => {
                for (idx, line) in lineinfo.iter().enumerate() {
                    println!("{} {}", idx+1, line.content);
                }
            },
            "n" => {
                match single_step(lineinfo, cpu, memory){
                    Ok(execution_status) => match execution_status {
                        ExecutionStatus::Continue => {},
                        ExecutionStatus::Break => { println!("broke"); },
                        ExecutionStatus::Complete => return Ok(()),
                    },
                    Err(e) => return Err(e),
                };
            }
            /*
            "p" => {
                if user_input.len() != 2 {
                    println!("p expects 1 argument, received {}", user_input.len()-1);
                    continue;
                }
                #[allow(unused_assignments)]
                let mut reg_idx: usize = 33;
                match REGISTERS.iter().position(|&x| x == user_input[1]){
                    Some(register) => {
                        reg_idx = register;
                    },
                    None => {
                        println!("{} is not a valid register.", user_input[1]);
                        continue;
                    }
                }

                println!("you want the value of register {}. wouldn't you like to know", reg_idx);
            }
            */
            "pb" => {
                println!("BP_NUM: LINE_NUM");
                for breakpoint in &breakpoints {
                    println!("{}: {}", breakpoint.bp_num, breakpoint.line_num);
                }
            },
            "b" => {
                // i know there's probably better error handling but i don't have time
                // to read the whole rust book rn soz
                if user_input.len() != 2 {
                    println!("b expects 1 argument, received {}", user_input.len()-1);
                    continue;
                }
                let line_num: u32 = user_input[1].parse().expect("b takes 32-bit unsigned int as input");
                global_bp_num += 1;
                breakpoints.push(Breakpoint::new(global_bp_num, line_num));
                // obviously make this work with the code later...
            }
            "del" => {
                if user_input.len() != 2 {
                    println!("b expects 1 argument, received {}", user_input.len()-1);
                    continue;
                }
                let bp_num: u16 = user_input[1].parse().expect("del takes a 16-bit unsigned int as input");
    
                // i KNOW this can be better
                if let Some(index) = breakpoints.iter().position(|brpt| brpt.bp_num == bp_num) {
                    let removed_element = breakpoints.remove(index);
                    println!("Removed {:?}", removed_element);
                    global_bp_num -= 1;
                } else {
                    println!("Breakpoint with bp_num {} not found", bp_num);
                }
            }
            _ => println!("Option not recognized. Type \"help\" to view accepted options.")
        };
    }
}


fn help_menu(){
    println!("help - Display this menu.");
    println!("r - Run the program normally.");
    println!("c - Continue program execution until the next breakpoint.");
    println!("n - Execute only the next instruction.");
    println!("l - Print the entire program. (this functionality will be much improved later)");
    // println!("p - Print the value of a register at the current place in program execution.");
    println!("pb - Print all breakpoints.");
    println!("b [N] - Insert a breakpoint at line number N.");
    println!("del [N] - Delete breakpoint number N.");
    println!("q - Exit (quit) debugger.");
}