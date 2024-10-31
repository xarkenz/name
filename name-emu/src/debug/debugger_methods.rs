/* Autocollapse is your best friend...
 */

use name_core::{
    constants::REGISTERS,
    structs::{LineInfo, ProgramState},
};

use crate::debug::debug_utils::{Breakpoint, DebuggerState, single_step, debugger};

impl Breakpoint {
    pub fn new(bp_num: u16, line_num: u32, lineinfo: &Vec<LineInfo>) -> Self {
        Breakpoint {
            bp_num,
            line_num,
            address: {
                match lineinfo.iter().find(|&line| line.line_number == line_num) {
                    Some(line) => line.start_address,
                    None => {
                        panic!("Breakpoint not found in memory.");
                    }
                }
            },
        }
    }
    // assembler::add_label is not the solution to male loneliness
}

impl DebuggerState {
    pub fn new() -> Self {
        let breakpoints: Vec<Breakpoint> = Vec::new();
        let global_bp_num: u16 = 0;
        let global_list_loc: usize = 5;
        DebuggerState {
            breakpoints,
            global_bp_num,
            global_list_loc,
        }
    }

    /* These are all functions that only impact the debugger and not the state of the program. */

    /// "pb"
    pub fn print_all_breakpoints(&self) -> Result<(), String> {
        println!("BP_NUM: LINE_NUM");
        for breakpoint in &self.breakpoints {
            println!("{:>6}: {}", breakpoint.bp_num, breakpoint.line_num);
        }
        Ok(())
    }

    // This method is used to shorten list_text.
    pub fn list_lines(&mut self, lineinfo: &Vec<LineInfo>, mut lnum: usize) {
        if lnum == 0 {
            lnum = self.global_list_loc;
        }

        let begin = lnum.saturating_sub(5);
        let end = std::cmp::min(lnum.saturating_add(3), lineinfo.len() - 1);
        for i in begin..=end {
            println!(
                "{:>3} #{:08x}  {}",
                lineinfo[i].line_number, lineinfo[i].start_address, lineinfo[i].content
            );
        }

        // by default, bind the global list pointer (i.e. the line number that is selected when no args are provided)
        // to this current line number.
        // in a hypothetical future, we can add a flag to make this an option
        if lnum + 9 <= lineinfo.len() {
            self.global_list_loc = lnum + 9;
        } else {
            self.global_list_loc = 5;
        }
    }

    /// "b"
    pub fn add_breakpoint(&mut self, lineinfo: &Vec<LineInfo>, db_args: &Vec<String>) -> Result<(), String>{
        if db_args.len() != 2 {
            return Err(format!("b expects 1 argument, received {}", db_args.len() - 1))
        }
        
        let line_num: u32 = db_args[1]
            .parse()
            .expect("b takes 32-bit unsigned int as input");

        if line_num > lineinfo.len().try_into().unwrap() { // something like that
            return Err(format!("{} exceeds number of lines in program.", line_num))
        }

        self.global_bp_num += 1;
        self.breakpoints.push(Breakpoint::new(self.global_bp_num, line_num, lineinfo));
        println!(
            "Successfully added breakpoint {} at line {}.",
            self.global_bp_num, line_num
        );
        Ok(())
    }

    /// "del"
    pub fn remove_breakpoint(&mut self, db_args: &Vec<String>) -> Result<(), String>{
        if db_args.len() != 2 {
            return Err(format!("del expects 1 argument, received {}", db_args.len() - 1))
        }

        let bp_num: u16 = db_args[1]
            .parse()
            .expect("del takes a 16-bit unsigned int as input");

        if let Some(index) = self.breakpoints.iter().position(|brpt| brpt.bp_num == bp_num) {
            let removed_element = self.breakpoints.remove(index);
            println!("Removed {:?}", removed_element);
            self.global_bp_num -= 1;
            Ok(())
        } else {
            Err(format!("Breakpoint with bp_num {} not found", bp_num))
        }
    }
}

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

    let line_num = program_state.cpu.pc;
    println!("Breakpoint at address {} reached.", line_num); // change that into line num for now
    register_dump(program_state);
    match debugger(lineinfo, program_state) {
        Ok(_) => {},
        Err(e) => panic!("{e}"),
    };
    //TODO: ("Finish breakpoint handler implementation @Nick");
}

/// "s"
// Also called by continuously_execute
pub fn db_step(
    lineinfo: &Vec<LineInfo>, 
    program_state: &mut ProgramState, 
    debugger_state: &mut DebuggerState
) -> Result<(), String> {
    single_step(lineinfo, program_state, debugger_state);
    if program_state.is_exception() {
        todo!("Handle exception");
        // return Err("exceptionnnnnnnnn".to_string())
    }
    Ok(())
}

/// "r", "c"
pub fn continuously_execute(
    lineinfo: &Vec<LineInfo>, 
    program_state: &mut ProgramState, 
    debugger_state: &mut DebuggerState
) -> Result<(), String> {
    loop {
        match db_step(lineinfo, program_state, debugger_state) {
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
        debugger_state.list_lines(lineinfo, 0);
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
    db_args: &Vec<String>
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
        return Err(format!("Congrats! You discovered an unimplemented feature... or you forgot the dollar sign on your register."));
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
    db_args: &Vec<String>
) -> Result<(), String>{
    if db_args.len() >= 1 {
        // this outputs a lot so make sure the user actually meant to type pa and not pb or p or something
        // made it >= so we can use this function to do register_dump()
        return Err(format!("pa expects 0 arguments, received {}", db_args.len() - 1))
    }

    for register in REGISTERS {
        // change this to loop through the enum in name-core::structs instead?
        let idx: usize = REGISTERS.iter().position(|&x| x == register).unwrap();
        println!(
            "{:>5}: {:08x}",
            register, program_state.cpu.general_purpose_registers[idx]
        );
    }
    Ok(())
}

fn register_dump(program_state: &mut ProgramState) {
    match print_all_registers(program_state, &Vec::new()) {
        Ok(_) => {},
        Err(e) => eprintln!("{e}"),
    };
}

// "m"
pub fn modify_register(
    program_state: &mut ProgramState, 
    db_args: &Vec<String>
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
