mod args;
mod debug;
// mod debug_utils;
mod decode;
mod definitions;
mod fetch;
mod simulator_helpers;
mod simulator;

#[cfg(test)]
mod tests;

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

    let _ = simulator::simulate(executable, args.debug);
    
    // println!("ploink");
}