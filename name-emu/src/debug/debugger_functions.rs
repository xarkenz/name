use name_core::{
    constants::REGISTERS,
    // elf_def::MIPS_ADDRESS_ALIGNMENT,
    // instruction::{information::InstructionInformation, instruction_set::INSTRUCTION_SET},
    structs::{/*ExecutionStatus,*/ LineInfo, Memory, Processor},
};

use crate::debug::debug_utils::{Breakpoint, DebuggerState};

// pub type DebugFn = fn(&Vec<LineInfo>, &mut Memory, &mut Processor, &mut DebuggerState, &Vec<str>) -> Result<(), String>;

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

    pub fn print_all_breakpoints(&self){
        println!("BP_NUM: LINE_NUM");
        for breakpoint in &self.breakpoints {
            println!("{:>6}: {}", breakpoint.bp_num, breakpoint.line_num);
        }
    }

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

    pub fn remove_breakpoint(&mut self, db_args: &Vec<String>) -> Result<(), String>{
        if db_args.len() != 2 {
            return Err(format!("del expects 1 argument, received {}", db_args.len() - 1))
        }

        let bp_num: u16 = db_args[1]
            .parse()
            .expect("del takes a 16-bit unsigned int as input");
        // i KNOW this can be better
        if let Some(index) = self.breakpoints.iter().position(|brpt| brpt.bp_num == bp_num) {
            let removed_element = self.breakpoints.remove(index);
            println!("Removed {:?}", removed_element);
            self.global_bp_num -= 1;
            Ok(())
        } else {
            Err(format!("Breakpoint with bp_num {} not found", bp_num))
        }
    }

    pub fn list_lines(&mut self, lineinfo: &Vec<LineInfo>, mut lnum: usize) {
        if lnum == 0 {
            lnum = self.global_list_loc;
        }
            
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
            self.global_list_loc = lnum + 9;
        } else {
            self.global_list_loc = 5;
        }
    }
}

pub fn list_text(
    lineinfo: &Vec<LineInfo>,
    _memory: &mut Memory,
    _cpu: &mut Processor,
    db_state: &mut DebuggerState,
    db_args: &Vec<String>,
) -> Result<(), String> {
    if db_args.len() == 1 {
        db_state.list_lines(lineinfo, 0);
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
                        db_state.list_lines(lineinfo, lnum);
                    }
                }
            };
            Ok(())
        }
    } else {
        Err(format!("l expects 0 or 1 arguments, received {}", db_args.len() - 1))
    }
}

pub fn print_register(cpu: &mut Processor, db_args: &Vec<String>) -> Result<(), String> {
    if db_args.len() < 2 {
        return Err(format!(
            "p expects a non-zero argument, received {}",
            db_args.len() - 1
        ));
    }

    if db_args[1].chars().nth(0) != Some('$') {
        return Err(format!("Congrats! You discovered an unimplemented feature... or you forgot the dollar sign on your register."));
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
                return Err(format!("{} is not a valid register.", db_args[1]));
            }
        }
    }
    Ok(())
}

pub fn print_all_registers(cpu: &mut Processor, db_args: &Vec<String>) -> Result<(), String>{
    if db_args.len() != 1 {
        // this outputs a lot so make sure the user actually meant to type pa and not pb or p or something
        return Err(format!("pa expects 0 arguments, received {}", db_args.len() - 1))
    }

    for register in REGISTERS {
        let idx: usize = REGISTERS.iter().position(|&x| x == register).unwrap();
        println!(
            "{:>5}: {:08x}",
            register, cpu.general_purpose_registers[idx]
        );
    }
    Ok(())
}

pub fn modify_register(cpu: &mut Processor, db_args: &Vec<String>) -> Result<(), String> {
    if db_args.len() != 3 {
        return Err(format!("m expects 2 arguments, received {}", db_args.len() - 1))
    }

    let register = match REGISTERS.iter().position(|&x| x == db_args[1]) {
        Some(found_register) => found_register,
        None => {
            return Err(format!("First argument to m must be a register. (Did you include the dollar sign?)"));
        }
    };

    let parsed_u32 = match db_args[2].parse::<u32>() {
        Ok(found) => found,
        Err(e) => {
            return Err(format!("{e}"));
        }
    };

    let original_val = cpu.general_purpose_registers[register];
    cpu.general_purpose_registers[register] = parsed_u32;
    println!(
        "Successfully modified value in register {} from {} to {}.",
        db_args[1], original_val, parsed_u32
    );
    Ok(())
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