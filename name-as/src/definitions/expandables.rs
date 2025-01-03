use crate::assembler::assembler::Assembler;
use crate::definitions::{constants::INSTRUCTION_TABLE, structs::LineComponent};
use name_core::constants::MIPS_TEXT_START_ADDR;
use name_core::elf_def::{RelocationEntry, RelocationEntryType};
use name_core::instruction::information::InstructionInformation;

/*
Each pseudo instruction must implement its own `expand` fn. This function expands the pseudoinstruction's content into its respective instructions.

It does this either by mapping existing arguments, or by creating new ones based on existing. Take `li` and `la` as examples, respectively.

It does not need to check its own argument setup. It can just piggy-back off existing logic from the main instruction assembly.
Any errors will clearly have code ID10T on the part of the user attempting to use the pseudoinstruction.
*/

pub(crate) type ExpansionFn =
    fn(
        &mut Assembler,
        &Vec<LineComponent>,
    ) -> Result<Vec<(&'static InstructionInformation, Vec<LineComponent>)>, String>;

// pub(crate) fn expand_bgt(environment: &mut Assembler, args: &Vec<LineComponent>) -> Result<Vec<(&'static InstructionInformation, Vec<LineComponent>)>, String> {
//     if args.len() < 3 {
//         return Err(format!(" - `li` expected 2 arguments, got {}", args.len()));
//     }

//     let rs: LineComponent = args[0].clone();
//     let rt: LineComponent = args[1].clone();
//     let label: LineComponent = args[2].clone();

//     let as_temp: LineComponent = LineComponent::Register(String::from("$1"));
//     let zero: LineComponent = LineComponent::Register(String::from("$0"));

//     let slt_info: &'static InstructionInformation;
//     let bne_info: &'static InstructionInformation;

//     {
//         // Immutable borrows are contained within this block
//         slt_info = match environment.instruction_table.get("slt") {
//             Some(info) => info,
//             None => return Err(format!(" - Failed to expand `bgt` pseudoinstruction. Its expansion was likely defined incorrectly (go use git blame on https://github.com/cameron-b63/name to find out who's at fault).")),
//         };

//         bne_info = match environment.instruction_table.get("bne") {
//             Some(info) => info,
//             None => return Err(format!(" - Failed to expand `bgt` pseudoinstruction. Its expansion was likely defined incorrectly (go use git blame on https://github.com/cameron-b63/name to find out who's at fault).")),
//         };
//     }

//     // This is where things get ludicrous. Backpatching needs to be accounted for here.
//     // A more sophisticated version of backpatching is necessary for this exact reason.

//     let mut resolved_symbol_value: u32 = BACKPATCH_PLACEHOLDER;
//     let mut must_backpatch: bool = false;
//     let identifier: String;

//     match label {
//         LineComponent::Identifier(ident) => {
//             identifier = ident;
//             match translate_identifier_to_address(&identifier, &environment.symbol_table) {
//                 Some(addr) => resolved_symbol_value = addr,
//                 None => {
//                     must_backpatch = true;
//                 },
//             }
//         },
//         _ => return Err(format!("`bgt` expected a label, got {:?}", label)),
//     }

//     // what do.
//     // let upper = LineComponent::Immediate((resolved_symbol_value >> 16) as i32);
//     // let lower = LineComponent::Immediate((resolved_symbol_value & 0xFFFF) as i32);

//     // if must_backpatch {
//     //     environment.add_backpatch(&lui_info, &vec![rd.clone(), upper.clone()], identifier.clone(), BackpatchType::Upper);
//     //     environment.add_backpatch(&lui_info, &vec![rd.clone(), rd.clone(), lower.clone()], identifier.clone(), BackpatchType::Lower);
//     // }

//     Ok(
//         vec![
//             // lui  $rd, UPPER
//             (slt_info, vec![as_temp, rs.clone(), rt.clone()]),
//             // ori  $rd, $rd, LOWER
//             (bne_info, vec![as_temp, zero, label]),
//     ])
// }

pub(crate) fn expand_bnez(
    _environment: &mut Assembler,
    args: &Vec<LineComponent>,
) -> Result<Vec<(&'static InstructionInformation, Vec<LineComponent>)>, String> {
    if args.len() < 2 {
        return Err(format!(
            " - `bnez` expected 2 arguments, got {}",
            args.len()
        ));
    }

    let rs = args[0].clone();
    let label = args[1].clone();

    let zero: LineComponent = LineComponent::Register(String::from("$0"));

    // let add_info: &'static InstructionInformation;

    let bne_info = match INSTRUCTION_TABLE.get("bne") {
        Some(info) => info,
        None => return Err(format!(" - Failed to expand `bnez` pseudoinstruction. Its expansion was likely defined incorrectly (go use git blame on https://github.com/cameron-b63/name to find out who's at fault).")),
    };

    Ok(vec![
        // bnez    $rs, $0, label
        (bne_info, vec![rs, zero, label]),
    ])
}

pub(crate) fn expand_li(
    _environment: &mut Assembler,
    args: &Vec<LineComponent>,
) -> Result<Vec<(&'static InstructionInformation, Vec<LineComponent>)>, String> {
    if args.len() < 2 {
        return Err(format!(" - `li` expected 2 arguments, got {}", args.len()));
    }

    let rd: LineComponent = args[0].clone();
    let imm: LineComponent = args[1].clone();

    let zero: LineComponent = LineComponent::Register(String::from("$0"));

    let ori_info: &'static InstructionInformation = match INSTRUCTION_TABLE.get("ori") {
        Some(info) => info,
        None => return Err(format!(" - Failed to expand `li` pseudoinstruction. Its expansion was likely defined incorrectly (go use git blame on https://github.com/cameron-b63/name to find out who's at fault)."))
    };

    Ok(vec![
        // ori  $rd, $zero, imm
        (ori_info, vec![rd, zero, imm]),
    ])
}

pub(crate) fn expand_la(
    environment: &mut Assembler,
    args: &Vec<LineComponent>,
) -> Result<Vec<(&'static InstructionInformation, Vec<LineComponent>)>, String> {
    if args.len() < 2 {
        return Err(format!(" - `la` expected 2 arguments, got {}", args.len()));
    }

    let rd = args[0].clone();
    let label = args[1].clone();

    let symbol_ident: String = label.to_string();

    let symbol_offset: u32 = environment.get_symbol_offset(symbol_ident);

    let lui_info =  match INSTRUCTION_TABLE.get("lui") {
            Some(info) => info,
            None => return Err(format!(" - Failed to expand `la` pseudoinstruction. Its expansion was likely defined incorrectly (go use git blame on https://github.com/cameron-b63/name to find out who's at fault).")),
    };
    let ori_info = match INSTRUCTION_TABLE.get("ori") {
            Some(info) => info,
            None => return Err(format!(" - Failed to expand `la` pseudoinstruction. Its expansion was likely defined incorrectly (go use git blame on https://github.com/cameron-b63/name to find out who's at fault).")),
    };

    // Create appropriate relocation entries:
    let entries: Vec<RelocationEntry> = vec![
        RelocationEntry {
            r_offset: environment.current_address - MIPS_TEXT_START_ADDR,
            r_sym: symbol_offset as u32,
            r_type: RelocationEntryType::Hi16,
        },
        RelocationEntry {
            r_offset: environment.current_address + 4 - MIPS_TEXT_START_ADDR,
            r_sym: symbol_offset as u32,
            r_type: RelocationEntryType::Lo16,
        },
    ];

    let new_bytes: Vec<u8> = entries.iter().flat_map(|entry| entry.to_bytes()).collect();

    environment.section_dot_rel.extend(new_bytes);

    // Placeholder zeros since this will be filled in during linking.
    let null_component = LineComponent::Immediate(0i32);

    // Prepare for assembly.
    Ok(vec![
        // lui  $rd, 0
        (lui_info, vec![rd.clone(), null_component.clone()]),
        // ori  $rd, $rd, 0
        (ori_info, vec![rd.clone(), rd.clone(), null_component]),
    ])
}

pub(crate) fn expand_move(
    _environment: &mut Assembler,
    args: &Vec<LineComponent>,
) -> Result<Vec<(&'static InstructionInformation, Vec<LineComponent>)>, String> {
    if args.len() < 2 {
        return Err(format!(" - `mv` expected 2 arguments, got {}", args.len()));
    }

    let rd = args[0].clone();
    let rs = args[1].clone();

    let zero: LineComponent = LineComponent::Register(String::from("$0"));

    let add_info = match INSTRUCTION_TABLE.get("add") {
        Some(info) => info,
        None => return Err(format!(" - Failed to expand `move` pseudoinstruction. Its expansion was likely defined incorrectly (go use git blame on https://github.com/cameron-b63/name to find out who's at fault).")),
    };

    Ok(vec![
        // add  $rd, $rs, $0
        (add_info, vec![rd, rs, zero]),
    ])
}

// pub(crate) fn expand_bnez(environment: &mut Assembler, args: &Vec<LineComponent>) -> Result<Vec<(&'static InstructionInformation, Vec<LineComponent>)>, String> {

// }
