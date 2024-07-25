use std::path::PathBuf;

// use name_const::structs::{InstructionInformation, LineComponent, Section};
// use name_const::elf_def::MIPS_ADDRESS_ALIGNMENT;

use crate::assembler::assemble_line::assemble_line;
use crate::assembler::assembler::Assembler;

// use crate::assembler::assemble_instruction::assemble_instruction;
// use crate::assembler::assembly_helpers::pretty_print_instruction;

// use crate::parser::parse_components;

// const BACKPATCH_PLACEHOLDER: u32 = 0;

/*
I can understand that this assemble function may at first seem to be kind of a behemoth. 
Perhaps I need to extract some functionality into helpers and be more descriptive (that is indeed what needed to happen. what a prophet).

The logic is as follows:
- Initialize the assembly environment (symbol table, sections, etc.)
- Move through file contents line by line
- Break each line into its components and specify by type what needs to happen for each component
- If an instruction was present, retrieve its information from the constant table
- If registers/immediates/identifiers are provided, push them to an arguments vector
- If symbols are encountered, attempt to resolve them. If unresolvable, save them to the environment's backpatches for fixing later.
- After all this is said and done, call the assemble_instruction helper with the arguments and symbol table if an instruction was present
- Update tracking variables (line_number, current_address, etc.) appropriately

The idea is, once the assembler is done running, if any errors were encountered, their content is pushed to the errors vector,
and the errors vector is returned as the Err variant of the Result for the caller to handle. This way, all forseeable errors are printed in one pass.
There should be next to no fatal errors. I will be vetting this code later to ensure there are no execution paths which crash.

The Ok variant contains the Assembler environment, which contains the needed information for ELF object file output.
*/

pub fn assemble(file_contents: String, current_dir: PathBuf) -> Result<Assembler, Vec<String>> {
    let mut environment: Assembler = Assembler::new();

    environment.current_dir = current_dir;


    for line in file_contents.split('\n') {    
        let mut expanded_line = line.to_string();
        for expandable in &environment.expandables {
            expanded_line = expandable.expand(&expanded_line);
        }

        assemble_line(&mut environment, line, &mut expanded_line);
        environment.line_number += 1;
    }

    // This return logic is out of scope of both the above for loops
    if environment.backpatches.len() > 0 {
        let undefined_symbols: Vec<String> = environment.backpatches.iter().map(|backpatch| backpatch.undiscovered_identifier.to_owned()).collect();
        let line_numbers: Vec<usize> = environment.backpatches.iter().map(|backpatch| backpatch.line_number).collect();
        
        let err_string: String = undefined_symbols.iter()
        .zip(line_numbers.iter())
        .map(|(symbol, &line_number)| format!(" - {symbol}: line {line_number}"))
        .collect::<Vec<String>>()
        .join("\n");

        environment.errors.push(format!("[*] Symbols referenced but undefined:\n{err_string}"));
    }

    dbg!(&environment.symbol_table);

    if environment.errors.len() == 0 {
        return Ok(environment);
    } else {
        return Err(environment.errors);
    }
}