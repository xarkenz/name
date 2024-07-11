use name_const::structs::{ArgumentType, InstructionInformation, InstructionType, LineComponent, Symbol};
use crate::assembly_utils::*;
use crate::assembly_helpers::{arg_configuration_is_ok, translate_identifier_to_address};

// Big logic for instruction assembly - this is the main driver code for actual packing of instructions once parsed.
pub fn assemble_instruction(info: &InstructionInformation, arguments: &Vec<LineComponent>, symbol_table: &Vec<Symbol>, current_address: &u32) -> Result<Option<u32>, String> {
    let has_alternate_configurations: bool = info.alt_args.is_some();

    // Find proper argument configuration early
    let configuration_to_use: &[ArgumentType];
    let existing_configuration_works: bool = arg_configuration_is_ok(arguments, info.args);

    if !has_alternate_configurations {
        if !existing_configuration_works{
            return Err(" - Bad arguments provided.".to_string());
        } else {
            configuration_to_use = info.args;
        }
    } else if !existing_configuration_works {
        let found_configuration = info.alt_args.unwrap().iter().find(|config| arg_configuration_is_ok(arguments, config));
        match found_configuration {
            Some(configuration) => configuration_to_use = configuration,
            None => return Err(" - Alternative argument configurations exist, but none matched.".to_string()),
        }
    } else {
        configuration_to_use = info.args;
    }

    match info.instruction_type {
        InstructionType::RType => {
            let funct: u32 = info.funct.expect("Improper implmentation of instructions (funct field undefined for R-type instr)\nIf you are a student reading this, understand this error comes entirely from a fundamental failure in the codebase of this vscode extension.");

            let (rd, rs, rt, shamt) = match assign_r_type_arguments(arguments, configuration_to_use) {
                Ok((rd, rs, rt, shamt)) => (rd, rs, rt, shamt),
                Err(e) => return Err(e),
            };

            match assemble_r_type(rd, rs, rt, shamt, funct){
                Ok(packed_instr) => {
                    return Ok(Some(packed_instr));
                },
                Err(e) => {
                    return Err(e);
                }
            }
        },
        InstructionType::IType => {
            let opcode: u32 = info.opcode.expect("Improper implmentation of instructions (opcode undefined for I-type instr)\nIf you are a student reading this, understand this error comes entirely from the codebase of this vscode extension.");

            let (rs, rt, imm) = match assign_i_type_arguments(arguments, configuration_to_use, symbol_table, current_address) {
                Ok( (rs, rt, imm) ) => (rs, rt, imm),
                Err(e) => return Err(e),
            };

            if imm.is_none() && configuration_to_use.contains(&ArgumentType::BranchLabel) {
                return Ok(None);
            }

            match assemble_i_type(opcode, rs, rt, imm){
                Ok(packed_instr) => {
                    return Ok(Some(packed_instr));
                },
                Err(e) => {
                    return Err(e);
                }
            }
        },
        InstructionType::JType => {
            let opcode: u32 = info.opcode.expect("Improper implmentation of instructions (opcode undefined for J-type instr)\nIf you are a student reading this, understand this error comes entirely from the codebase of this vscode extension.");
            
            
            let identifier = match assign_j_type_arguments(arguments, configuration_to_use) {
                Ok(identifier) => identifier,
                Err(e) => return Err(e),
            };

            let address = match translate_identifier_to_address(&identifier, symbol_table) {
                Some(addr) => addr >> 2,
                None => return Ok(None),    // Unresolved symbol (forward reference)
            };

            let assembled_output = assemble_j_type(opcode, Some(address));

            match assembled_output {
                Ok(_) => {
                    return assembled_output;
                },
                Err(e) => {
                    return Err(e);
                }
            }
        },
    }
}

#[test]
fn assemble_instruction_test() {
    // R-Type test
    let instruction_table = name_const::helpers::generate_instruction_hashmap();
    let add_info = instruction_table.get(&"add".to_string()).unwrap();

    let arguments: Vec<&'static str> = vec!["$t0", "$t1", "$t2"];
    let wrapped_arguments: Vec<LineComponent> = arguments.into_iter().map(|x| 
            LineComponent::Register(x.to_string())
    ).collect();

    let mock_symbol_table: Vec<Symbol> = vec![
        Symbol {symbol_type: 2, identifier:"test".to_string(),value:0x004020, size: 4, visibility: name_const::structs::Visibility::Local, section: name_const::structs::Section::Text }
    ]; 

    let mock_current_address = name_const::elf_def::MIPS_TEXT_START_ADDR;

    assert_eq!(assemble_instruction(add_info, &wrapped_arguments, &mock_symbol_table, &mock_current_address), Ok(Some(0x012A4020)));
    
    // J-Type test
    let jal_info = instruction_table.get(&"jal".to_string()).unwrap();
    let arguments: Vec<&'static str> = vec!["test"];
    let wrapped_arguments: Vec<LineComponent> = arguments.into_iter().map(|x| 
            LineComponent::Identifier(x.to_string())
    ).collect();

    assert_eq!(assemble_instruction(jal_info, &wrapped_arguments, &mock_symbol_table, &mock_current_address), Ok(Some(0x0c004020)));

    let mock_symbol_table: Vec<Symbol> = vec!();

    assert_eq!(assemble_instruction(jal_info, &wrapped_arguments, &mock_symbol_table, &mock_current_address), Ok(None));
}