use name_core::{
    // constants::REGISTERS,
    // elf_def::MIPS_ADDRESS_ALIGNMENT,
    // instruction::{information::InstructionInformation, instruction_set::INSTRUCTION_SET},
    structs::{/*ExecutionStatus,*/ LineInfo, Memory, Processor},
};

use crate::debug::debug_utils::DebuggerState;

// pub type DebugFn = fn(&Vec<LineInfo>, &mut Memory, &mut Processor, &Vec<Breakpoint>, &Vec<str>, &mut usize) -> Result<(), String>;

pub fn list_text(
    lineinfo: &Vec<LineInfo>,
    _memory: &mut Memory,
    _cpu: &mut Processor,
    db_state: &mut DebuggerState,
    db_args: &Vec<String>,
) -> Result<(), String> {
    if db_args.len() == 1 {
        let num_lines = lineinfo.len();

        let begin = db_state.global_list_loc.saturating_sub(5);
        let end = std::cmp::min(db_state.global_list_loc.saturating_add(3), num_lines - 1);
        for i in begin..=end {
            println!(
                "{:>3} #{:08x}  {}",
                lineinfo[i].line_number, lineinfo[i].start_address, lineinfo[i].content
            );
        }

        // wrap the default line number around if it exceeds the number of lines of the program
        db_state.global_list_loc = if db_state.global_list_loc + 9 <= num_lines {
            db_state.global_list_loc + 9
        } else {
            5
        };
        Ok(())
    } else if db_args.len() == 2 {
        if db_args[1] == "all" {
            for line in lineinfo {
                println!(
                   "{:>3} #{:08x}  {}",
                    line.line_number, line.start_address, line.content
                );
            }
            Ok(())
        } else {
            match db_args[1].parse::<usize>() {
                Err(_) => {
                    return Err(format!("l expects an unsigned int or \"all\" as an argument"));
                }
                Ok(lnum) => {
                    if lnum > lineinfo.len() {
                        eprintln!("{} out of bounds of program.", lnum);
                    } else {
                        let begin = lnum.saturating_sub(5);
                        let end =
                            std::cmp::min(lnum.saturating_add(3), lineinfo.len() - 1);
                        for i in begin..=end {
                            println!(
                                "{:>3} #{:08x}  {}",
                                lineinfo[i].line_number,
                                lineinfo[i].start_address,
                                lineinfo[i].content
                            );
                        }

                        // by default, bind the global list pointer (i.e. the line number that is selected when no args are provided)
                        // to this current line number.
                        // in a hypothetical future, we can add a flag to make this an option
                        if lnum + 9 <= lineinfo.len() {
                            db_state.global_list_loc = lnum + 9;
                        } else {
                            db_state.global_list_loc = 5;
                        }
                    }
                }
            };
            Ok(())
        }
    } else {
        Err(format!("l expects 0 or 1 arguments, received {}", db_args.len() - 1))
    }
}


pub fn help_menu(args: Vec<String>) {
    if args.len() == 1 {
        println!("help - Display this menu.");
        println!("help [CMD] - Get more information about a specific db command CMD.");
        println!("r - Begin execution of program.");
        println!("c - Continue program execution until the next breakpoint.");
        println!("s - Execute only the next instruction.");
        println!("l - Print the entire program. (this functionality will be much improved later)");
        println!("p - Print the value of a register (or registers) at the current place in program execution (please include the dollar sign).");
        println!("pa - Print value of ALL registers at once.");
        println!("m - Modify the value currently in the supplied register.");
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
            "m" => {
                println!("Change the value currently stored in a register.");
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