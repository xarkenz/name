use std::collections::HashMap;
use std::path::PathBuf;

use name_const::elf_def::{MIPS_DATA_START_ADDR, MIPS_TEXT_START_ADDR, STT_FUNC, STT_OBJECT};
use name_const::structs::{Section, Symbol, Visibility};

use crate::assembler::assemble_instruction::assemble_instruction;
use crate::assembler::assembly_helpers::{generate_instruction_hashmap, get_mnemonics, pretty_print_instruction};

use crate::constants::expandables::Expandable;
use crate::constants::pseudo_instructions::PSEUDO_INSTRUCTION_SET;
use crate::constants::structs::{Backpatch, InstructionInformation, LineComponent, PseudoInstruction};

// This file contains the struct definition and extracted functions used in the assembler_logic file. There was far too much inlined, so I have extracted it.

#[derive(Debug)]
pub(crate) struct Assembler {
    pub(crate) instruction_table: std::collections::HashMap<String, &'static InstructionInformation>,
    pub(crate) mnemonics: Vec<String>,
    pub(crate) section_dot_text: Vec<u8>,
    pub(crate) section_dot_data: Vec<u8>,
    pub(crate) symbol_table: Vec<Symbol>,
    pub(crate) equivalences: HashMap<String, String>,
    pub(crate) expandables: Vec<Box<dyn Expandable>>,
    pub(crate) errors: Vec<String>,
    pub(crate) backpatches: Vec<Backpatch>,
    pub(crate) current_section: Section,
    pub(crate) current_address: u32,
    pub(crate) current_dir: PathBuf,
    pub(crate) text_address: u32,
    pub(crate) data_address: u32,
    pub(crate) line_number: usize,
    pub(crate) line_prefix: String,
    pub(crate) most_recent_label: String,
}

impl Assembler {

    // Initialize the assembler environment - default constructor.
    pub(crate) fn new() -> Self {
        Assembler {
            instruction_table: generate_instruction_hashmap(),
            mnemonics: get_mnemonics(),
            section_dot_text: vec![],
            section_dot_data: vec![],
            symbol_table: vec![],
            equivalences: HashMap::new(),
            expandables: vec![],
            errors: vec![],
            backpatches: vec![],
            current_section: Section::Null,
            current_address: 0,
            current_dir: PathBuf::new(),
            text_address: MIPS_TEXT_START_ADDR,
            data_address: MIPS_DATA_START_ADDR,
            line_number: 1,
            line_prefix: String::from(""),
            most_recent_label: String::from(""),
        }
    }

    // Check if a target symbol exists in the symbol table.
    // Returns a boolean representing if the symbol is present.
    pub(crate) fn symbol_exists(&self, symbol_identifier: &String) -> bool {
        let ident = symbol_identifier.clone();
        self.symbol_table.iter().find(|symbol| symbol.identifier == ident).is_some()
    }

    // Add a backpatch to the backpatches list. Used if a forward reference was detected to signal that it must be resolved later.
    pub(crate) fn add_backpatch(&mut self, instruction_info: &'static InstructionInformation, args: &Vec<LineComponent>, ident: String) {
        self.backpatches.push(
            Backpatch {
                instruction_info: instruction_info,
                arguments: args.clone(),
                undiscovered_identifier: ident,
                backpatch_address: self.current_address,
                byte_offset: self.section_dot_text.len(),
                line_number: self.line_number,
            }
        );
    }

    // Add a label to the symbol table.
    pub(crate) fn add_label(&mut self, ident: &String) {
        self.symbol_table.push(
            Symbol {
                symbol_type: match self.current_section {
                    Section::Null => {
                        self.errors.push(format!("[*] On line {}:", self.line_number));
                        self.errors.push(" - Cannot declare label outside a section.".to_string());
                        0
                    },
                    Section::Text => STT_FUNC,
                    Section::Data => STT_OBJECT,
                },
                identifier: ident.to_owned(),
                value: self.current_address,
                size: 4,
                visibility: Visibility::Local,
                section: self.current_section.clone(), 
            }
        );

        self.most_recent_label = ident.clone();
    }

    // Resolve all backpatches attached to a label. Used once a forward-reference label has been discovered and defined.
    pub(crate) fn resolve_backpatches(&mut self, ident: &String) {
        let label: String = ident.clone();

        loop {
            let backpatch_found = self.backpatches.iter().find(|backpatch| backpatch.undiscovered_identifier == label );
            let backpatch: &Backpatch;

            if backpatch_found.is_none() { 
                break; 
            } else {
                backpatch = backpatch_found.unwrap();
            }

            let backpatch_address: u32 = backpatch.backpatch_address;
            
            let assembled_result = assemble_instruction(backpatch.instruction_info, &backpatch.arguments, &self.symbol_table, &backpatch_address);
            match assembled_result {
                Ok(assembled_instruction) => {
                    match assembled_instruction {
                        Some(word) => {
                            let insert_offset = backpatch.byte_offset;
                            let bytes_to_insert = word.to_be_bytes();

                            self.section_dot_text.splice(insert_offset..insert_offset+4, bytes_to_insert.iter().cloned());
                            
                            let label = backpatch.undiscovered_identifier.clone();
                            let line = backpatch.line_number.clone();
                            println!(" - Backpatch resolved for label {label} on line {line}:");
                            pretty_print_instruction(&word);

                            let found_index = self.backpatches.iter().position(|bp| bp == backpatch).expect("Literally impossible to get this error.");

                            self.backpatches.remove(found_index);
                        },
                        None => {
                            unreachable!("Backpatch unable to be resolved. This indicates a stupidly difficult extension error.");
                        },
                    }
                },
                Err(e) => {
                    self.errors.push(e);
                    let found_index = self.backpatches.iter().position(|bp| bp == backpatch).expect("Literally impossible to get this error.");

                    self.backpatches.remove(found_index);
                },
            }
            
        }
    }

    // Expand a line. Try replacing all instances of equivalences first, then expand pseudoinstructions.
    pub fn expand_line(&self, line: &str) -> String {
        let mut expanded_line = String::new();

        let mut found_pseudo: Option<&PseudoInstruction> = None;

        // Replace equivalences
        for token in line.split_whitespace() {
            if let Some(expansion) = self.equivalences.get(token) {
                expanded_line.push_str(expansion);
            } else {
                expanded_line.push_str(token);
            }

            if let Some(pseudo_instruction_info) = PSEUDO_INSTRUCTION_SET.iter().find(|instr| instr.mnemonic == token) {
                found_pseudo = Some(pseudo_instruction_info);
            } 

            expanded_line.push(' ');
        }

        // Expand multilines
        match found_pseudo {
            Some(instr) => expanded_line = instr.expand(expanded_line.as_str()),
            None => {},
        }

        expanded_line.trim_end().to_string()
    }

}