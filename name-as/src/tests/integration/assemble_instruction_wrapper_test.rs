use name_const::structs::Symbol;

use crate::assembler::assemble_instruction::assemble_instruction;
use crate::definitions::structs::LineComponent;

#[test]
// Test ensures an arbitrary r-type instruction is correctly detected and packed by the assembler.
fn r_type_assemble() {
    let instruction_table = crate::assembler::assembly_helpers::generate_instruction_hashmap();
    let add_info = instruction_table.get("add").unwrap();

    let arguments: Vec<&'static str> = vec!["$t0", "$t1", "$t2"];
    let wrapped_arguments: Vec<LineComponent> = arguments.into_iter().map(|x| 
            LineComponent::Register(x.to_string())
    ).collect();

    let mock_symbol_table: Vec<Symbol> = vec![
        Symbol {symbol_type: 2, identifier:"test".to_string(),value:0x004020, size: 4, visibility: name_const::structs::Visibility::Local, section: name_const::structs::Section::Text }
    ]; 

    let mock_current_address = name_const::elf_def::MIPS_TEXT_START_ADDR;

    assert_eq!(assemble_instruction(add_info, &wrapped_arguments, &mock_symbol_table, &mock_current_address), Ok(Some(0x012A4020)));
}

#[test]
// Test ensures an arbitrary i-type instruction is detected and packed by assembler.
fn i_type_assemble() {
    let instruction_table = crate::assembler::assembly_helpers::generate_instruction_hashmap();
    let ori_info = instruction_table.get("ori").unwrap();

    let arguments: Vec<LineComponent> = vec![
        LineComponent::Register(String::from("$t1")),
        LineComponent::Register(String::from("$t2")),
        LineComponent::Immediate(42),
    ];

    let mock_symbol_table = vec!();

    let mock_current_address = name_const::elf_def::MIPS_TEXT_START_ADDR;

    assert_eq!(assemble_instruction(ori_info, &arguments, &mock_symbol_table, &mock_current_address), Ok(Some(0x3549002a)));
}

#[test]
// Test ensures a j-type instruction without forward referencing is correctly packed.
fn good_j_type_assemble() {
        let instruction_table = crate::assembler::assembly_helpers::generate_instruction_hashmap();
        // J-Type test
        let jal_info = instruction_table.get("jal").unwrap();
        let arguments: Vec<&'static str> = vec!["test"];
        let wrapped_arguments: Vec<LineComponent> = arguments.into_iter().map(|x| 
                LineComponent::Identifier(x.to_string())
        ).collect();

        let mock_symbol_table: Vec<Symbol> = vec![
            Symbol {symbol_type: 2, identifier:"test".to_string(),value:0x400020, size: 4, visibility: name_const::structs::Visibility::Local, section: name_const::structs::Section::Text }
        ]; 
    
        let mock_current_address = name_const::elf_def::MIPS_TEXT_START_ADDR;
    
        assert_eq!(assemble_instruction(jal_info, &wrapped_arguments, &mock_symbol_table, &mock_current_address), Ok(Some(0x0c100008)));
}

#[test]
// Test ensures a j-type instruction with a forward reference correctly signals a forward reference was detected.
fn forward_reference_detection_j_type() {
    let instruction_table = crate::assembler::assembly_helpers::generate_instruction_hashmap();
    // J-Type test
    let jal_info = instruction_table.get("jal").unwrap();
    let arguments: Vec<&'static str> = vec!["test"];
    let wrapped_arguments: Vec<LineComponent> = arguments.into_iter().map(|x| 
            LineComponent::Identifier(x.to_string())
    ).collect();

    let mock_current_address = name_const::elf_def::MIPS_TEXT_START_ADDR;

    let mock_symbol_table: Vec<Symbol> = vec!();
    
    assert_eq!(assemble_instruction(jal_info, &wrapped_arguments, &mock_symbol_table, &mock_current_address), Ok(None));
}