use std::string;

use name_const::structs::{InstructionInformation, LineComponent, Section};
use name_const::elf_def::MIPS_ADDRESS_ALIGNMENT;

use crate::assembler::assembler::Assembler;

use crate::assembler::assemble_instruction::assemble_instruction;
use crate::assembler::assembly_helpers::pretty_print_instruction;

use crate::parser::parse_components;

const BACKPATCH_PLACEHOLDER: u32 = 0;

pub fn assemble_line(environment: &mut Assembler, line: &str, expanded_line: &mut str){
    println!("{}: {}", environment.line_number, line);

    // let mut expanded_line = line.to_string();
    // for expandable in &environment.expandables {
    //     expanded_line = expandable.expand(&expanded_line);
    // }

    let line_components_result = parse_components(expanded_line.to_string(), &environment.mnemonics);

    let line_components;
    match line_components_result {
        Ok(components) => line_components = components,
        Err(e) => {
            environment.errors.push(format!("[*] On line {}:", environment.line_number) + &e);
            // environment.line_number += 1;
            return;
        },
    }
    
    if Option::is_none(&line_components) {
        // environment.line_number += 1;
        return;
    }

    let mut instruction_information: Option<&'static InstructionInformation> = None;
    let mut found_directive: Option<String> = None;
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
                        environment.errors.push(format!("[*] On line {}:", environment.line_number));
                        environment.errors.push(format!(" - Failed to retrieve instruction information for specified mnemonic"));
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

                // If the symbol does not exist in the symbol table, a backpatch must be created.
                if !environment.symbol_exists(&content) {
                    match instruction_information {
                        Some(instruction_info) => {
                            environment.add_backpatch(instruction_info, &arguments, content);
                            println!(" - Forward reference detected (line {}).", environment.line_number);
                        },
                        None => {
                            // If there's no instruction information on this line, the identifier is likely for a preprocessor macro.
                            // Nothing else needs to be done at this time.
                        },
                    }
                }
            },
            LineComponent::Label(content) => {
                environment.add_label(&content);
                environment.resolve_backpatches(&content);
            },
            LineComponent::Directive(content) => {
                found_directive = Some(content.clone());
            },
            LineComponent::DoubleQuote(_) => {
                arguments.push(component);
            }
        }
    }

    // To save you time on reading closing braces, at this point of execution all the components of an individual line have been processed.
    // We are still in scope of the outer for loop, iterating line by line
    match instruction_information {
        None => {},
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
                }
            }
        }
    }

    match found_directive {
        Some(directive) => environment.handle_directive(&directive, &arguments),
        None => {},
    }

    if let Section::Text = environment.current_section {
        environment.current_address += MIPS_ADDRESS_ALIGNMENT;
    }

    // environment.line_number += 1;
}