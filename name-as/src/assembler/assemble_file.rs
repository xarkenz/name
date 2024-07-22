use name_const::structs::{InstructionInformation, LineComponent, Section, Symbol};
use name_const::elf_def::MIPS_ADDRESS_ALIGNMENT;

use crate::assembler::assembler::Assembler;

use crate::assembler::assemble_instruction::assemble_instruction;
use crate::assembler::assembly_helpers::pretty_print_instruction;

use crate::parser::parse_components;

const BACKPATCH_PLACEHOLDER: u32 = 0;

/*
I can understand that this assemble function may at first seem to be kind of a behemoth. 
Perhaps I need to extract some functionality into helpers and be more descriptive.

The logic is as follows:
- Define the variables needed for the assembly environment
- Move through file contents line by line
- Break each line into its components and specify by type what's going on
- If an instruction was present, retrieve its information from the table
- If registers/immediates/identifiers are provided, push them to an arguments vector
- If symbols are encountered, attempt to resolve them
- After all this is said and done, call the assemble_instruction helper with the arguments and symbol table if an instruction was present
- Update tracking variables (line_number, current_address, etc.) appropriately

The idea is, once the assembler is done running, if any errors were encountered, their content is pushed to the errors vector,
and the errors vector is returned as the Err variant of the Result for the caller to handle.

The Ok variant consists of any data needed for the ELF object file output.
*/

pub fn assemble(file_contents: String) -> Result<(Vec<u8>, Vec<Symbol>), Vec<String>> {
    let mut environment: Assembler = Assembler::new();

    for line in file_contents.split('\n') {
        println!("{}: {}", environment.line_number, line);

        let line_components_result = parse_components(line.to_string(), &environment.mnemonics);

        let line_components;
        match line_components_result {
            Ok(components) => line_components = components,
            Err(e) => {
                environment.errors.push(format!("[*] On line {}:", environment.line_number));
                environment.errors.push(e);
                environment.errors.push("".to_string());
                environment.line_number += 1;
                continue;
            },
        }
        
        if Option::is_none(&line_components) {
            environment.line_number += 1;
            continue;
        }

        let mut instruction_information: Option<&'static InstructionInformation> = None;
        let mut arguments: Vec<LineComponent> = vec!();

        for component in line_components.unwrap() {
            match component {
                LineComponent::Mnemonic(mnemonic) => {
                    let retrieved_instruction_option = environment.instruction_table.get(&mnemonic);
                    
                    match retrieved_instruction_option {
                        Some(retrieved_instruction_information) => {
                            instruction_information = Some(retrieved_instruction_information);
                        },
                        None => {
                            environment.errors.push(format!(" - Failed to retrieve instruction information for specified mnemonic on line {}.", environment.line_number));
                            break;
                        },
                    }

                },
                LineComponent::Register(_) => {
                    arguments.push(component);
                },
                LineComponent::Immediate(_) => {
                    arguments.push(component);
                },
                LineComponent::Identifier(content) => {
                    arguments.push(
                        LineComponent::Identifier(content.clone())
                    );

                    if !environment.symbol_exists(&content) {
                        match instruction_information {
                            Some(instruction_info) => {
                                environment.add_backpatch(instruction_info, &arguments, content);
                                println!(" - Forward reference detected (line {}).", environment.line_number);
                            },
                            None => environment.errors.push(format!(" - Found dangling identifier attached to no instruction on line {}.\nEnsure you are using a valid instruction.", environment.line_number)),
                        }
                    }
                },
                LineComponent::Label(content) => {
                    environment.add_label(&content);
                    environment.resolve_backpatches(&content);
                },
                LineComponent::Directive(content) => {
                    environment.handle_directive(&content);
                },
            }
        }

        // To save you time on reading closing braces, at this point of execution all the components of an individual line have been processed.
        // We are still in scope of the outer for loop, iterating line by line
        match instruction_information {
            None => {
                environment.line_number += 1;
                continue;
            },
            Some(info) => {
                let assembled_instruction_result = assemble_instruction(info, &arguments, &environment.symbol_table, &environment.current_address);

                match assembled_instruction_result {
                    Ok(assembled_instruction) => {
                        match assembled_instruction {
                            Some(packed) => {
                                environment.section_dot_text.extend_from_slice(
                                    &packed.to_be_bytes()
                                );

                                pretty_print_instruction(&packed);
                            },
                            None => {
                                environment.section_dot_text.extend_from_slice(
                                    &BACKPATCH_PLACEHOLDER.to_be_bytes()
                                );

                                println!(" - Placeholder bytes appended to section .text.\n");
                            }
                        }
                    },
                    Err(e) => {
                        environment.errors.push(format!("[*] On line {}:", environment.line_number));
                        environment.errors.push(e);
                        environment.errors.push("".to_string());
                    }
                }
            }
        }

        if let Section::Text = environment.current_section {
            environment.current_address += MIPS_ADDRESS_ALIGNMENT;
        }

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


    if environment.errors.len() == 0 {
        return Ok((environment.section_dot_text, environment.symbol_table));
    } else {
        return Err(environment.errors);
    }
}