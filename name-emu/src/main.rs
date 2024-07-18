mod args;
mod decode;
mod execute;
mod simulator;

use crate::args::Cli;

use name_const::elf_def::Elf;
use name_const::elf_utils::read_bytes_to_elf;

use clap::Parser;

use std::fs::read;

pub fn main() {
    let args = Cli::parse();

    let elf_contents: Vec<u8> = read(args.input_filename).expect("File not found");

    let executable: Elf = match read_bytes_to_elf(elf_contents) {
        Ok(elf) => elf,
        Err(e) => panic!("{}", e),
    };

    let _ = simulator::simulate(executable);
    
    println!("Hello, world!");
}

#[test]
fn emulator_quick_test() {
    let input_fn: std::path::PathBuf = From::from("/home/teqqy/Projects/name/test_files/instruction_demonstration/mips_test");
    
    let elf_contents: Vec<u8> = read(input_fn).expect("[*] FATAL: NAME cannot files that don't exist...");
    
    let executable: Elf = match read_bytes_to_elf(elf_contents){
        Ok(elf) => elf,
        Err(e) => panic!("{}", e),
    };

    // TODO: compliance check executable (format, existence, etc.)

    let _ = simulator::simulate(executable);

    assert_eq!(1+1, 2);
}