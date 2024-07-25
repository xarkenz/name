use std::fs::read_to_string;
use std::path::PathBuf;

use crate::assembler::assemble_file::assemble;
use crate::assembler::assembler::Assembler;

use crate::constants::structs::LineComponent;

use name_const::structs::Section;

impl Assembler {

    // .asciiz
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

    // .eqv
    pub(crate) fn new_eqv(&mut self, arguments: &Vec<LineComponent>) {
        if arguments.len() < 2 {
            self.errors.push(format!("[*] On line {}{}:", self.line_prefix, self.line_number));
            self.errors.push(format!(" - `.eqv` expected 2 or more arguments, received {}.", arguments.len()));
            return;
        }
        
        let name: String;
        let expansion: String;

        match &arguments[0] {
            LineComponent::Identifier(ident) => name = ident.clone(),
            _ => {
                self.errors.push(format!("[*] On line {}{}:", self.line_prefix, self.line_number));
                self.errors.push(format!(" - `.eqv` expected identifier, found {:?}", &arguments[0]));
                return;
            },
        }

        expansion = arguments.iter().skip(1).map(|component| component.to_string()).collect::<Vec<String>>().join(" ");

        self.equivalences.insert(name, expansion);
    }


    // .include
    pub(crate) fn include_file(&mut self, arguments: &Vec<LineComponent>) {
        if arguments.len() != 1 {
            self.errors.push(format!("[*] On line {}{}:", self.line_prefix, self.line_number));
            self.errors.push(format!(" - Too many arguments for .include directive provided on line {} (expected 1 argument)", self.line_number));
            return;
        }

        let filename: PathBuf = match arguments[0].clone() {
            LineComponent::DoubleQuote(quoted_filename) => self.current_dir.join(PathBuf::from(quoted_filename)),
            _ => {
                self.errors.push(format!("[*] On line {}{}:", self.line_prefix, self.line_number));
                self.errors.push(format!(" - .include expects a double quoted string filename"));
                return;
            },
        };

        println!("\n[+] Found .include, attempting to include {:?}", filename);

        let file_contents = match read_to_string(&filename) {
            Ok(content) => content,
            Err(_) => {
                self.errors.push(format!("[*] On line {}{}:", self.line_prefix, self.line_number));
                self.errors.push(format!("Could not open file {:?} referenced on line {}.", filename, self.line_number));
                return;
            },
        };
        
        let line_prefix: String = format!("{}  {}->", self.line_prefix, self.line_number);

        let returned_assembler: Result<Assembler, Vec<String>> = assemble(file_contents, filename, Some(line_prefix));
        match returned_assembler {
            Ok(returned_env) => {
                self.expandables.extend(returned_env.expandables);
            },
            Err(errors) => {
                self.errors.extend(errors);
            },
        }

        println!("[+] Module included.\n");
    }

    pub(crate) fn switch_to_text_section(&mut self) {
        match self.current_section {
            Section::Null => {
                self.current_address = self.text_address;
            },
            Section::Text => {
                self.errors.push(format!("[*] On line {}{}:", self.line_prefix, self.line_number));
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
                self.errors.push(format!("[*] On line {}{}:", self.line_prefix, self.line_number));
                self.errors.push(format!(" - Cannot declare current_section .data when already in current_section .data (line {})", self.line_number));
            },
        }

        self.current_section = Section::Data;
    }
}