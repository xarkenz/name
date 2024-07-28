use crate::assembler::assembly_helpers::translate_identifier_to_address;
use crate::constants::structs::{InstructionInformation, LineComponent};
use crate::assembler::assembler::Assembler;

use super::constants::BACKPATCH_PLACEHOLDER;

/*
Each pseudo instruction must implement its own `expand` fn. This function expands the pseudoinstruction's content into its respective instructions.

It does this either by mapping existing arguments, or by creating new ones based on existing. Take `li` and `la` as examples, respectively.

It does not need to check its own argument setup. It can just piggy-back off existing logic from the main instruction assembly.
Any errors will clearly have code ID10T on the part of the user attempting to use the pseudoinstruction.
*/

pub(crate) type ExpansionFn = fn(&Assembler, &Vec<LineComponent>) -> Result<Vec<(&'static InstructionInformation, Vec<LineComponent>)>, String>;

pub(crate) fn expand_li(environment: &Assembler, args: &Vec<LineComponent>) -> Result<Vec<(&'static InstructionInformation, Vec<LineComponent>)>, String> {
    if args.len() < 2 {
        return Err(format!(" - `li` expected 2 arguments, got {}", args.len()));
    }
    
    let rd = args[0].clone();
    let imm = args[1].clone();

    let ori_info = match environment.instruction_table.get("ori") {
        Some(info) => info,
        None => return Err(format!(" - Failed to expand `li` psedoinstruction. Its expansion was likely defined incorrectly (go use git blame on https://github.com/cameron-b63/name to find out who's at fault)."))
    };
    
    Ok(vec![
        // ori  $rd, $rd, imm
        (ori_info, vec![rd.clone(), rd, imm])
    ])
}

pub(crate) fn expand_la(environment: &Assembler, args: &Vec<LineComponent>) -> Result<Vec<(&'static InstructionInformation, Vec<LineComponent>)>, String> {
    if args.len() < 2 {
        return Err(format!(" - `la` expected 2 arguments, got {}", args.len()));
    }
    
    let rd = args[0].clone();
    let label = args[1].clone();

    let lui_info = match environment.instruction_table.get("lui") {
        Some(info) => info,
        None => return Err(format!(" - Failed to expand `la` pseudoinstruction. Its expanstion was likely defined incorrectly (go use git blame on https://github.com/cameron-b63/name to find out who's at fault).")),
    };

    let ori_info = match environment.instruction_table.get("ori") {
        Some(info) => info,
        None => return Err(format!(" - Failed to expand `li` psedoinstruction. Its expansion was likely defined incorrectly (go use git blame on https://github.com/cameron-b63/name to find out who's at fault)."))
    };

    // This is where things get ludicrous. Backpatching needs to be accounted for here.
    // A more sophisticated version of backpatching is necessary for this exact reason.

    let mut resolved_symbol_value: u32 = BACKPATCH_PLACEHOLDER; 
    if let LineComponent::Label(identifier) = label {
        match translate_identifier_to_address(&identifier, &environment.symbol_table) {
            Some(addr) => resolved_symbol_value = addr,
            None => {
                todo!("Implement backpatching across pseudoinstruction expansions.");
            },
        }
    }

    let upper = LineComponent::Immediate((resolved_symbol_value >> 16) as i32);
    let lower = LineComponent::Immediate((resolved_symbol_value & 0xFFFF) as i32);

    Ok(vec![
        // lui  $rd, UPPER
        (lui_info, vec![rd.clone(), upper]),
        // ori  $rd, $rd, LOWER
        (ori_info, vec![rd.clone(), rd, lower]),
    ])
}