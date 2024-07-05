use name_const::structs::{ArgumentType, InstructionInformation, InstructionType, LineComponent, Symbol};
use name_const::constants::REGISTERS;

pub fn assemble_instruction(info: &InstructionInformation, arguments: &Vec<LineComponent>, symbol_table: &Vec<Symbol>, current_address: &u32) -> Result<Option<u32>, String> {
    let num_of_specified_args = info.args.len();
    if arguments.len() != num_of_specified_args {
        return Err("Improper number of arguments provided for instruction.".to_string());
    }

    match info.instruction_type {
        InstructionType::RType => {
            let mut rd: Option<String> = None;
            let mut rs: Option<String> = None;
            let mut rt: Option<String> = None;
            let mut shamt: Option<String> = None;
            let funct: u32 = info.funct.expect("Improper implmentation of instructions (funct field undefined for R-type instr)\nIf you are a student reading this, understand this error comes entirely from the codebase of this vscode extension.");

            for(i, passed) in arguments.iter().enumerate() {
                match info.args[i] {
                    ArgumentType::Rd => rd = Some(passed.content.clone()),
                    ArgumentType::Rs => rs = Some(passed.content.clone()),
                    ArgumentType::Rt => rt = Some(passed.content.clone()),
                    ArgumentType::Immediate => shamt = Some(passed.content.clone()),

                    _ => return Err("Improper type of arguments provided for instruction.".to_string()),
                }
            }

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
            let mut rt: Option<String> = None;
            let mut rs: Option<String> = None;
            let mut imm: Option<String> = None;
            let mut ident: Option<String> = None;

            for (i, passed) in arguments.iter().enumerate() {
                match info.args[i] {
                    ArgumentType::Rt => rt = Some(passed.content.clone()),
                    ArgumentType::Rs => rs = Some(passed.content.clone()),
                    ArgumentType::Immediate => imm = Some(passed.content.clone()),
                    ArgumentType::Identifier => ident = Some(passed.content.clone()),
                    _ => return Err("Improper type of arguments provided for instruction.".to_string()),
                }
            }

            if ident.is_some() {
                let target_addr: u32;
                let unwrapped_ident = ident.unwrap();
                if let Some(symbol) = symbol_table.iter().find(|symbol| symbol.identifier == unwrapped_ident){
                    target_addr = symbol.value;
                    // Translate from address to offset from this instruction's address
                    let offset: i16 = (current_address - target_addr) as i16;
                    imm = Some((offset as u16).to_string());
                } else {
                    return Ok(None);
                }
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
            let mut identifier: Option<String> = None;

            for (i, passed) in arguments.iter().enumerate() {
                match info.args[i] {
                    ArgumentType::Identifier => identifier = Some(passed.content.clone()),
                    _ => return Err("Improper type of arguments provided for instruction.".to_string()),
                }
            }
            let mut address: Option<u32> = None;

            if let Some(ident) = identifier {
                let lookup_result = symbol_table.iter().find(|symbol| symbol.identifier == ident);
                match lookup_result {
                    Some(symbol) => address = Some(symbol.value),
                    None => address = None,
                }
            }

            let assembled_output = assemble_j_type(opcode, address);

            match assembled_output {
                Ok(packed_instr) => {
                    match packed_instr {
                        Some(packed_value) => {
                            return Ok(Some(packed_value));
                        },
                        None => {
                            return Ok(None);
                        }
                    }
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
            LineComponent {
                component_type: name_const::structs::ComponentType::Register,
                content: x.to_string(),
            }
    ).collect();

    let mock_symbol_table: Vec<Symbol> = vec![
        Symbol { symbol_type: name_const::structs::SymbolType::Address, identifier: "test".to_string(), value: 0x004020 }
    ]; 

    let mock_current_address = name_const::elf_utils::MIPS_TEXT_START_ADDR;

    assert_eq!(assemble_instruction(add_info, &wrapped_arguments, &mock_symbol_table, &mock_current_address), Ok(Some(0x012A4020)));
    
    // J-Type test
    let jal_info = instruction_table.get(&"jal".to_string()).unwrap();
    let arguments: Vec<&'static str> = vec!["test"];
    let wrapped_arguments: Vec<LineComponent> = arguments.into_iter().map(|x| 
            LineComponent {
                component_type: name_const::structs::ComponentType::Register,
                content: x.to_string(),
            }
    ).collect();

    assert_eq!(assemble_instruction(jal_info, &wrapped_arguments, &mock_symbol_table, &mock_current_address), Ok(Some(0x0c004020)));

    let mock_symbol_table: Vec<Symbol> = vec!();

    assert_eq!(assemble_instruction(jal_info, &wrapped_arguments, &mock_symbol_table, &mock_current_address), Ok(None));
}

fn assemble_r_type(rd: Option<String>, rs: Option<String>, rt: Option<String>, shamt: Option<String>, funct: u32) -> Result<u32, String> {

    // I'm using these unwrap_or statements to ensure that when packing R-type instructions that don't use all 3, the fields default to 0 in the packed word.
    // The '?' operators are to ensure the proper error message is propagated up through to the assembler's 'errors' vec.
    let parsed_rd: u32 = parse_register_to_u32(&rd.unwrap_or("$0".to_string()))?;
    let parsed_rs: u32 = parse_register_to_u32(&rs.unwrap_or("$0".to_string()))?;
    let parsed_rt: u32 = parse_register_to_u32(&rt.unwrap_or("$0".to_string()))?;

    let parsed_shamt: u32;
    match shamt {
        Some(shift_amount) => {
            if let Ok(parsed_shift_amount) = shift_amount.parse::<u32>(){
                if parsed_shift_amount < 32 {
                    parsed_shamt = parsed_shift_amount;
                } else {
                    return Err("Shift amount out of bounds (>31)".to_string());
                }
            } else {
                return Err("Shift amount parsing failed.".to_string());
            }

        },
        None => {
            parsed_shamt = 0;
        },
    }

    // The opcode for all R-type instructions is 0.
    let opcode: u32 = 0;

    return Ok(
        (opcode << 26) |
        (parsed_rs << 21) |
        (parsed_rt << 16) | 
        (parsed_rd << 11) |
        (parsed_shamt << 6) | 
        funct 
    );

}

#[test]
fn assemble_r_type_test() {
    let rd = Some("$t0".to_string());
    let rs = Some("$t1".to_string());
    let rt = Some("$t2".to_string());
    let shamt = Some("0".to_string());
    let assembled_output = assemble_r_type(rd, rs, rt, shamt, 32);
    assert_eq!(assembled_output, Ok(0x012A4020));

    let assembled_err = assemble_r_type(Some("bad register".to_string()), None, None, None, 32);
    assert!(assembled_err.is_err());

    let rd = Some("$t0".to_string());
    let rs = Some("$t1".to_string());
    let shamt = Some ("32".to_string());
    let assembled_shamt_err = assemble_r_type(rd, rs, None, shamt, 32);
    assert!(assembled_shamt_err.is_err());
}

fn assemble_i_type(opcode: u32, rs: Option<String>, rt: Option<String>, immediate: Option<String>) -> Result<u32, String> {
    let parsed_rs: u32 = parse_register_to_u32(&rs.unwrap_or("$0".to_string()))?;
    let parsed_rt: u32 = parse_register_to_u32(&rt.unwrap_or("$0".to_string()))?;
    let parsed_immediate: u32 = base_parse(&immediate.unwrap_or("0".to_string()))?;

    if parsed_immediate > 0x10000 {
        return Err("Immediate exceeds 16 bits".to_string());
    }

    Ok(
        (opcode << 26) |
        (parsed_rs << 21) |
        (parsed_rt << 16) |
        (parsed_immediate)
    )
}

#[test]
fn assemble_i_type_test() {
    let opcode: u32 = 13;
    let rt: Option<String> = Some("$t0".to_string());
    let rs: Option<String> = Some("$t2".to_string());
    let immediate: Option<String> = Some("0xBEEF".to_string());

    let assembled_output = assemble_i_type(opcode, rs, rt, immediate);
    assert_eq!(assembled_output, Ok(0x3548BEEF));
}

fn assemble_j_type(opcode: u32, target: Option<u32>) -> Result<Option<u32>, String> {
    let address: u32;
    
    match target {
        Some(addr) => {
            if addr & 0xFC000000 == 0 {
                address = addr;
            } else {
                return Err("Target address out of range for J-Type instruction.".to_string());
            }
        },
        None => {
            return Ok(None);
        }
    }

    Ok(Some(
        (opcode << 26) |
        (address)
    ))
}

#[test]
fn assemble_j_type_test() {
    let opcode: u32 = 3;
    let target: u32 = 0x40BEE0;

    let assembled_output = assemble_j_type(opcode, Some(target));
    assert_eq!(assembled_output, Ok(Some(0x0c40BEE0)));
}

fn parse_register_to_u32(register: &String) -> Result<u32, String> {
    // Check the early exit
    if !register.starts_with("$") { 
        return Err("Register parse failed.".to_string());
    }

    // First, try a simple lookup on the REGISTERS constant.
    if let Some(index) = REGISTERS.iter().position(|&x| x == register){
        return Ok(index as u32);
    } else if let Ok(attempted_direct_parse) = register.chars().skip(1).collect::<String>().parse::<u32>(){
        // This line looks like wizard stuff but really I'm just removing the first char from the string by
        // using an iterator, skipping an item, and collecting everything else back together
        // This is for registers given like '$0' and '$3'
        return Ok(attempted_direct_parse);
    } else {
        return Err("Register parse failed".to_string());
    }
}

// Parses literals in hex, bin, oct, and decimal.
pub fn base_parse(input: &str) -> Result<u32, &'static str> {
    if input.starts_with("0x") {
        // Hexadecimal
        u32::from_str_radix(&input[2..], 16).map_err(|_| "Failed to parse as hexadecimal")
    } else if input.starts_with("0b") {
        // Binary
        u32::from_str_radix(&input[2..], 2).map_err(|_| "Failed to parse as binary")
    } else if input.starts_with('0') && input.len() > 1 {
        // Octal
        u32::from_str_radix(&input[1..], 8).map_err(|_| "Failed to parse as octal")
    } else {
        // Decimal
        input
            .parse::<u32>()
            .map_err(|_| "Failed to parse as decimal")
    }
}

pub fn pretty_print_instruction(packed: &u32){
    println!(" - 0x{:08x}", packed);
    println!(" - 0b{:032b}", packed);
}