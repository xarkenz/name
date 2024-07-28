use std::fs::read_to_string;
use std::path::PathBuf;

use crate::assembler::assemble_file::assemble;
use crate::helpers::extract_symbol_table_to_sections;

use name_const::elf_utils::{create_new_et_rel, write_elf_to_file};

#[test]
fn full_integration_test() {
    let base_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
    .parent().expect("[*] FATAL: No parent directory found (did you clone the entire repo?)")
    .join("test_files")
    .join("instruction_demonstration");

    let test_file_path: PathBuf = base_path.join("mips_test.asm");
    let test_output_filename: PathBuf = base_path.join("mips_test.o");

    let file_contents: String = read_to_string(test_file_path).expect("Failed to read input file (likely does not exist).");

    let assembled_output = assemble(file_contents, base_path, None);

    match assembled_output {
        Ok(assembler_environment) => {
            let (section_dot_symtab, section_dot_strtab) = extract_symbol_table_to_sections(assembler_environment.symbol_table);

            let et_rel = create_new_et_rel(assembler_environment.section_dot_text, assembler_environment.section_dot_data, section_dot_symtab, section_dot_strtab);
            match write_elf_to_file(&test_output_filename, &et_rel) {
                Ok(()) => println!("Object file successfuly written to {:?}.", test_output_filename),
                Err(e) => {
                    eprintln!("{}", e);
                    panic!();
                }
            }

            println!("Assembly was successful.");
        },
        Err(errors) => {
            eprintln!();
            eprintln!("The following errors were encountered during assembly (others slipped by): \n");
            let joined_errors = errors.join("\n");
            eprintln!("{joined_errors}");

            println!("\nAssembly was unsuccessful.");
        },
    }
}