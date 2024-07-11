use name_const::structs::{Backpatch, InstructionInformation, LineComponent, Section, Symbol, Visibility};
use name_const::helpers::{generate_instruction_hashmap, get_mnemonics};
use name_const::elf_utils::{MIPS_TEXT_START_ADDR, MIPS_DATA_START_ADDR, MIPS_ADDRESS_ALIGNMENT, STT_FUNC, STT_OBJECT};

use crate::assemble_instruction::assemble_instruction;
use crate::assembly_helpers::pretty_print_instruction;
use crate::parser::parse_components;

use std::collections::HashMap;

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
    let mut errors: Vec<String> = vec!();

    let instruction_table: HashMap<String, &'static InstructionInformation> = generate_instruction_hashmap();

    let mut symbol_table: Vec<Symbol> = vec!();
    let mut section_dot_text: Vec<u8> = vec!();

    let mut backpatches: Vec<Backpatch> = vec!();

    let mut current_address = 0;
    let mut text_address = MIPS_TEXT_START_ADDR;
    let mut data_address = MIPS_DATA_START_ADDR;

    let mut current_section: Section = Section::Null;

    let mut line_number: usize = 1;

    let mnemonics = get_mnemonics();

    for line in file_contents.split('\n') {
        println!("{line_number}: {line}");

        let line_components_result = parse_components(line.to_string(), &mnemonics);

        let line_components;
        match line_components_result {
            Ok(components) => line_components = components,
            Err(e) => {
                errors.push(format!("[*] On line {line_number}:"));
                errors.push(e);
                errors.push("".to_string());
                line_number += 1;
                continue;
            },
        }
        
        if Option::is_none(&line_components) {
            line_number += 1;
            continue;
        }

        let mut instruction_information: Option<&'static InstructionInformation> = None;
        let mut arguments: Vec<LineComponent> = vec!();

        for component in line_components.unwrap() {
            match component {
                LineComponent::Mnemonic(mnemonic) => {
                    let retrieved_instruction_option = instruction_table.get(&mnemonic);
                    
                    match retrieved_instruction_option {
                        Some(retrieved_instruction_information) => {
                            instruction_information = Some(retrieved_instruction_information);
                        },
                        None => {
                            errors.push(format!(" - Failed to retrieve instruction information for specified mnemonic on line {line_number}."));
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

                    if symbol_table.iter().find(|symbol| symbol.identifier == content).is_none() {
                        if instruction_information.is_none() {
                            errors.push(format!(" - Found dangling identifier attached to no instruction on line {line_number}.\nEnsure you are using a valid instruction."));
                        }

                        backpatches.push(Backpatch {
                            instruction_info: instruction_information.unwrap(),
                            arguments: arguments.clone(),
                            undiscovered_identifier: content.to_owned(),
                            backpatch_address: current_address,
                            byte_offset: section_dot_text.len(),
                            line_number: line_number,
                        });

                        println!(" - Forward reference detected (line {line_number}).");
                    }
                },
                LineComponent::Label(content) => {
                    symbol_table.push(
                        Symbol {
                            symbol_type: match current_section {
                                Section::Null => {
                                    errors.push(" - Cannot declare label outside a section.".to_string());
                                    0
                                },
                                Section::Text => STT_FUNC,
                                Section::Data => STT_OBJECT,
                            },
                            identifier: content.to_owned(),
                            value: current_address,
                            size: 4,
                            visibility: Visibility::Local,
                            section: Section::Text, 
                        }
                    );
                    
                    loop {
                        let backpatch_found = backpatches.iter().find(|backpatch| backpatch.undiscovered_identifier == content );
                        let backpatch: &Backpatch;

                        if backpatch_found.is_none() { 
                            break; 
                        } else {
                            backpatch = backpatch_found.unwrap();
                        }

                        let backpatch_address: u32 = backpatch.backpatch_address;
                        
                        let assembled_result = assemble_instruction(backpatch.instruction_info, &backpatch.arguments, &symbol_table, &backpatch_address);
                        match assembled_result {
                            Ok(assembled_instruction) => {
                                match assembled_instruction {
                                    Some(word) => {
                                        let insert_offset = backpatch.byte_offset;
                                        let bytes_to_insert = word.to_be_bytes();

                                        section_dot_text.splice(insert_offset..insert_offset+4, bytes_to_insert.iter().cloned());
                                        
                                        let label = backpatch.undiscovered_identifier.clone();
                                        let line = backpatch.line_number.clone();
                                        println!(" - Backpatch resolved for label {label} on line {line}:");
                                        pretty_print_instruction(&word);

                                        let found_index = backpatches.iter().position(|bp| bp == backpatch).expect("Literally impossible to get this error.");

                                        backpatches.remove(found_index);
                                    },
                                    None => {
                                        // If there still exists an unresolved reference, something has gone terribly wrong.
                                        panic!("Unable to resolve backpatching for label declared on line {line_number}");
                                    },
                                }
                            },
                            Err(e) => {
                                errors.push(e);
                                let found_index = backpatches.iter().position(|bp| bp == backpatch).expect("Literally impossible to get this error.");

                                backpatches.remove(found_index);
                            },
                        }
                        
                    }
                    
                },
                LineComponent::Directive(content) => {
                    match content.as_str() {
                        ".text" => {
                            match current_section {
                                Section::Null => {
                                    current_address = text_address
                                },
                                Section::Text => {
                                    errors.push(format!(" - Cannot declare current_section .text when already in current_section .text on line {line_number}"));
                                },
                                Section::Data => {
                                    data_address = current_address;
                                    current_address = text_address;
                                },
                            }

                            current_section = Section::Text;
                        },
                        ".data" => {
                            match current_section {
                                Section::Null => {
                                    current_address = text_address
                                },
                                Section::Text => {
                                    text_address = current_address;
                                    current_address = data_address;
                                },
                                Section::Data => {
                                    errors.push(format!(" - Cannot declare current_section .data when already in current_section .data (line {line_number})"));
                                },
                            }

                            current_section = Section::Data;
                        },
                        _ => {
                            errors.push(format!(" - Unrecognized directive on line {line_number}"));
                        }
                    }
                },
            }
        }

        // To save you time on reading closing braces, at this point of execution all the components of an individual line have been processed.
        // We are still in scope of the outer for loop, iterating line by line
        match instruction_information {
            None => {
                line_number += 1;
                continue;
            },
            Some(info) => {
                let assembled_instruction_result = assemble_instruction(info, &arguments, &symbol_table, &current_address);

                match assembled_instruction_result {
                    Ok(assembled_instruction) => {
                        match assembled_instruction {
                            Some(packed) => {
                                section_dot_text.extend_from_slice(
                                    &packed.to_be_bytes()
                                );

                                pretty_print_instruction(&packed);
                            },
                            None => {
                                section_dot_text.extend_from_slice(
                                    &BACKPATCH_PLACEHOLDER.to_be_bytes()
                                );

                                println!(" - Placeholder bytes appended to section .text.\n");
                            }
                        }
                    },
                    Err(e) => {
                        errors.push(format!("[*] On line {line_number}:"));
                        errors.push(e);
                        errors.push("".to_string());
                    }
                }
            }
        }

        if let Section::Text = current_section {
            current_address += MIPS_ADDRESS_ALIGNMENT;
        }

        line_number += 1;
    }

    // This return logic is out of scope of both the above for loops
    if backpatches.len() > 0 {
        let undefined_symbols: Vec<String> = backpatches.iter().map(|backpatch| backpatch.undiscovered_identifier.to_owned()).collect();
        let line_numbers: Vec<usize> = backpatches.iter().map(|backpatch| backpatch.line_number).collect();
        
        let err_string: String = undefined_symbols.iter()
        .zip(line_numbers.iter())
        .map(|(symbol, &line_number)| format!(" - {symbol}: line {line_number}"))
        .collect::<Vec<String>>()
        .join("\n");

        errors.push(format!("[*] Symbols referenced but undefined:\n{err_string}"));
    }


    if errors.len() == 0 {
        return Ok((section_dot_text, symbol_table));
    } else {
        return Err(errors);
    }
}