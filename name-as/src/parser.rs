use crate::tokens::Token; 

use crate::definitions::structs::LineComponent;

use logos::Logos;

// This function uses a regex library to simplify parsing.
pub fn parse_components(line: String) -> Result<Option<Vec<LineComponent>>, String> {
    let mut components = vec!();
    let mut lexer = Token::lexer(&line);

    let mut line_position: i32 = 0;
    let mut label_encountered: bool = false;

    let mut mnemonic_expected: bool = true;

    while let Some(token) = lexer.next() {
        let unwrapped_token = match token {
            Ok(tok) => tok,
            Err(_) => return Err(format!(" - NAME did not like that token: {:?}\n - In context: {}", lexer.slice(), line)),
        };
        let slice = lexer.slice();
        let component_result = token_to_line_component(unwrapped_token, slice,  mnemonic_expected);


        let component: LineComponent;
        match component_result {
            Ok(found) => component = found,
            Err(e) => return Err(e),
        }

        match component {
            LineComponent::Label(_) => label_encountered = true,
            _ => {},
        }

        line_position += 1;
        mnemonic_expected = line_position == 1 && label_encountered;

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
fn token_to_line_component(token: Token, slice: &str, mnemonic_expected: bool) -> Result<LineComponent, String> {
    match token {
        Token::Directive => return Ok(LineComponent::Directive(slice.to_string())),
        Token::Label => {
            return Ok(LineComponent::Label(slice[..slice.len()-1].to_string()));
        },
        Token::Identifier => {
            if mnemonic_expected {
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
        Token::SingleQuote => {
            let mut new_slice = slice.chars().skip(1);
            if new_slice.next().unwrap() as u32 == 0x5c { // 0x5c is backslash
                let escape_sequence_char = new_slice.next().unwrap();
                let escaped_slice: char = parse_escape_sequence(escape_sequence_char);
                return Ok(LineComponent::Immediate(escaped_slice as i32))
            } else {
                let escaped_slice: u32 = new_slice.next().unwrap() as u32; // TODO: Implement escape sequence stuff
                return Ok(LineComponent::Immediate(escaped_slice as i32))
            }
        }
        Token::DoubleQuote => {
            // return Ok(LineComponent::DoubleQuote(slice[1..slice.len()-1].to_string()));

            let bingbong = &slice[1..slice.len() - 1];
            let mut result = String::new();
            let mut chars = bingbong.chars().peekable();

            // this works for now i'll deal with it later
            while let Some(c) = chars.next() {
                if c == '\\' {
                    if let Some(&next_char) = chars.peek() {
                        let parsed_char = parse_escape_sequence(next_char);
                        result.push(parsed_char);
                        chars.next();
                    } else {
                        result.push(c);
                    }
                } else {
                    result.push(c);
                }
            }

            return Ok(LineComponent::DoubleQuote(result));
        }
        Token::Colon => {
            return Ok(LineComponent::Colon);
        }
        _ => return Err(format!("pattern \"{slice}\" could not be matched by parser.")),
    }
}

// This function just maps a character to its respective escape sequence.
fn parse_escape_sequence(escaped_char: char) -> char {
    let escaped_slice_char: char = match escaped_char {
        't' =>  9 as char, // tab
        'n' => 10 as char, // newline
        'r' => 13 as char, // carriage return (who uses this)
        'b' =>  8 as char, // backspace (what.)
        'f' => 12 as char, // form feed (yeah okay)
        '\'' => '\'',  // take this out if it doesn't work or is redundant
        '\"' => '\"',  // (this might be the only non redundant part actually)
        '\\' => '\\',
        _ => {
            println!("Escape sequence \\{} not implemented.", escaped_char);
            return escaped_char
        }
    };
    return escaped_slice_char
}