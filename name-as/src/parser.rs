use crate::assembly_utils::base_parse;

use name_const::structs::{ComponentType, LineComponent};
use name_const::helpers::get_mnemonics;


pub fn parse_components(line: String) -> Option<Vec<LineComponent>> {
    let mut components: Vec<LineComponent> = vec!();

    for word in line.replace(","," ").split_whitespace() {
        if word.starts_with('#') {
            // Disregard entire commented portion and return
            break;
        } else if word.starts_with('$') {
            let register: LineComponent = LineComponent {
                component_type: ComponentType::Register,
                content: word.to_string(),
            };

            components.push(register);
        } else if word.starts_with('.') {
            let directive: LineComponent = LineComponent {
                component_type: ComponentType::Directive,
                content: word.to_string(),
            };

            components.push(directive);
        } else if word.ends_with(':') {
            let label: LineComponent = LineComponent {
                component_type: ComponentType::Label,
                content: word[..word.len()-1].to_string(),
            };

            components.push(label);
        } else if let Ok(_) = base_parse(word) {
            let immediate: LineComponent = LineComponent {
                component_type: ComponentType::Immediate,
                content: word.to_string(),
            };

            components.push(immediate);
        } else if word.chars().all(|c| c.is_alphanumeric()) {
            if get_mnemonics().contains(&word.to_string()) {
                let mnemonic: LineComponent = LineComponent {
                    component_type: ComponentType::Mnemonic,
                    content: word.to_string(),
                };

                components.push(mnemonic);
            } else {
                let identifier: LineComponent = LineComponent {
                    component_type: ComponentType::Identifier,
                    content: word.to_string(),
                };

                components.push(identifier);
            }
        }
    }

    match components.len() {
        0 => {
            return None;
        },
        _ => {
            return Some(components);
        }
    }
}