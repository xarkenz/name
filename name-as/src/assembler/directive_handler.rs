use crate::assembler::assembler::Assembler;

use name_const::structs::{LineComponent, Section};

impl Assembler {
    pub(crate) fn handle_directive(&mut self, ident: &String, _arguments: &Vec<LineComponent>) {
        match ident.as_str() {
            ".data" => {
                self.switch_to_data_section();
            },
            ".text" => {
                self.switch_to_text_section();
            },
            ".include" => {
                todo!("Implement .include directive");
            }
            _ => {
                self.errors.push(format!(" - Unrecognized directive on line {}", self.line_number));
            }
        }
    }

    fn switch_to_text_section(&mut self) {
        match self.current_section {
            Section::Null => {
                self.current_address = self.text_address
            },
            Section::Text => {
                self.errors.push(format!(" - Cannot declare current_section .text when already in current_section .text on line {}", self.line_number));
            },
            Section::Data => {
                self.data_address = self.current_address;
                self.current_address = self.text_address;
            },
        }

        self.current_section = Section::Text;
    }

    fn switch_to_data_section(&mut self) {
        match self.current_section {
            Section::Null => {
                self.current_address = self.text_address
            },
            Section::Text => {
                self.text_address = self.current_address;
                self.current_address = self.data_address;
            },
            Section::Data => {
                self.errors.push(format!(" - Cannot declare current_section .data when already in current_section .data (line {})", self.line_number));
            },
        }

        self.current_section = Section::Data;
    }
}