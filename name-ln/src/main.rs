mod args;
mod conformity;
mod one_module_linker;

#[cfg(test)]
mod tests;

use args::Cli;
use std::path::PathBuf;
use one_module_linker::one_module_linker;

use name_const::elf_utils::{read_bytes_to_elf, write_elf_to_file};
use name_const::elf_def::Elf;

use clap::Parser;

fn main() {
    // Take in all object files as cli arguments
    let args: Cli = Cli::parse();
    let base_path: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("[*] FATAL: No parent directory found (did you clone the entire repo?)")
        .to_path_buf();

    // TODO: implement multiple input files (ehhhhh)
    // uncomment this block when can add multiple input files
    /* let module_input_fns: Vec<PathBuf> = args.input_filenames
        .iter()
        .map(|filename| base_path.join(filename))
        .collect(); */
    let single_module_input_fn = base_path.join(args.input_filenames); // this is just one file right now. obviously it's supposed to not be that
    let single_module_output_fn = base_path.join(args.output_filename);

    // invoke correct version of linker based on number of arguments supplied
    // uncomment this block when can add multiple input files
    /* let single_file_contents: Vec<u8> = module_input_fns
        .iter()
        .map(|filename| std::fs::read(filename).expect("Unable to open object file"))
        .collect(); */
    let single_file_contents: Vec<u8> = std::fs::read(single_module_input_fn).expect("Unable to open object file");

    // let single_et_rel: Elf = read_bytes_to_elf(single_file_contents).expect("This shouldn't fail rn");
    let constructed_elf: Elf = match read_bytes_to_elf(single_file_contents) {
        Ok(relocatable) => relocatable,
        Err(e) => panic!("{e}"),
    };
    
    // let linked_single_module = one_module_linker(single_et_rel);
    let executable_contents: Elf = match one_module_linker(constructed_elf) {
        Ok(result) => result,
        Err(e) => {
            panic!("{e}");
        },
    };

    // output final ET_EXEC ELF (this comment might not be correct anymore. idk)
    // let _ = write_elf_to_file(&args.output_filename, &linked_single_module.unwrap());
    match write_elf_to_file(&single_module_output_fn, &executable_contents) {
        Ok(_) => println!("Single module linking performed successfully."),
        Err(e) => panic!("{e}"),
    }
    println!("Imagine, if you will, an ET_EXEC has been emitted.");
}