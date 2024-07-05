use name_const::structs::{Backpatch, InstructionInformation, LineComponent, Symbol, SymbolType, Section};
use name_const::helpers::generate_instruction_hashmap;
use name_const::elf_utils::{MIPS_TEXT_START_ADDR, MIPS_DATA_START_ADDR, MIPS_ADDRESS_ALIGNMENT};

use crate::assembly_utils::{assemble_instruction, pretty_print_instruction};
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
*/
pub fn assemble(file_contents: String) -> Result<(), Vec<String>> {
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

    for line in file_contents.split('\n') {
        println!("\n{line_number}: {line}");

        let line_components = parse_components(line.to_string());
        
        if Option::is_none(&line_components) {
            line_number += 1;
            continue;
        }

        let mut instruction_information: Option<&'static InstructionInformation> = None;
        let mut arguments: Vec<LineComponent> = vec!();

        for component in line_components.unwrap() {
            match component.component_type {
                name_const::structs::ComponentType::Mnemonic => {
                    let retrieved_instruction_option = instruction_table.get(&component.content);
                    
                    match retrieved_instruction_option {
                        Some(retrieved_instruction_information) => {
                            instruction_information = Some(retrieved_instruction_information);
                        },
                        None => {
                            errors.push(format!("Failed to retrieve instruction information for specified mnemonic on line {line_number}."));
                            break;
                        },
                    }

                },
                name_const::structs::ComponentType::Register => {
                    arguments.push(component);
                },
                name_const::structs::ComponentType::Immediate => {
                    arguments.push(component);
                },
                name_const::structs::ComponentType::Identifier => {
                    arguments.push(component.clone());

                    if symbol_table.iter().find(|symbol| symbol.identifier == component.content).is_none() {
                        if instruction_information.is_none() {
                            errors.push(format!("Found dangling identifier attached to no instruction on line {line_number}.\nEnsure you are using a valid instruction."));
                        }

                        backpatches.push(Backpatch {
                            instruction_info: instruction_information.unwrap(),
                            arguments: arguments.clone(),
                            undiscovered_identifier: component.content.to_owned(),
                            byte_offset: section_dot_text.len(),
                        });

                        println!("Forward reference detected (line {line_number}).");
                    }
                },
                name_const::structs::ComponentType::Label => {
                    symbol_table.push(
                        Symbol { 
                            symbol_type: SymbolType::Address,
                            identifier: component.content.to_owned(),
                            value: current_address, 
                        }
                    );
                    
                    loop {
                        let backpatch_found = backpatches.iter().find(|backpatch| backpatch.undiscovered_identifier == component.content );
                        let backpatch: &Backpatch;

                        if backpatch_found.is_none() { 
                            break; 
                        } else {
                            backpatch = backpatch_found.unwrap();
                        }
                        
                        let assembled_result = assemble_instruction(backpatch.instruction_info, &backpatch.arguments, &symbol_table, &current_address);
                        match assembled_result {
                            Ok(assembled_instruction) => {
                                match assembled_instruction {
                                    Some(word) => {
                                        let insert_offset = backpatch.byte_offset;
                                        let bytes_to_insert = word.to_be_bytes();
                                        section_dot_text.splice(insert_offset..insert_offset+4, bytes_to_insert.iter().cloned());
                                        
                                        let label = backpatch.undiscovered_identifier.clone();
                                        println!(" - Backpatch resolved for label {label}");

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
                                // errors.push(e);
                                panic!("{e}");
                            },
                        }

                        break;
                        
                    }
                    
                },
                name_const::structs::ComponentType::Directive => {
                    match component.content.as_str() {
                        ".text" => {
                            match current_section {
                                Section::Null => {
                                    current_address = text_address
                                },
                                Section::Text => {
                                    errors.push(format!("Cannot declare current_section .text when already in current_section .text on line {line_number}"));
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
                                    errors.push(format!("Cannot declare current_section .data when already in current_section .data (line {line_number})"));
                                },
                            }

                            current_section = Section::Data;
                        },
                        _ => {
                            errors.push(format!("Unrecognized directive on line {line_number}"));
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
                        errors.push(format!("On line {line_number}:"));
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
    if errors.len() == 0 {
        return Ok(());
    } else {
        return Err(errors);
    }
}