use std::env;
// This is the port on which vscode's DAP communicates.
const VSCODE_DAP_PORT: i32 = 63321;

#[derive(Debug)]
pub struct Args{
    pub port: i32,
    pub source_file: String,
    pub object_file: String,
    pub lineinfo_file: String,
    pub debug: bool,
}

fn help(){
    println!("USAGE: name-emu [OPTIONS] SOURCE OBJECT LINEINFO\n");
    println!("Required:");
    println!("  SOURCE        The source code file");
    println!("  OBJECT        The corresponding object file");
    println!("  LINEINFO      The corresponding lineinfo file");
    println!("Optional:");
    println!("  --debug ");
    println!("  -d            Enable debugging (for vscode DAP)");
}

pub fn parse_args() -> Result<Args, &'static str> {
    let mut args: Args = Args {
        port: VSCODE_DAP_PORT,
        source_file: String::new(),
        object_file: String::new(),
        lineinfo_file: String::new(),
        debug: false,
    };
    let args_strings: Vec<String> = env::args().collect();

    if args_strings.len() < 4 {
        help();
        return Err("Improper usage.");
    }

    let mut arg_index = 1;
    for arg in args_strings.iter().skip(1) {
        let mut parsed_option = true;
        match arg.as_str() {
            "-d" | "--debug" => args.debug = true,
            _ => parsed_option = false,
        };

        if parsed_option {
            continue;
        }

        match arg_index {
            1 => args.source_file = arg.to_string(),
            2 => args.object_file = arg.to_string(),
            3 => args.lineinfo_file = arg.to_string(),
            _ => return Err("Argument out of bounds"),
        }

        arg_index += 1;
    }

    if args.source_file == String::new() {
        return Err("Expected a source file but none provided");
    } else if args.object_file == String::new() {
        return Err("Expected an assembled object file but none provided");
    } else if args.lineinfo_file == String::new() {
        return Err("Expected a lineinfo file but none provided");
    }

    Ok(args)
}