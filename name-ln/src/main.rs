mod args;
mod one_module_linker;

use args::Cli;
use one_module_linker::one_module_linker;

use name_const::elf_utils::{read_bytes_to_et_rel, write_et_exec_to_file};
use name_const::elf_def::RelocatableElf;

use clap::Parser;

fn main() {
    // Take in all object files as cli arguments
    let args: Cli = Cli::parse();
    
    // invoke correct version of linker based on number of arguments supplied
    let single_file_contents: Vec<u8> = "hi mom".as_bytes().to_owned();
    let single_et_rel: RelocatableElf = read_bytes_to_et_rel(single_file_contents).expect("This shouldn't fail rn");
    
    let linked_single_module = one_module_linker(single_et_rel);

    // output final ET_EXEC ELF
    let _ = write_et_exec_to_file(&args.output_filename, linked_single_module.unwrap());
    println!("Imagine, if you will, an ET_EXEC has been emitted.");
}


#[test]
fn one_module_linker_test() {
    let single_module_input_fn = "/home/teqqy/Projects/name/test_files/instruction_demonstration/mips_test.o";
    let single_module_output_fn = std::path::PathBuf::from("/home/teqqy/Projects/name/test_files/instruction_demonstration/mips_test");

    let single_file_contents: Vec<u8> = std::fs::read(single_module_input_fn).expect("Unable to open object file");
    let constructed_elf: name_const::elf_def::RelocatableElf = match name_const::elf_utils::read_bytes_to_et_rel(single_file_contents) {
        Ok(relocatable) => relocatable,
        Err(e) => panic!("{e}"),
    };

    dbg!(&constructed_elf);

    let executable_contents: name_const::elf_def::ExecutableElf = match one_module_linker::one_module_linker(constructed_elf) {
        Ok(result) => result,
        Err(e) => {
            panic!("{e}");
        },
    };

    match name_const::elf_utils::write_et_exec_to_file(&single_module_output_fn, executable_contents) {
        Ok(_) => println!("Single module linking performed successfully."),
        Err(e) => panic!("{e}"),
    }
}