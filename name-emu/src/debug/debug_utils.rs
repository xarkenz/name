use name_const::elf_def::MIPS_ADDRESS_ALIGNMENT;
use name_const::structs::{LineInfo, Memory, Processor};
use name_const::constants::REGISTERS;

use crate::decode::{decode, InstructionFn};

use crate::definitions::structs::ExecutionStatus;
use crate::fetch::fetch;
use crate::simulator_helpers::generate_err;

use std::io::{self, Write};


pub fn single_step(lineinfo: &Vec<LineInfo>, cpu: &mut Processor, memory: &mut Memory, bps: &Vec<Breakpoint>) -> Result<ExecutionStatus, String> {
    // passing a breakpoints vector into this function is a very messy way of doing this, i'm aware,,,
    // ideally, a break instruction is physically injected into the code and everything works politely from there without extra shenaniganery.
    // however, for now, this will have to do
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

    // check breakpoint after instruction on the line is executed
    for bp in bps{
        if cpu.pc == bp.address { 
            // println!("Breakpoint at line {} reached. (This ran in single_step())", bp.line_num);
            return Ok(ExecutionStatus::Break);
        }
    }

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
    pub address: u32,
}

impl Breakpoint {
    pub fn new(bp_num: u16, line_num: u32, lineinfo: &Vec<LineInfo>) -> Self {
        Breakpoint {
            bp_num,
            line_num,
            address: {
                match lineinfo.iter().find(|&line| line.line_number == line_num){
                    Some(line) => line.start_address,
                    None => { 
                        eprintln!("Breakpoint not found in memory????"); 
                        0
                    }
                }
            },
        }
    }
    // assembler::add_label is not the solution to male loneliness
}

// This is the name debugger. Have fun...
pub fn debugger(lineinfo: &Vec<LineInfo>, memory: &mut Memory, cpu: &mut Processor) -> Result<(), String> {
    let mut breakpoints: Vec<Breakpoint> = Vec::new();
    let mut global_bp_num: u16 = 0;

    println!("Welcome to the NAME debugger.");
    println!("For a list of commands, type \"help\".");

    // i have not written real rust before. please forgive me
    loop {
        print!("(name-db) ");
        io::stdout().flush().expect("Failed to flush stdout");  // i took cs 3377 and i still don't know why this is a thing

        let mut user_input = String::new();
        match io::stdin().read_line(&mut user_input) {
            Ok(_) => {},
            Err(e) => eprintln!("stdin error: {e}"),
        };
        let user_input: Vec<&str> = user_input.trim().split(" ").collect(); 

        // this could be better mayb
        match user_input[0] {
            "help" => { help_menu(); },
            "q" => return Ok(()),
            "exit" => return Ok(()),
            "quit" => return Ok(()),
            "r" => {
                // this is supposed to always start program execution from the beginning.
                // idk how to make it do that rn
                // maybe these can secretly be the same thing and we just keep it here to make it look liek gdb :shrug:
                loop {
                    match single_step(lineinfo, cpu, memory, &breakpoints){
                        Ok(execution_status) => match execution_status {
                            ExecutionStatus::Continue => {},
                            ExecutionStatus::Break => {    
                                match lineinfo.iter().find(|&line| line.start_address == cpu.pc){
                                    Some(line) => { println!("Breakpoint at line {} reached.", line.line_number); }
                                    None => { eprintln!("I don't know how to describe this error. Good luck"); }
                                }
                                break; 
                            },
                            ExecutionStatus::Complete => return Ok(()),
                        },
                        Err(e) => return Err(e),
                    };
                }
            },
            "c" => {
                loop {
                    match single_step(lineinfo, cpu, memory, &breakpoints){
                        Ok(execution_status) => match execution_status {
                            ExecutionStatus::Continue => {},
                            ExecutionStatus::Break => {    
                                match lineinfo.iter().find(|&line| line.start_address == cpu.pc){
                                    Some(line) => { println!("Breakpoint at line {} reached.", line.line_number); }
                                    None => { eprintln!("I don't know how to describe this error. Good luck"); }
                                }
                                break; 
                            },
                            ExecutionStatus::Complete => return Ok(()),
                        },
                        Err(e) => return Err(e),
                    };
                }
            }
            "l" => {
                for line in lineinfo {
                    println!("{:>3} #{:08x}  {}", line.line_number, line.start_address, line.content);
                }
            },
            "s" => {
                match single_step(lineinfo, cpu, memory, &breakpoints){
                    Ok(execution_status) => match execution_status {
                        ExecutionStatus::Continue => {},
                        ExecutionStatus::Break => { 
                            match lineinfo.iter().find(|&line| line.start_address == cpu.pc){
                                Some(line) => { println!("Breakpoint at line {} reached.", line.line_number); }
                                None => { eprintln!("I don't know how to describe this error. Good luck"); }
                            }
                        },
                        ExecutionStatus::Complete => return Ok(()),
                    },
                    Err(e) => return Err(e),
                };
            }
            "p" => {
                if user_input.len() != 2 {
                    eprintln!("p expects 1 argument, received {}", user_input.len()-1);
                    continue;
                }
                // #[allow(unused_assignments)]  // oh boy
                match REGISTERS.iter().position(|&x| x == user_input[1]){
                    Some(register) => {
                        println!("Value in register {} is {:08x}", register, cpu.general_purpose_registers[register]);
                    },
                    None => {
                        println!("{} is not a valid register.", user_input[1]);
                        continue;
                    }
                }
            }
            "pa" => {
                if user_input.len() != 1 {
                    // this outputs a lot so make sure the user actually meant to type pa and not pb or p or something
                    eprintln!("pa expects 0 arguments, received {}", user_input.len()-1); 
                    continue;
                }
                for register in REGISTERS {
                    let idx: usize = REGISTERS.iter().position(|&x| x == register).unwrap();
                    println!("{:>5}: {:08x}", register, cpu.general_purpose_registers[idx]);
                }
            }
            "pb" => {
                println!("BP_NUM: LINE_NUM");
                for breakpoint in &breakpoints {
                    println!("{:>6}: {}", breakpoint.bp_num, breakpoint.line_num);  // format this...?
                }
            },
            "b" => {
                // i know there's probably better error handling but i don't have time
                // to read the whole rust book rn soz
                if user_input.len() != 2 {
                    eprintln!("b expects 1 argument, received {}", user_input.len()-1);
                    continue;
                }
                let line_num: u32 = user_input[1].parse().expect("b takes 32-bit unsigned int as input");

                if line_num > lineinfo.len().try_into().unwrap(){
                    eprintln!("{} exceeds number of lines in program.", line_num);  // something like that
                }

                global_bp_num += 1;
                breakpoints.push(Breakpoint::new(global_bp_num, line_num, lineinfo));
                println!("Successfully added breakpoint {} at line {}.", global_bp_num, line_num);
            }
            "del" => {
                if user_input.len() != 2 {
                    eprintln!("del expects 1 argument, received {}", user_input.len()-1);
                    continue;
                }
                let bp_num: u16 = user_input[1].parse().expect("del takes a 16-bit unsigned int as input");
    
                // i KNOW this can be better
                if let Some(index) = breakpoints.iter().position(|brpt| brpt.bp_num == bp_num) {
                    let removed_element = breakpoints.remove(index);
                    println!("Removed {:?}", removed_element);
                    global_bp_num -= 1;
                } else {
                    eprintln!("Breakpoint with bp_num {} not found", bp_num);
                }
            }
            _ => println!("Option not recognized. Type \"help\" to view accepted options.")
        };
    }
}

fn help_menu(){
    println!("help - Display this menu.");
    println!("r - Begin execution of program.");
    println!("c - Continue program execution until the next breakpoint.");
    println!("s - Execute only the next instruction.");
    println!("l - Print the entire program. (this functionality will be much improved later)");
    println!("p - Print the value of a register at the current place in program execution.");
    println!("pa - Print value of ALL registers at once.");
    println!("pb - Print all breakpoints.");
    println!("b [N] - Insert a breakpoint at line number N.");
    println!("del [N] - Delete breakpoint number N.");
    println!("q - Exit (quit) debugger.");
}

// fn run_wrapper(lineinfo: &Vec<LineInfo>, cpu: &mut Processor, memory: &mut Memory, bps: Vec<Breakpoint>) -> Result<ExecutionStatus, String>{
//     match single_step(lineinfo, cpu, memory, &bps){
//         Ok(execution_status) => match execution_status {
//             ExecutionStatus::Continue => {},
//             ExecutionStatus::Break => { 
//                 match lineinfo.iter().find(|&line| line.start_address == cpu.pc){
//                     Some(line) => { println!("Breakpoint at line {} reached.", line.line_number); }
//                     None => { eprintln!("I don't know how to describe this error. Good luck"); }
//                 }
//                 Ok(ExecutionStat)
//             },
//             ExecutionStatus::Complete => return Ok(ExecutionStatus::Complete),
//         },
//         Err(e) => return Err(e),
//     };
// }