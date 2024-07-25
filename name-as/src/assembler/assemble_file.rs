use std::path::PathBuf;

use crate::assembler::assemble_line::assemble_line;
use crate::assembler::assembler::Assembler;

/*
This function is essentially a wrapper over assemble_line.rs, allowing for some better handling in most steps

The idea is, once the assembler is done running, if any errors were encountered, their content is pushed to the errors vector,
and the errors vector is returned as the Err variant of the Result for the caller to handle. This way, all forseeable errors are printed in one pass.
There should be next to no fatal errors. I will be vetting this code later to ensure there are no execution paths which crash.

The Ok variant contains the Assembler environment, which contains the needed information for ELF object file output.
*/

pub fn assemble(file_contents: String, current_dir: PathBuf, line_prefix: Option<String>) -> Result<Assembler, Vec<String>> {
    let mut environment: Assembler = Assembler::new();

    environment.current_dir = current_dir;
    
    match line_prefix {
        Some(s) => environment.line_prefix = s,
        None => {},
    }

    for line in file_contents.split('\n') {

        // Pre-process line (expand pseudoinstructions, macros, and .eqv values here)    
        let expanded_line = environment.expand_line(line);

        assemble_line(&mut environment, line, expanded_line);
        environment.line_number += 1;
    }

    if environment.backpatches.len() > 0 {
        let undefined_symbols: Vec<String> = environment.backpatches.iter().map(|backpatch| backpatch.undiscovered_identifier.to_owned()).collect();
        let line_numbers: Vec<usize> = environment.backpatches.iter().map(|backpatch| backpatch.line_number).collect();
        
        let err_string: String = undefined_symbols.iter()
        .zip(line_numbers.iter())
        .map(|(symbol, &line_number)| format!(" - {}: line {}{}", symbol, environment.line_prefix, line_number))
        .collect::<Vec<String>>()
        .join("\n");

        environment.errors.push(format!("[*] Symbols referenced but undefined:\n{err_string}"));
    }

    if environment.errors.len() == 0 {
        return Ok(environment);
    } else {
        return Err(environment.errors);
    }
}