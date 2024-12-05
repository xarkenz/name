use std::collections::HashMap;
use std::path::PathBuf;

use name_core::constants::{MIPS_ADDRESS_ALIGNMENT, MIPS_DATA_START_ADDR, MIPS_TEXT_START_ADDR};
use name_core::elf_def::{STT_FUNC, STT_OBJECT};
use name_core::instruction::information::InstructionInformation;
use name_core::structs::{Section, Symbol, Visibility};

use crate::assembler::assemble_instruction::assemble_instruction;
use crate::assembler::assembly_helpers::{
    generate_pseudo_instruction_hashmap, pretty_print_instruction,
};

use crate::definitions::structs::{LineComponent, PseudoInstruction};

// This file contains the struct definition and extracted functions used in the assembler_logic file. There was far too much inlined, so I have extracted it.

#[derive(Debug)]
pub struct Assembler {
    pub(crate) pseudo_instruction_table: HashMap<&'static str, &'static PseudoInstruction>,
    pub section_dot_text: Vec<u8>,
    pub section_dot_data: Vec<u8>,
    pub section_dot_rel: Vec<u8>,
    pub section_dot_line: Vec<u8>,
    pub symbol_table: Vec<Symbol>,
    pub(crate) equivalences: HashMap<String, String>,
    pub(crate) errors: Vec<String>,
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
            pseudo_instruction_table: generate_pseudo_instruction_hashmap(),
            section_dot_text: vec![],
            section_dot_data: vec![],
            section_dot_rel: vec![],
            section_dot_line: vec![],
            symbol_table: vec![],
            equivalences: HashMap::new(),
            errors: vec![],
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
        self.symbol_table
            .iter()
            .find(|symbol| symbol.identifier == ident)
            .is_some()
    }

    // Add a label to the symbol table.
    pub(crate) fn add_label(&mut self, ident: &String) {
        self.symbol_table.push(Symbol {
            symbol_type: match self.current_section {
                Section::Null => {
                    self.errors
                        .push(format!("[*] On line {}:", self.line_number));
                    self.errors
                        .push(" - Cannot declare label outside a section.".to_string());
                    0
                }
                Section::Text => STT_FUNC,
                Section::Data => STT_OBJECT,
            },
            identifier: ident.to_owned(),
            value: self.current_address,
            size: 4,
            visibility: Visibility::Local,
            section: self.current_section.clone(),
        });

        println!("Inserted symbol {} at 0x{:x}", ident, self.current_address);

        self.most_recent_label = ident.clone();
    }

    // Expand a line. Try replacing all instances of equivalences.
    pub fn expand_line(&self, line: &str) -> String {
        let mut expanded_line = String::new();

        // Replace equivalences
        for token in line.split_whitespace() {
            if let Some(expansion) = self.equivalences.get(token) {
                expanded_line.push_str(expansion);
            } else {
                expanded_line.push_str(token);
            }

            expanded_line.push(' ');
        }

        expanded_line.trim_end().to_string()
    }

    // Attempt to assemble a parsed line. If successful, add bytes to section .text - else, extend errors and keep it pushing.
    pub fn handle_assemble_instruction(
        &mut self,
        info: &InstructionInformation,
        args: &Vec<LineComponent>,
    ) {
        let assembled_instruction_result =
            assemble_instruction(info, &args, &self.symbol_table, &mut self.section_dot_rel, &self.current_address);

        match assembled_instruction_result {
            Ok(assembled_instruction) => match assembled_instruction {
                packed => {
                    self.section_dot_text
                        .extend_from_slice(&packed.to_be_bytes());

                    pretty_print_instruction(&self.current_address, &packed);
                }
            },
            Err(e) => {
                self.errors.push(format!(
                    "[*] On line {}{}:",
                    self.line_prefix, self.line_number
                ));
                self.errors.push(e);
            }
        }

        self.current_address += MIPS_ADDRESS_ALIGNMENT;
    }
}
