mod args;
mod assembler;
mod assembly_utils;
mod lineinfo;
mod parser;

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

    // Preprocessor would do its work in macro expansion here

    // Allowing assemble to take ownership of the source file contents, because this is the end of its utility in this function.
    let _assembled_output = assemble(file_contents).unwrap();
}

/*
#[test]
fn test_assembly() {
    let test_file_path = "/home/teqqy/Projects/name/test_files/instruction_demonstration/mips_test.asm";
    let file_contents: String = std::fs::read_to_string(test_file_path).expect("Failed to read input file (likely does not exist).");

    let _assembled_output = assemble(file_contents).unwrap();
}
*/