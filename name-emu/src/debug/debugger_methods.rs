use name_core::{
    constants::REGISTERS,
    // structs::Register,
    structs::{LineInfo, OperatingSystem, ProgramState},
};

use crate::debug::debug_utils::{single_step, /*cli_debugger, Breakpoint, */ DebuggerState};

use crate::exception_handler::handle_exception;


pub fn handle_breakpoint(program_state: &mut ProgramState, lineinfo: &Vec<LineInfo>) -> () {
    /* Needs to do the following:
     * Transfer control to the user
     *      Register dump (pretty pa)
     *      Type in a letter to get a hex dump of .data
     * Note that cp0 should have flags for whether user ran c or s
     * Idea: simply replace the instruction on bp.line_num with break
     *      when done, rereplace the instruction and decrement pc by 4 :jadCensored:
     * Use the code in the break instruction to match injectively (:nerd:) to the instruction you replaced
     */

    let line_addr = program_state.cpu.pc;
    let line_num = match lineinfo.iter().find(|line| line.start_address == line_addr) {
        Some(found_line) => found_line.line_number,
        None => {
            panic!(
                    "Line number with associated breakpoint address 0x{:x} not found. Something has gone seriously wrong.", 
                    line_addr
                );
        }
    };
    println!("Breakpoint at line {} reached.", line_num); // use address for now...
    register_dump(program_state);
    //TODO: ("Finish breakpoint handler implementation @Nick");
}


//
// Everything below this point is function spam for the cli debugger.
// Autocollapse is your best friend...
//


/// "s"
// Also called by continuously_execute
pub fn db_step(
    lineinfo: &Vec<LineInfo>,
    program_state: &mut ProgramState,
    os: &mut OperatingSystem,
    debugger_state: &mut DebuggerState,
) -> Result<(), String> {
    single_step(lineinfo, program_state, debugger_state);
    if program_state.is_exception() {
        // todo!("Handle exception");
        // return Err("exceptionnnnnnnnn".to_string())
        handle_exception(program_state, os, lineinfo);
    }
    Ok(())
}

/// "r", "c"
pub fn continuously_execute(
    lineinfo: &Vec<LineInfo>,
    program_state: &mut ProgramState,
    os: &mut OperatingSystem,
    debugger_state: &mut DebuggerState,
) -> Result<(), String> {
    loop {
        match db_step(lineinfo, program_state, os, debugger_state) {
            Ok(_) => continue,
            Err(e) => return Err(e),
        }
    }
}

/// "l"
pub fn list_text(
    lineinfo: &Vec<LineInfo>,
    debugger_state: &mut DebuggerState,
    db_args: &Vec<String>,
) -> Result<(), String> {
    if db_args.len() == 1 {
        debugger_state.list_lines(lineinfo, debugger_state.global_list_loc);
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
                    return Err(format!(
                        "l expects an unsigned int or \"all\" as an argument"
                    ));
                }
                Ok(lnum) => {
                    if lnum > lineinfo.len() {
                        return Err(format!("{} out of bounds of program.", lnum));
                    } else {
                        debugger_state.list_lines(lineinfo, lnum);
                    }
                }
            };
            Ok(())
        }
    } else {
        Err(format!(
            "l expects 0 or 1 arguments, received {}",
            db_args.len() - 1
        ))
    }
}

// "p"
pub fn print_register(
    program_state: &mut ProgramState,
    db_args: &Vec<String>,
) -> Result<(), String> {
    if db_args.len() < 2 {
        return Err(format!(
            "p expects a non-zero argument, received {}",
            db_args.len() - 1
        ));
    }

    // if the first character of the argument isn't a dollar sign,
    // assume the user isn't referring to a register
    // (eventually expand the functionality of this method to also be able to print the value of memory addresses
    // and stuff like that)
    if db_args[1].chars().nth(0) != Some('$') {
        return Err(format!(
            "Congrats! You discovered an unimplemented feature... or you forgot the dollar sign on your register."
        ));
    }

    for register in db_args[1..].to_vec() {
        match REGISTERS.iter().position(|&x| x == register) {
            Some(found_register) => {
                // should we continue printing the actual number of the register?
                // this will all eventually be a table or something anyways :^)
                println!(
                    "Value in register {} is {:08x}",
                    found_register, program_state.cpu.general_purpose_registers[found_register]
                );
            }
            None => {
                return Err(format!("{} is not a valid register.", db_args[1]));
            }
        }
    }
    Ok(())
}

// "pa"
pub fn print_all_registers(
    program_state: &mut ProgramState,
    db_args: &Vec<String>,
) -> Result<(), String> {
    if db_args.len() > 1 {
        // this outputs a lot so make sure the user actually meant to type pa and not pb or p or something
        // made it > so we can use this function to do register_dump()
        return Err(format!(
            "pa expects 0 arguments, received {}",
            db_args.len() - 1
        ));
    }

    // for register in Register.values() {
    for register in REGISTERS {
        // change this to loop through the enum in name-core::structs instead?
        let idx: usize = REGISTERS.iter().position(|&x| x == register).unwrap();
        println!(
            "{:>5}: {:08x}",
            register,
            program_state.cpu.general_purpose_registers[idx] // register, program_state.cpu.general_purpose_registers[register]
        );
    }
    Ok(())
}

fn register_dump(program_state: &mut ProgramState) {
    match print_all_registers(program_state, &Vec::new()) {
        Ok(_) => {}
        Err(e) => eprintln!("{e}"),
    };
}

// "m"
pub fn modify_register(
    program_state: &mut ProgramState,
    db_args: &Vec<String>,
) -> Result<(), String> {
    if db_args.len() != 3 {
        return Err(format!(
            "m expects 2 arguments, received {}",
            db_args.len() - 1
        ));
    }

    // grab the register we want to modify
    let register = match REGISTERS.iter().position(|&x| x == db_args[1]) {
        Some(found_register) => found_register,
        None => {
            return Err(format!(
                "First argument to m must be a register. (Did you include the dollar sign?)"
            ));
        }
    };

    // grab the value we want to change the register to
    let parsed_u32 = match db_args[2].parse::<u32>() {
        Ok(found) => found,
        Err(e) => {
            return Err(format!("{e}"));
        }
    };

    let original_val = program_state.cpu.general_purpose_registers[register];
    program_state.cpu.general_purpose_registers[register] = parsed_u32;
    println!(
        "Successfully modified value in register {} from {} to {}.",
        db_args[1], original_val, parsed_u32
    );
    Ok(())
}

pub fn help_menu(db_args: Vec<String>) -> Result<(), String> {
    if db_args.len() == 1 {
        println!("help - Display this menu.");
        println!("help [CMD] - Get more information about a specific db command CMD.");
        println!("r - Begin execution of program.");
        println!("c - Continue program execution until the next breakpoint.");
        println!("s - Execute only the next instruction.");
        println!("l - Print the entire program. (this functionality will be much improved later)");
        println!("p - Print the value of a register (or registers) at the current place in program execution (please include the dollar sign).");
        println!("pa - Print value of ALL registers at once.");
        println!("pb - Print all breakpoints.");
        println!("b [N] - Insert a breakpoint at line number N.");
        println!("del [N] - Delete breakpoint number N.");
        println!("q - Exit (quit) debugger.");
    } else if db_args.len() == 2 {
        match &db_args[1] as &str {
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
            "help" => {
                println!("you're funny");
            }
            "q" => {
                println!("please work :wq please work :wq plea");
            }
            _ => {
                eprintln!("{} is either not recognized as a valid command or the help menu for it was neglected to be implemented.", db_args[1]);
            }
        };
    }
    Ok(())
}
