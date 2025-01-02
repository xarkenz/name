use std::fs::read_to_string;
use std::path::PathBuf;

use name_as::assembler::assemble_file::assemble;

use name_core::{
    elf_def::ElfType,
    elf_utils::{create_new_elf, extract_symbol_table_to_sections, write_elf_to_file},
};

#[test]
fn palindrome_as_test() {
    let test_dir_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("[*] FATAL: No parent directory found (did you clone the entire repo?)")
        .join("tests")
        .join("samples")
        .join("palindrome");

    let input_output_paths = [
        (test_dir_path.join("main.asm"), test_dir_path.join("main.o")),
        (test_dir_path.join("check.asm"), test_dir_path.join("check.o")),
        (test_dir_path.join("util.asm"), test_dir_path.join("util.o")),
    ];

    let mut assembly_errors = Vec::new();

    for (input_path, output_path) in &input_output_paths {
        let Ok(file_contents) = read_to_string(input_path) else {
            panic!("Failed to read {input_path:?} (likely does not exist).");
        };

        let assembled_output = assemble(file_contents, test_dir_path.clone(), None);

        match assembled_output {
            Ok(assembler_environment) => {
                let (section_dot_symtab, section_dot_strtab) =
                    extract_symbol_table_to_sections(assembler_environment.symbol_table);

                let et_rel = create_new_elf(
                    vec![
                        assembler_environment.section_dot_data,
                        assembler_environment.section_dot_text,
                        assembler_environment.section_dot_rel,
                        section_dot_symtab,
                        section_dot_strtab,
                        assembler_environment.section_dot_line,
                    ],
                    ElfType::Relocatable,
                    true,
                );

                if let Err(err) = write_elf_to_file(output_path, &et_rel) {
                    panic!("{err}");
                } else {
                    println!("Object file successfully written to {output_path:?}.");
                }
            }
            Err(mut errors) => {
                assembly_errors.push(format!("In {input_path:?}:"));
                assembly_errors.append(&mut errors);
            }
        }
    }

    if assembly_errors.is_empty() {
        println!("Assembly was successful.");
    } else {
        eprintln!();
        eprintln!("The following errors were encountered during assembly (others slipped by): \n");
        for error in assembly_errors {
            eprintln!("{error}");
        }
        eprintln!();

        println!("Assembly was unsuccessful.");
    }
}
