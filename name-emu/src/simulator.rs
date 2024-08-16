use crate::definitions::structs::ExecutionStatus;
use crate::simulator_helpers::extract_loadable_sections;

use crate::debug_utils::single_step;

use name_const::elf_def::Elf;
use name_const::elf_utils::extract_lineinfo;
use name_const::structs::{LineInfo, Memory, Processor};

use std::io::{self, Write};

pub fn simulate(elf: Elf, debug: bool) -> Result<(), String> {
    // Set up simulation environment
    let mut cpu: Processor = Processor::new(elf.file_header.e_entry);

    let (data, text) = extract_loadable_sections(&elf);

    let lineinfo: Vec<LineInfo> = extract_lineinfo(&elf);

    let mut memory: Memory = Memory::new(data, text);

    if debug {
        println!("For a list of commands, type \"help\".");

        // i have not written real rust before. please forgive me
        loop {
            // obviously refactor this later into a name-emu/debug folder. 
            print!("(name-db) ");
            io::stdout().flush().expect("Failed to flush stdout");

            let mut buffer = String::new();
            let stdin = io::stdin();
            match stdin.read_line(&mut buffer){
                Ok(_) => {}, // do nothing
                Err(e) => println!("Error: {e}"),
            };

            let buffer = buffer.trim(); 

            if buffer == "help" {
                println!("r - run the program normally.");
                println!("help - display this menu.");
                println!("b [N] - insert a breakpoint at line number N.");
                println!("c - continue program execution until the next breakpoint.");
                println!("del [N] - delete breakpoint number N.");
                println!("l - list breakpoints (output form: bp num: line num");
                println!("q - exit debugger.")
            } else if buffer == "q" {
                return Ok(());
            } else if buffer == "r" {
                loop {
                    match single_step(&lineinfo, &mut cpu, &mut memory){
                        Ok(execution_status) => match execution_status {
                            ExecutionStatus::Continue => {},
                            ExecutionStatus::Complete => return Ok(()),
                        },
                        Err(e) => return Err(e),
                    };
                }
            } else {
                println!("Option not recognized. Run the help command to view accepted options.");
            }
        }
    } else {
        // Begin fetch/decode/execute cycle
        loop {
            match single_step(&lineinfo, &mut cpu, &mut memory){
                Ok(execution_status) => match execution_status {
                    ExecutionStatus::Continue => {},
                    ExecutionStatus::Complete => return Ok(()),
                },
                Err(e) => return Err(e),
            };
        }
    }
}