mod args;
mod assembler;
mod assembly_utils;
mod lineinfo;
mod parser;
mod tokens;

use args::Cli;
use assembler::assemble;
use lineinfo::get_lineinfo;

use name_const::structs::LineInfo;

use clap::Parser;

fn main() {
    let args = Cli::parse();
    let file_contents: String = std::fs::read_to_string(args.input_filename).expect("Failed to read input file (likely does not exist).");

    if args.lines {
        let _section_dot_line: Vec<LineInfo> = get_lineinfo(&file_contents).expect("NAME will not assemble an empty file.");
    }

    // Preprocessor would do its work in macro/pseudoinstruction expansion here

    // Allowing assemble to take ownership of the source file contents, because this is the end of its utility in this function.
    let assembled_result = assemble(file_contents);
    match assembled_result {
        Ok(_) => {
            println!("Assembly was successful.");
        },
        Err(errors) => {
            eprintln!("Errors were encountered during assembly: \n");
            let joined_errors = errors.join("\n");
            eprintln!("{joined_errors}");
            
            // This exit with a bad exit code tells the vscode extension to not bother with linking or emulation.
            std::process::exit(1);
        }
    }
}

#[test]
fn full_integration_test() {

    let test_file_path = "/home/teqqy/Projects/name/test_files/instruction_demonstration/mips_test.asm";
    let file_contents: String = std::fs::read_to_string(test_file_path).expect("Failed to read input file (likely does not exist).");

    let assembled_output = assemble(file_contents);

    match assembled_output {
        Ok(_) => {
            println!("Assembly was successful.");
        },
        Err(errors) => {
            eprintln!();
            eprintln!("Errors were encountered during assembly: \n");
            let joined_errors = errors.join("\n");
            eprintln!("{joined_errors}");
        },
    }// assert!(assembled_output.is_ok());
}