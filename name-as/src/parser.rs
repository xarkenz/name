use crate::tokens::Token; 

use name_const::structs::LineComponent;

use logos::Logos;

// This function uses a regex library to simplify parsing.
pub fn parse_components(line: String, mnemonics: &Vec<String>) -> Result<Option<Vec<LineComponent>>, String> {
    let mut components = vec!();
    let mut lexer = Token::lexer(&line);

    while let Some(token) = lexer.next() {
        let slice = lexer.slice();
        let component_result = token_to_line_component(token.unwrap(), slice, &mnemonics);


        let component: LineComponent;
        if component_result.is_err() {
            return Err(component_result.unwrap_err());
        } else {
            component = component_result.unwrap();
        }

        components.push(component);
    }

    match components.len() {
        0 => {
            return Ok(None);
        },
        _ => {
            return Ok(Some(components));
        }
    }
}

// Required implementation for regex library
fn token_to_line_component(token: Token, slice: &str, mnemonics: &Vec<String>) -> Result<LineComponent, String> {
    match token {
        Token::Directive => return Ok(LineComponent::Directive(slice.to_string())),
        Token::Label => {
            return Ok(LineComponent::Label(slice[..slice.len()-1].to_string()));
        },
        Token::Identifier => {
            if mnemonics.contains(&slice.to_string()) {
                return Ok(LineComponent::Mnemonic(slice.to_string()));
            } else {
                return Ok(LineComponent::Identifier(slice.to_string()));
            }
        },
        Token::Register => return Ok(LineComponent::Register(slice.to_string())),
        Token::HexNumber => {
            if let Ok(value) = i32::from_str_radix(&slice[2..], 16) {
                return Ok(LineComponent::Immediate(value));
            } else {
                return Err("Failed to parse as hexadecimal.".to_string());
            }
        },
        Token::BinaryNumber => {
            if let Ok(value) = i32::from_str_radix(&slice[2..], 2) {
                return Ok(LineComponent::Immediate(value));
            } else {
                return Err("Failed to parse as binary.".to_string());
            }
        },
        Token::OctalNumber => {
            if let Ok(value) = i32::from_str_radix(&slice[1..], 8) {
                return Ok(LineComponent::Immediate(value));
            } else {
                return Err("Failed to parse as octal.".to_string());
            }
        },
        Token::DecimalNumber => {
            if let Ok(value) = i32::from_str_radix(&slice, 10) {
                return Ok(LineComponent::Immediate(value));
            } else {
                return Err("Failed to parse as decimal.".to_string());
            }
        },
        _ => return Err(format!("pattern \"{slice}\" could not be matched by parser.")),
    }
}