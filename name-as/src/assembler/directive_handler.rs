use crate::assembler::assembler::Assembler;

use crate::constants::structs::LineComponent;

impl Assembler {
    pub(crate) fn handle_directive(&mut self, ident: &String, arguments: &Vec<LineComponent>) {
        match ident.as_str() {
            ".asciiz" => {
                self.add_new_asciiz(arguments);
            }
            ".data" => {
                self.switch_to_data_section();
            },
            ".eqv" => {
                self.new_eqv(arguments);
            }
            ".include" => {
                self.include_file(arguments);
            }
            ".text" => {
                self.switch_to_text_section();
            },
            _ => {
                self.errors.push(format!("[*] On line {}:", self.line_number));
                self.errors.push(format!(" - Unrecognized directive."));
            }
        }
    }
}