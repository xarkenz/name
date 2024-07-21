mod args;
mod conformity;
mod one_module_linker;

#[cfg(test)]
mod tests;

use args::Cli;
use one_module_linker::one_module_linker;

use name_const::elf_utils::{read_bytes_to_elf, write_elf_to_file};
use name_const::elf_def::Elf;

use clap::Parser;

fn main() {
    // Take in all object files as cli arguments
    let args: Cli = Cli::parse();
    
    // invoke correct version of linker based on number of arguments supplied
    let single_file_contents: Vec<u8> = "hi mom".as_bytes().to_owned();
    let single_et_rel: Elf = read_bytes_to_elf(single_file_contents).expect("This shouldn't fail rn");
    
    let linked_single_module = one_module_linker(single_et_rel);

    // output final ET_EXEC ELF
    let _ = write_elf_to_file(&args.output_filename, &linked_single_module.unwrap());
    println!("Imagine, if you will, an ET_EXEC has been emitted.");
}