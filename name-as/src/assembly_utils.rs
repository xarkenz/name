use name_const::structs::{ArgumentType, InstructionInformation, InstructionType, LineComponent, Symbol};
use name_const::constants::REGISTERS;

pub fn assemble_instruction(info: &InstructionInformation, arguments: &Vec<LineComponent>, symbol_table: &Vec<Symbol>, current_address: &u32) -> Result<Option<u32>, String> {
    let num_of_passed_args = arguments.len();
    let num_of_specified_args = arguments.len();

    if (num_of_passed_args != num_of_specified_args && !info.has_optional) || (num_of_passed_args > num_of_specified_args) {
        return Err(format!("Improper number of arguments provided for `{}`", info.mnemonic));
    }

    match info.instruction_type {
        InstructionType::RType => {
            let mut rd: Option<String> = None;
            let mut rs: Option<String> = None;
            let mut rt: Option<String> = None;
            let mut shamt: Option<i32> = None;
            let funct: u32 = info.funct.expect("Improper implmentation of instructions (funct field undefined for R-type instr)\nIf you are a student reading this, understand this error comes entirely from the codebase of this vscode extension.");

            for(i, passed) in arguments.iter().enumerate() {
                let mut content = String::from("");
                let mut immediate = 0;
                match passed {
                    LineComponent::Register(register) => content = register.clone(),
                    LineComponent::Immediate(imm) => immediate = imm.clone(),
                    _ => {}
                }

                match info.args[i] {
                    ArgumentType::Rd => rd = Some(content.clone()),
                    ArgumentType::Rs => rs = Some(content.clone()),
                    ArgumentType::Rt => rt = Some(content.clone()),
                    ArgumentType::Immediate => shamt = Some(immediate),

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
            let mut imm: Option<i32> = None;
            let mut ident: Option<String> = None;

            for (i, passed) in arguments.iter().enumerate() {
                let mut content: String = String::from("");
                let mut immediate: i32 = 0;
                match passed {
                    LineComponent::Register(register) => content = register.clone(),
                    LineComponent::Immediate(imm) => immediate = imm.clone(),
                    LineComponent::Identifier(identifier) => content = identifier.clone(),
                    _ => {},
                }

                match info.args[i] {
                    ArgumentType::Rt => rt = Some(content.clone()),
                    ArgumentType::Rs => rs = Some(content.clone()),
                    ArgumentType::Immediate => imm = Some(immediate.clone()),
                    ArgumentType::Identifier => ident = Some(content.clone()),
                    _ => return Err(" - Improper type of arguments provided for instruction.".to_string()),
                }
            }

            if ident.is_some() {
                let target_addr: u32;
                let unwrapped_ident = ident.unwrap();
                if let Some(symbol) = symbol_table.iter().find(|symbol| symbol.identifier == unwrapped_ident){
                    target_addr = symbol.value;
                    // Translate from address to offset from this instruction's address
                    // Bit shifted twice right for extra range - instruction bytes are already aligned to 4 so bottom 2 bits are extra
                    let offset_unchecked: i32 = (target_addr as i32 - current_address .clone() as i32) >> 2;
                    imm = Some(offset_unchecked);
                    
                    if offset_unchecked as i16 as i32 != offset_unchecked {
                        return Err(" - Branch target out of range. I never thought we'd get here...".to_string());
                    } 
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
                let content;
                if let LineComponent::Identifier(ident) = passed {
                    content = ident;
                } else {
                    return Err(" - Malformed argument.".to_string())
                }
                
                match info.args[i] {
                    ArgumentType::Identifier => identifier = Some(content.clone()),
                    _ => return Err(" - Improper type of arguments provided for instruction.".to_string()),
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
            LineComponent::Register(x.to_string())
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
            LineComponent::Register(x.to_string())
    ).collect();

    assert_eq!(assemble_instruction(jal_info, &wrapped_arguments, &mock_symbol_table, &mock_current_address), Ok(Some(0x0c004020)));

    let mock_symbol_table: Vec<Symbol> = vec!();

    assert_eq!(assemble_instruction(jal_info, &wrapped_arguments, &mock_symbol_table, &mock_current_address), Ok(None));
}

fn assemble_r_type(rd: Option<String>, rs: Option<String>, rt: Option<String>, shamt: Option<i32>, funct: u32) -> Result<u32, String> {

    // I'm using these unwrap_or statements to ensure that when packing R-type instructions that don't use all 3, the fields default to 0 in the packed word.
    // The '?' operators are to ensure the proper error message is propagated up through to the assembler's 'errors' vec.
    let parsed_rd: u32 = parse_register_to_u32(&rd.unwrap_or("$0".to_string()))?;
    let parsed_rs: u32 = parse_register_to_u32(&rs.unwrap_or("$0".to_string()))?;
    let parsed_rt: u32 = parse_register_to_u32(&rt.unwrap_or("$0".to_string()))?;
    let unchecked_shamt: i32 = shamt.unwrap_or(0);

    // The opcode for all R-type instructions is 0.
    let opcode: u32 = 0;

    // Check shamt for range
    let parsed_shamt: u32 = unchecked_shamt as u32;
    if unchecked_shamt < 0 || unchecked_shamt > 31 {
        return Err("Shift amount out of range on shift instruction.".to_string());
    }

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
    let shamt = Some(0);
    let assembled_output = assemble_r_type(rd, rs, rt, shamt, 32);
    assert_eq!(assembled_output, Ok(0x012A4020));

    let assembled_err = assemble_r_type(Some("bad register".to_string()), None, None, None, 32);
    assert!(assembled_err.is_err());

    let rd = Some("$t0".to_string());
    let rs = Some("$t1".to_string());
    let shamt = Some (32);
    let assembled_shamt_err = assemble_r_type(rd, rs, None, shamt, 32);
    assert!(assembled_shamt_err.is_err());
}

fn assemble_i_type(opcode: u32, rs: Option<String>, rt: Option<String>, immediate: Option<i32>) -> Result<u32, String> {
    let parsed_rs: u32 = parse_register_to_u32(&rs.unwrap_or("$0".to_string()))?;
    let parsed_rt: u32 = parse_register_to_u32(&rt.unwrap_or("$0".to_string()))?;
    let unchecked_immediate: u32 = immediate.unwrap_or(0) as u32;

    // Check range on immediate value
    let parsed_immediate = unchecked_immediate as u16 as u32;
    if parsed_immediate != unchecked_immediate {
        return Err("Immediate out of range. Consider a psuedoinstruction that allows for 32-bit range.".to_string());
    }

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
    let immediate: Option<i32> = Some(0xBEEF);

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

pub fn pretty_print_instruction(packed: &u32){
    println!(" - 0x{:08x}", packed);
    println!(" - 0b{:032b}", packed);
    println!();
}