use name_const::structs::{InstructionInformation, LineComponent, Symbol, SymbolType, Section};
use name_const::helpers::generate_instruction_hashmap;
use name_const::elf_utils::{MIPS_TEXT_START_ADDR, MIPS_DATA_START_ADDR, MIPS_ADDRESS_ALIGNMENT};

use crate::assembly_utils::{assemble_instruction, pretty_print_instruction};
use crate::parser::parse_components;

use std::collections::HashMap;

pub fn assemble(file_contents: String) -> Result<(), String> {
    let instruction_table: HashMap<String, &'static InstructionInformation> = generate_instruction_hashmap();

    let mut symbol_table: Vec<Symbol> = vec!();
    let mut section_dot_text: Vec<u8> = vec!();

    let mut current_address = 0;
    let mut text_address = MIPS_TEXT_START_ADDR;
    let mut data_address = MIPS_DATA_START_ADDR;

    let mut section: Section = Section::Null;

    for line in file_contents.split('\n') {
        let line_components = parse_components(line.to_string());
        
        if Option::is_none(&line_components) { continue; }

        let mut instruction_information: Option<&'static InstructionInformation> = None;
        let mut arguments: Vec<LineComponent> = vec!();

        for component in line_components.unwrap() {
            match component.component_type {
                name_const::structs::ComponentType::Mnemonic => {
                    if let Some(retrieved_instruction_information) = instruction_table.get(&component.content){
                        instruction_information = Some(retrieved_instruction_information);
                    } else {
                        return Err("Failed to retrieve instruction information for specified mnemonic.".to_string());
                    }
                },
                name_const::structs::ComponentType::Register => {
                    arguments.push(component);
                },
                name_const::structs::ComponentType::Immediate => {
                    arguments.push(component);
                },
                name_const::structs::ComponentType::Identifier => {
                    arguments.push(component);
                    /*
                    If identifier represents a forward reference, add it to backpatches list
                    Else, don't bother
                     */
                },
                name_const::structs::ComponentType::Label => {
                    symbol_table.push(
                        Symbol { 
                            symbol_type: SymbolType::Address,
                            value: current_address, 
                        }
                    );
                },
                name_const::structs::ComponentType::Directive => {
                    match component.content.as_str() {
                        ".text" => {
                            match section {
                                Section::Null => {
                                    current_address = text_address
                                },
                                Section::Text => {
                                    return Err("Cannot declare section .text when already in section .text")?;
                                },
                                Section::Data => {
                                    data_address = current_address;
                                    current_address = text_address;
                                },
                            }

                            section = Section::Text;
                        },
                        ".data" => {
                            match section {
                                Section::Null => {
                                    current_address = text_address
                                },
                                Section::Text => {
                                    text_address = current_address;
                                    current_address = data_address;
                                },
                                Section::Data => {
                                    return Err("Cannot declare section .data when already in section .data")?;
                                },
                            }

                            section = Section::Data;
                        },
                        _ => {
                            return Err("Unrecognized directive")?;
                        }
                    }
                },
            }
        }

        match instruction_information {
            None => continue,
            Some(info) => {
                if let Ok(assembled_instruction) = assemble_instruction(info, arguments, &symbol_table){
                    section_dot_text.extend_from_slice(
                        &assembled_instruction.to_be_bytes()
                    );

                    pretty_print_instruction(&assembled_instruction);
                }
            }
        }

        match section {
            Section::Text => current_address += MIPS_ADDRESS_ALIGNMENT,
            _ => {},
        }
    }

    Ok(())
}