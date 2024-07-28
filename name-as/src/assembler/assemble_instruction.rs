use name_const::structs::Symbol;
use crate::assembler::assembly_utils::*;
use crate::assembler::assembly_helpers::{arg_configuration_is_ok, translate_identifier_to_address};
use crate::definitions::structs::{ArgumentType, InstructionInformation, InstructionType, LineComponent};

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