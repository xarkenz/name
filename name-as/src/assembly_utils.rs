use name_const::structs::{ArgumentType, InstructionInformation, InstructionType, LineComponent, Symbol};
use name_const::constants::REGISTERS;

pub fn assemble_instruction(info: &InstructionInformation, arguments: Vec<LineComponent>, _symbol_table: &Vec<Symbol>) -> Result<u32, String> {
    let num_of_specified_args = info.args.len();
    if arguments.len() != num_of_specified_args {
        return Err("Improper number of arguments provided for instruction.")?;
    }

    let mut rd = None;
    let mut rs = None;
    let mut rt = None;
    let mut _identifier = None;
    let mut _immediate = None;

    for(i, passed) in arguments.iter().enumerate() {
        match info.args[i] {
            ArgumentType::Rd => rd = Some(passed.content.clone()),
            ArgumentType::Rs => rs = Some(passed.content.clone()),
            ArgumentType::Rt => rt = Some(passed.content.clone()),
            ArgumentType::Identifier => _identifier = Some(passed.content.clone()),
            ArgumentType::Immediate => _immediate = Some(passed.content.clone()),
        }
    }

    match info.instruction_type {
        InstructionType::RType => {
            return assemble_r_type(rd, rs, rt, info.shamt, info.funct);
        },
        InstructionType::IType => {},
        InstructionType::JType => {},
    }

    Ok(0xDEADBEEF)
}

fn assemble_r_type(rd: Option<String>, rs: Option<String>, rt: Option<String>, shamt: u32, funct: u32) -> Result<u32, String> {
    let parsed_rd: u32 = parse_register_to_u32(&rd.unwrap_or("$0".to_string()))?;
    let parsed_rs: u32 = parse_register_to_u32(&rs.unwrap_or("$0".to_string()))?;
    let parsed_rt: u32 = parse_register_to_u32(&rt.unwrap_or("$0".to_string()))?;

    // The opcode is 0 for all R-type instructions.
    let opcode: u32 = 0;

    return Ok(
        (opcode << 26) |
        (parsed_rd << 21) |
        (parsed_rs << 16) | 
        (parsed_rt << 11) |
        (shamt << 6) | 
        funct 
    );

}

fn parse_register_to_u32(register: &String) -> Result<u32, String> {
    // First, try a simple lookup on the REGISTERS constant.
    if let Some(index) = REGISTERS.iter().position(|&x| x == register){
        return Ok(index as u32);
    } else if let Ok(attempted_direct_parse) = register.chars().skip(1).collect::<String>().parse::<u32>(){
        // This line looks like wizard stuff but really I'm just removing the first char from the string by
        // using an iterator, skipping an item, and collecting everything else back together
        // This is for registers given like '$0' and '$3'
        return Ok(attempted_direct_parse);
    } else {
        return Err("Register parse failed")?;
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
    println!("0x{:08x}", packed);
    println!("0b{:032b}", packed);
}