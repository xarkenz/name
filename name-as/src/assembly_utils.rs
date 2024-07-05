use name_const::structs::{ArgumentType, InstructionInformation, InstructionType, LineComponent, Symbol};
use name_const::constants::REGISTERS;

pub fn assemble_instruction(info: &InstructionInformation, arguments: Vec<LineComponent>, _symbol_table: &Vec<Symbol>) -> Result<u32, String> {
    let num_of_specified_args = info.args.len();
    if arguments.len() != num_of_specified_args {
        return Err("Improper number of arguments provided for instruction.".to_string());
    }

    match info.instruction_type {
        InstructionType::RType => {
            let mut rd: Option<String> = None;
            let mut rs: Option<String> = None;
            let mut rt: Option<String> = None;

            for(i, passed) in arguments.iter().enumerate() {
                match info.args[i] {
                    ArgumentType::Rd => rd = Some(passed.content.clone()),
                    ArgumentType::Rs => rs = Some(passed.content.clone()),
                    ArgumentType::Rt => rt = Some(passed.content.clone()),

                    _ => return Err("Improper type of arguments provided for instruction.".to_string()),
                }
            }

            return assemble_r_type(rd, rs, rt, info.shamt, info.funct);
        },
        InstructionType::IType => todo!(),
        InstructionType::JType => todo!(),
    }
}

#[test]
fn assemble_instruction_test() {
    let instruction_table = name_const::helpers::generate_instruction_hashmap();
    let add_info = instruction_table.get(&"add".to_string()).unwrap();

    let arguments: Vec<&'static str> = vec!["$t0", "$t1", "$t2"];
    let wrapped_arguments: Vec<LineComponent> = arguments.into_iter().map(|x| 
            LineComponent {
                component_type: name_const::structs::ComponentType::Register,
                content: x.to_string(),
            }
    ).collect();

    let mock_symbol_table: Vec<Symbol> = vec!(); 

    assert_eq!(assemble_instruction(add_info, wrapped_arguments, &mock_symbol_table), Ok(0x012A4020));
}

fn assemble_r_type(rd: Option<String>, rs: Option<String>, rt: Option<String>, shamt: u32, funct: u32) -> Result<u32, String> {

    // I'm using these unwrap_or statements to ensure that when packing R-type instructions that don't use all 3, the fields default to 0 in the packed word.
    // The '?' operators are to ensure the proper error message is propagated up through to the assembler's 'errors' vec.
    let parsed_rd: u32 = parse_register_to_u32(&rd.unwrap_or("$0".to_string()))?;
    let parsed_rs: u32 = parse_register_to_u32(&rs.unwrap_or("$0".to_string()))?;
    let parsed_rt: u32 = parse_register_to_u32(&rt.unwrap_or("$0".to_string()))?;

    // The opcode for all R-type instructions is 0.
    let opcode: u32 = 0;

    return Ok(
        (opcode << 26) |
        (parsed_rs << 21) |
        (parsed_rt << 16) | 
        (parsed_rd << 11) |
        (shamt << 6) | 
        funct 
    );

}

#[test]
fn assemble_r_type_test() {
    let rd = Some("$t0".to_string());
    let rs = Some("$t1".to_string());
    let rt = Some("$t2".to_string());
    let assembled_output = assemble_r_type(rd, rs, rt, 0, 32);
    assert_eq!(assembled_output, Ok(0x012A4020));

    let assembled_err = assemble_r_type(Some("bad register".to_string()), None, None, 0, 32);
    assert!(assembled_err.is_err());

    let rd = Some("$t0".to_string());
    let rs = Some("$t1".to_string());
    let assembled_hidden_err = assemble_r_type(rd, rs, Some("t2".to_string()), 0, 32);
    assert!(assembled_hidden_err.is_err());
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
    println!();
    println!("0x{:08x}", packed);
    println!("0b{:032b}", packed);
}