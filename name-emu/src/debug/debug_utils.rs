use name_core::{
    constants::REGISTERS,
    elf_def::MIPS_ADDRESS_ALIGNMENT,
    instruction::Instruction,
    structs::{LineInfo, Memory},
};

use crate::definitions::processor::Processor;

use crate::definitions::structs::ExecutionStatus;
use crate::fetch::fetch;
use crate::simulator_helpers::generate_err;

use std::io::{self, Write};

pub fn single_step(
    lineinfo: &Vec<LineInfo>,
    cpu: &mut Processor,
    memory: &mut Memory,
    bps: &Vec<Breakpoint>,
) -> Result<ExecutionStatus, String> {
    // passing a breakpoints vector into this function is a very messy way of doing this, i'm aware,,,
    // ideally, a break instruction is physically injected into the code and everything works politely from there without extra shenaniganery.
    // however, for now, this will have to do
    if cpu.pc > memory.text_end || cpu.pc < memory.text_start {
        return Err(generate_err(
            lineinfo,
            cpu.pc - 4,
            "Program fell off bottom.",
        ));
    }

    // println!("{}", cpu.pc);

    // Fetch
    let fetched_instruction = fetch(&cpu.pc, &memory)?;

    cpu.pc += MIPS_ADDRESS_ALIGNMENT;

    // Decode
    let instruction =
        Instruction::from_raw_instruction(fetched_instruction).ok_or(generate_err(
            lineinfo,
            cpu.pc - 4,
            format!("Failed instruction fetch").as_str(),
        ))?;

    // Execute
    let instruction_result = cpu.process_instruction(memory, instruction);

    // The $0 register should never have been permanently changed. Don't let it remain changed.
    cpu.general_purpose_registers[0] = 0;

    // check if there's a breakpoint after instruction on the line is executed
    // TODO change this to before execution
    for bp in bps {
        if cpu.pc == bp.address {
            // println!("Breakpoint at line {} reached. (This ran in single_step())", bp.line_num);
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
                cpu.pc - 4,
                format!("{}", e).as_str(),
            ))
        }
    }
}

// equivalent to running a single line of the code.
// this function was written to make the debugger itself look a little less ugly
// although at this point it may be overdoing it
fn run_wrapper(
    lineinfo: &Vec<LineInfo>,
    cpu: &mut Processor,
    memory: &mut Memory,
    bps: &Vec<Breakpoint>,
) -> Result<ExecutionStatus, String> {
    match single_step(lineinfo, cpu, memory, &bps) {
        Ok(execution_status) => match execution_status {
            ExecutionStatus::Continue => Ok(execution_status),
            ExecutionStatus::Break => {
                match lineinfo.iter().find(|&line| line.start_address == cpu.pc) {
                    Some(line) => {
                        println!("Breakpoint at line {} reached.", line.line_number);
                    }
                    None => {
                        eprintln!("Illegal state during single-step (lineinfo could not be located for current PC 0x{:x}", cpu.pc);
                    }
                }
                Ok(execution_status)
            }
            ExecutionStatus::Complete => return Ok(ExecutionStatus::Complete),
        },
        Err(e) => return Err(e),
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

// This is the name debugger. Have fun...
pub fn debugger(
    lineinfo: &Vec<LineInfo>,
    memory: &mut Memory,
    cpu: &mut Processor,
) -> Result<(), String> {
    let mut breakpoints: Vec<Breakpoint> = Vec::new();
    let mut global_bp_num: u16 = 0;
    let mut global_list_loc: usize = 5; // for the l command

    println!("Welcome to the NAME debugger.");
    println!("For a list of commands, type \"help\".");

    // i have not written real rust before. please forgive me
    loop {
        print!("(name-db) ");
        io::stdout().flush().expect("Failed to flush stdout"); // i took cs 3377 and i still don't know why this is a thing

        let mut user_input = String::new();
        match io::stdin().read_line(&mut user_input) {
            Ok(_) => {}
            Err(e) => eprintln!("stdin error: {e}"),
        };
        let db_args: Vec<&str> = user_input.trim().split(" ").collect();

        // pub type DebugFn: fn(&Vec<LineInfo>, &mut Memory, &mut Processor, &Vec<Breakpoint>) -> Result<(), String>;

        // this could be better mayb
        match db_args[0] {
            "help" => {
                help_menu(db_args.iter().map(|&s| s.to_string()).collect());
            } // life would be too easy if there was only one type of string
            "q" => return Ok(()),
            "exit" => return Ok(()),
            "quit" => return Ok(()),
            "r" => {
                // this is supposed to always start program execution from the beginning.
                // idk how to make it do that rn
                // maybe these can secretly be the same thing and we just keep it here to make it look liek gdb :shrug:
                loop {
                    match run_wrapper(lineinfo, cpu, memory, &breakpoints) {
                        Ok(ex_stat) => match ex_stat {
                            ExecutionStatus::Complete => return Ok(()),
                            _ => {}
                        },
                        Err(e) => return Err(e),
                    };
                }
            }
            "c" => loop {
                match run_wrapper(lineinfo, cpu, memory, &breakpoints) {
                    Ok(ex_stat) => match ex_stat {
                        ExecutionStatus::Complete => return Ok(()),
                        _ => {}
                    },
                    Err(e) => return Err(e),
                };
            },
            "s" => {
                match run_wrapper(lineinfo, cpu, memory, &breakpoints) {
                    Ok(ex_stat) => match ex_stat {
                        ExecutionStatus::Complete => return Ok(()),
                        _ => {}
                    },
                    Err(e) => return Err(e),
                };
            }
            "l" => {
                if db_args.len() == 1 {
                    let num_lines = lineinfo.len();

                    let begin = global_list_loc.saturating_sub(5);
                    let end = std::cmp::min(global_list_loc.saturating_add(3), num_lines - 1);
                    for i in begin..=end {
                        println!(
                            "{:>3} #{:08x}  {}",
                            lineinfo[i].line_number, lineinfo[i].start_address, lineinfo[i].content
                        );
                    }

                    // wrap the default line number around if it exceeds the number of lines of the program
                    global_list_loc = if global_list_loc + 8 <= num_lines {
                        global_list_loc + 8
                    } else {
                        5
                    };
                } else if db_args.len() == 2 {
                    if db_args[1] == "all" {
                        for line in lineinfo {
                            println!(
                                "{:>3} #{:08x}  {}",
                                line.line_number, line.start_address, line.content
                            );
                        }
                    } else {
                        match db_args[1].parse::<usize>() {
                            Err(_) => {
                                eprintln!("l expects an unsigned int or \"all\" as an argument");
                            }
                            Ok(lnum) => {
                                if lnum > lineinfo.len() {
                                    eprintln!("{} out of bounds of program.", lnum);
                                } else {
                                    let begin = lnum.saturating_sub(5);
                                    let end = std::cmp::min(lnum.saturating_add(3), lineinfo.len());
                                    for i in begin..end {
                                        println!(
                                            "{:>3} #{:08x}  {}",
                                            lineinfo[i].line_number,
                                            lineinfo[i].start_address,
                                            lineinfo[i].content
                                        );
                                    }
                                }
                            }
                        };
                    }
                } else {
                    eprintln!("l expects 0 or 1 arguments, received {}", db_args.len() - 1);
                }
            }
            "p" => {
                if db_args.len() != 2 {
                    eprintln!("p expects 1 argument, received {}", db_args.len() - 1);
                    continue;
                }
                // #[allow(unused_assignments)]  // oh boy
                match REGISTERS.iter().position(|&x| x == db_args[1]) {
                    Some(register) => {
                        println!(
                            "Value in register {} is {:08x}",
                            register, cpu.general_purpose_registers[register]
                        );
                    }
                    None => {
                        println!("{} is not a valid register.", db_args[1]);
                        continue;
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
            "pb" => {
                println!("BP_NUM: LINE_NUM");
                for breakpoint in &breakpoints {
                    println!("{:>6}: {}", breakpoint.bp_num, breakpoint.line_num);
                    // format this...?
                }
            }
            "b" => {
                // i know there's probably better error handling but i don't have time
                // to read the whole rust book rn soz
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

                global_bp_num += 1;
                breakpoints.push(Breakpoint::new(global_bp_num, line_num, lineinfo));
                println!(
                    "Successfully added breakpoint {} at line {}.",
                    global_bp_num, line_num
                );
            }
            "del" => {
                if db_args.len() != 2 {
                    eprintln!("del expects 1 argument, received {}", db_args.len() - 1);
                    continue;
                }
                let bp_num: u16 = db_args[1]
                    .parse()
                    .expect("del takes a 16-bit unsigned int as input");

                // i KNOW this can be better
                if let Some(index) = breakpoints.iter().position(|brpt| brpt.bp_num == bp_num) {
                    let removed_element = breakpoints.remove(index);
                    println!("Removed {:?}", removed_element);
                    global_bp_num -= 1;
                } else {
                    eprintln!("Breakpoint with bp_num {} not found", bp_num);
                }
            }
            _ => println!("Option not recognized. Type \"help\" to view accepted options."),
        };
    }
}

fn help_menu(args: Vec<String>) {
    if args.len() == 1 {
        println!("help - Display this menu.");
        println!("help [CMD] - Get more information about a specific db command CMD.");
        println!("r - Begin execution of program.");
        println!("c - Continue program execution until the next breakpoint.");
        println!("s - Execute only the next instruction.");
        println!("l - Print the entire program. (this functionality will be much improved later)");
        println!("p - Print the value of a register at the current place in program execution (please include the dollar sign).");
        println!("pa - Print value of ALL registers at once.");
        println!("pb - Print all breakpoints.");
        println!("b [N] - Insert a breakpoint at line number N.");
        println!("del [N] - Delete breakpoint number N.");
        println!("q - Exit (quit) debugger.");
    } else if args.len() == 2 {
        match &args[1] as &str {
            "help" => {
                println!("you're funny");
            }
            "r" => {
                println!("Begin execution of program.");
            }
            "c" => {
                println!("Continue program execution until the next breakpoint.");
            }
            "s" => {
                println!("Execute only the next instruction.");
            }
            "l" => {
                println!("When provided no arguments: print the first ten lines of the program. Then, print the next 10, and so forth.");
                println!("When provided a line number (positive integer): print 9 lines around the given line number.");
                println!("When provided the argument \"all\": print the entire program.");
            }
            "p" => {
                println!("Print the value stored in the provided register.");
            }
            "pa" => {
                println!("Print each register and the value stored therein.");
            }
            "pb" => {
                println!("Print all user-created breakpoints.");
            }
            "b" => {
                println!("Insert a breakpoint at the line number provided. Note that this line will be executed before the break occurs.");
            }
            "del" => {
                println!("Delete the breakpoint with the associated number. (run pb to find out which number the desired breakpoint has)");
            }
            "q" => {
                println!("please work :wq please work :wq plea");
            }
            _ => {
                eprintln!("{} is either not recognized as a valid command or the help menu for it was neglected to be implemented.", args[1]);
            }
        };
    }
}
