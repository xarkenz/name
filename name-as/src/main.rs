mod args;
mod assembler;
mod helpers;
mod parser;
mod tokens;

#[cfg(test)]
mod tests;

use std::path::PathBuf;

use args::Cli;
use assembler::assemble_file::assemble;
use helpers::extract_symbol_table_to_sections;

use name_const::elf_utils::{create_new_et_rel, write_elf_to_file};

use clap::Parser;

fn main() {
    let args = Cli::parse();
    let file_contents: String = std::fs::read_to_string(&args.input_filename).expect("Failed to read input file (likely does not exist).");

    let mut base_path = PathBuf::from(&args.input_filename);
    base_path.pop();

    // Preprocessor would do its work here

    // Allowing assemble to take ownership of the source file contents, because this is the end of its utility in this function.
    let assembled_result = assemble(file_contents, base_path, None);
    match assembled_result {
        Ok(assembler_environment) => {
            let section_dot_data: Vec<u8> = assembler_environment.section_dot_data;
            let (section_dot_symtab, section_dot_strtab) = extract_symbol_table_to_sections(assembler_environment.symbol_table);

            let et_rel = create_new_et_rel(assembler_environment.section_dot_text, section_dot_data, section_dot_symtab, section_dot_strtab);
            match write_elf_to_file(&args.output_filename, &et_rel) {
                Ok(()) => println!("Object file successfuly written to {:?}", args.output_filename),
                Err(e) => {
                    eprintln!("{}", e);
                    panic!();
                }
            }

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