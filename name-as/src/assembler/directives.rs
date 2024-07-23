use std::fs::read_to_string;
use std::path::PathBuf;

use crate::assembler::assemble_file::assemble;
use crate::assembler::assembler::Assembler;
use crate::assembler::expandables::Equivalence;

use name_const::structs::{LineComponent, Section};

impl Assembler {
    pub(crate) fn add_new_asciiz(&mut self, arguments: &Vec<LineComponent>) {
        if arguments.len() != 1 {
            self.errors.push(format!(" - `.asciiz` directive expects only one argument, received {}", arguments.len()));
            return;
        }

        let mut to_push: Vec<u8> = arguments[0].to_string().chars().map(|c| c as u8).collect::<Vec<u8>>();
        to_push.push(b'\0');

        self.current_address += to_push.len() as u32;
        self.section_dot_data.extend(&to_push);

        match self.symbol_table.iter_mut().find(|s| s.identifier == self.most_recent_label) {
            Some(res) => res.size = to_push.len() as u32,
            None => {},
        }
    }
    pub(crate) fn new_eqv(&mut self, arguments: &Vec<LineComponent>) {
        if arguments.len() < 2 {
            self.errors.push(format!(" - `.eqv` expected 2 or more arguments, received {}.", arguments.len()));
            return;
        }
        
        let name: String;
        let expansion: String;

        match &arguments[0] {
            LineComponent::Identifier(ident) => name = ident.clone(),
            _ => {
                self.errors.push(format!(" - `.include` expected identifier, found {:?}", std::any::Any::type_id(&arguments[0])));
                return;
            },
        }

        expansion = arguments.iter().skip(1).map(|component| component.to_string()).collect::<Vec<String>>().join(" ");

        self.expandables.push(Box::new(
            Equivalence {
                name: name,
                expansion: expansion,
            }
        ));
    }

    pub(crate) fn include_file(&mut self, arguments: &Vec<LineComponent>) {
        if arguments.len() != 1 {
            self.errors.push(format!(" - Too many arguments for .include directive provided on line {} (expected 1 argument)", self.line_number));
            return;
        }

        let filename: PathBuf = match arguments[0].clone() {
            LineComponent::DoubleQuote(quoted_filename) => self.current_dir.join(PathBuf::from(quoted_filename)),
            _ => {
                self.errors.push(format!(" - .include expects a double quoted string filename"));
                return;
            },
        };

        let file_contents = match read_to_string(&filename) {
            Ok(content) => content,
            Err(_) => {
                self.errors.push(format!("Could not open file {:?} referenced on line {}.", filename, self.line_number));
                return;
            },
        };

        for line in file_contents.split('\n') {
            if line.trim().starts_with('#') {
                continue;
            }
            if line.trim().starts_with(".eqv") {
                let returned_env = match assemble(line.to_string(), self.current_dir.clone()) {
                    Ok(res) => res,
                    Err(e) => {
                        self.errors.extend(e);
                        continue;
                    },
                };

                self.expandables.extend(returned_env.expandables);
            } else {
                self.errors.push(format!("`.include` files may only contain preprocessor macros (like .eqv, .macro, etc.)"));
            }
        }
    }

    pub(crate) fn switch_to_text_section(&mut self) {
        match self.current_section {
            Section::Null => {
                self.current_address = self.text_address;
            },
            Section::Text => {
                self.errors.push(format!("[*] On line {}:", self.line_number));
                self.errors.push(format!(" - Cannot declare current_section .text when already in current_section .text on line {}", self.line_number));
            },
            Section::Data => {
                self.data_address = self.current_address;
                self.current_address = self.text_address;
            },
        }

        self.current_section = Section::Text;
    }

    pub(crate) fn switch_to_data_section(&mut self) {
        match self.current_section {
            Section::Null => {
                self.current_address = self.data_address
            },
            Section::Text => {
                self.text_address = self.current_address;
                self.current_address = self.data_address;
            },
            Section::Data => {
                self.errors.push(format!("[*] On line {}:", self.line_number));
                self.errors.push(format!(" - Cannot declare current_section .data when already in current_section .data (line {})", self.line_number));
            },
        }

        self.current_section = Section::Data;
    }
}