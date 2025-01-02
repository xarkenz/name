use crate::assembler::assembler::Assembler;

use crate::definitions::structs::LineComponent;

impl Assembler {
    pub(crate) fn handle_directive(&mut self, ident: &str, arguments: &Vec<LineComponent>) {
        match ident {
            ".asciiz" => {
                self.add_new_asciiz(arguments);
            }
            ".data" => {
                self.switch_to_data_section();
            }
            ".eqv" => {
                self.new_eqv(arguments);
            }
            ".include" => {
                self.include_file(arguments);
            }
            ".text" => {
                self.switch_to_text_section();
            }
            ".word" => {
                self.new_word(arguments);
            }
            _ => {
                self.errors.push(format!("[*] On line {}:", self.line_number));
                self.errors.push(format!(" - Directive \"{ident}\" not recognized."));
            }
        }
    }
}
