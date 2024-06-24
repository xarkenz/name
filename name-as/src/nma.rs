/// NAME Mips Assembler
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::str;
use pest::Parser;

use name_const::lineinfo::LineInfo;

use crate::args::Args;
use crate::parser::*;
use crate::elf_utils;

// This is temporary behavior for testing purposes. Uncomment the logic to receive a test object file in /tmp.
fn test_elf_writing() -> Result <(), String> {
    /*
    let text_section: Vec<u8> = vec![b'h', b'i', b' ', b'\0'];
    let data_section: Vec<u8> = vec![b'm', b'o', b'm', b' ', b'\0'];
    let debug_section: Vec<u8> = vec![b'h', b'o', b'p', b'e', b' ', b'y', b'o', b'u', b'\'', b'r', b'e', b' ', b'\0'];
    let line_section: Vec<u8> = vec![b'w', b'e', b'l', b'l', b'\0'];

    let new_rel = create_new_et_rel(text_section, data_section, debug_section, line_section);
    write_et_rel_to_file("/tmp/test1.o", &new_rel)?;
    */
    Ok(())
}

// Declare consts for keeping track of section
const NUM_OF_SECTIONS: usize = 3;
const NULL_SECTION: usize = 0;
const DOT_TEXT: usize = 1;
const DOT_DATA: usize = 2;

pub fn assemble(program_arguments: &Args) -> Result<(), String> {
    let _ = test_elf_writing();

    // IO Setup
    let input_fn = &program_arguments.input_as;
    let output_fn = &program_arguments.output_as;

    // Read input
    let unparsed_file: String = match fs::read_to_string(input_fn) {
        Ok(v) => v,
        Err(_) => return Err("Failed to read input file contents".to_string()),
    };

    // Parse input file (check validity)
    let file_contents = match MipsParser::parse(Rule::file, &unparsed_file) {
        Ok(v) => v,
        Err(_) => return Err("Invalid assembly source file (discovered before assembly had begun)".to_string()),
    };

    // Prepare for parsing input line by line
    let mut line_number: u32 = 1;
    let mut current_address: u32 = 0;

    let mut current_section: usize = NULL_SECTION;
    let mut section_dot_text_bytes: Vec<u8> = vec!();
    let mut section_dot_data_bytes: Vec<u8> = vec!();
    let mut section_dot_debug_bytes: Vec<u8> = vec!();
    let mut section_dot_line_bytes: Vec<u8> = vec!();

    let mut lineinfo: Vec<LineInfo> = vec![];
    let mut labels: HashMap<&str, u32> = HashMap::new();

    // Setup encountered sections tracker
    let mut encountered_section: [bool; NUM_OF_SECTIONS] = [false; NUM_OF_SECTIONS];
    encountered_section[NULL_SECTION] = true;

    // Parse input line by line
    for line in file_contents {
        // Determine type of line - instructions, comments, directives, etc.
        // Instructions, then labels, then keywords, then directives, then comments, then blank lines. Common case fast
        match line.as_rule() {
            Rule::instruction => {
                // Handle instruction
                if current_section != DOT_TEXT {
                    return Err(format!("Instruction found outside section .text on line {line_number}"));
                } else {
                    // Save line content
                    let line_content: &str = line.clone().as_str();
                    // Assemble instruction and append it to .text vector
                    let mut inner: pest::iterators::Pairs<Rule> = line.into_inner();
                    let opcode: &str = inner.next().unwrap().as_str();
                    let args: Vec<&str> = inner.clone().map(|p| p.as_str()).collect::<Vec<&str>>();
                    let instr: MipsCST = MipsCST::Instruction(opcode, args);
                    if let Ok(instr_as_word) = assemble_instruction(instr, &labels, current_address as u32){
                        section_dot_text_bytes.extend(instr_as_word.to_be_bytes());
                    } else {
                        return Err(format!("Bad instruction on line {line_number}"));
                    };

                    // Append to lineinfo
                    lineinfo.push(
                        LineInfo{
                            instr_addr: current_address,
                            line_number: line_number,
                            line_contents: format!("{line_content}"),
                            psuedo_op: format!(""),
                        }
                    )
                }
            },
            Rule::label => {
                // Handle label
                match current_section {
                    DOT_TEXT => {
                        // Add text label (requires struct not yet impl)
                        let label_str = line.into_inner().next().unwrap().as_str();
                        println!("Inserting label {} at {:x}", label_str, current_address);
                        labels.insert(label_str, current_address);
                    },
                    DOT_DATA => {
                        // Add data label (requires struct not yet impl)
                        let label_str = line.into_inner().next().unwrap().as_str();
                        println!("Inserting label {} at {:x}", label_str, current_address);
                        labels.insert(label_str, current_address);
                    }
                    _ => {
                        return Err(format!("Bad section on line {line_number}"));
                    }
                }
            },
            Rule::keyword => {
                // Handle keyword
                if current_section != DOT_TEXT {
                    return Err(format!("Instruction found outside section .text on line {line_number}"));
                } else {
                    // Save line content
                    let line_content: &str = line.clone().as_str();
                    // Update line info
                    lineinfo.push(LineInfo {
                        instr_addr: current_address,
                        line_number: line_number,
                        line_contents: format!("{line_content}"),
                        psuedo_op: "".to_string(),
                    });

                    // the only keyword i can think of is syscall, which is just an r-type instruction.
                    let keyword = line.into_inner().next().unwrap().as_str();
                    if let Ok(instr_info) = r_operation(keyword) {
                        println!("-----------------------------------");
                        println!("Assembling instruction: {}", keyword);
                        match assemble_r_keyword(instr_info){
                            Ok(assembled_r) => {
                                section_dot_text_bytes.extend(assembled_r.to_be_bytes());
                            }
                            Err(e) => return Err(e.to_string()),
                        }
                    } else {
                        return Err("Failed to match keyword".to_string());
                    }
                }
            },
            Rule::directive => {
                // Handle directive

                // New plan: Pass the data section and text section bytes to the directive handler and let it do its thing on its own.
                // Also hell, let it borrow the encountered_section table. Don't care!
                // Make the directive handler return the current section we're in to update current_section. Check state before and after to update current_address.
                let given_directive = line.into_inner().next().unwrap().as_str();
                // Check for invalid directive (error from directive handler)
                if let Err(_check_good_directive) = follow_directive(given_directive, &mut encountered_section, &mut current_address, &mut current_section, &mut section_dot_data_bytes) {
                    return Err(format!("Invalid directive on line {line_number}"));
                }
            },
            Rule::comment => {
                // Just add content to debug
                section_dot_debug_bytes.extend_from_slice(&line_number.to_be_bytes());  // Ensure emulator expects a 4-byte word for line number
                section_dot_debug_bytes.extend_from_slice((line.as_str()).as_bytes());  // Followed by null-term string
                section_dot_debug_bytes.extend_from_slice(b"\0");
            },
            Rule::WHITESPACE => {
                // Do nothing, just increment line number
            },
            _ => {
                return Err(format!("Malformed line: {line_number}"));
            }
        }
        line_number += 1;
        current_address += elf_utils::MIPS_ADDRESS_ALIGNMENT;
    }

    // Collect and serialize line info
    for line in lineinfo {
        section_dot_line_bytes.extend(line.to_bytes());
    }

    let relocatable = elf_utils::create_new_et_rel(section_dot_text_bytes, section_dot_data_bytes, section_dot_debug_bytes, section_dot_line_bytes);
    elf_utils::write_et_rel_to_file(output_fn, &relocatable)?;

    Ok(())
}

fn mask_u8(n: u8, x: u8) -> Result<u8, &'static str> {
    let out = n & ((1 << x) - 1);
    if out != n {
        Err("Masking error")
    } else {
        Ok(out)
    }
}

fn mask_u32(n: u32, x: u8) -> Result<u32, &'static str> {
    let out = n & ((1 << x) - 1);
    if out != n {
        Err("Masking error")
    } else {
        Ok(out)
    }
}

// Parses literals in hex, bin, oct, and decimal.
fn base_parse(input: &String) -> Result<u32, &'static str> {
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

/// The form of an R-type instruction, specificially
/// which arguments it expects in which order
enum RForm {
    RdRsRt,
    RdRtShamt,
}

/// The variable components of an R-type instruction
pub struct R {
    shamt: u8,
    funct: u8,
    form: RForm,
}

/// The form of an I-type instruction, specifically
/// which arguments it expects in which order
enum IForm {
    RtImm,
    RtImmRs,
    RtRsImm,
    RsRtLabel,
}

/// The variable components of an I-type instruction
pub struct I {
    opcode: u8,
    form: IForm,
}

/// The variable component of a J-type instruction
pub struct J {
    opcode: u8,
}

/// Parses an R-type instruction mnemonic into an [R]
pub fn r_operation(mnemonic: &str) -> Result<R, &'static str> {
    match mnemonic {
        "sll" => Ok(R {
            shamt: 0,
            funct: 0x00,
            form: RForm::RdRtShamt,
        }),
        "srl" => Ok(R {
            shamt: 0,
            funct: 0x02,
            form: RForm::RdRtShamt,
        }),
        "syscall" => Ok(R {
            shamt: 0,
            funct: 0x0c,
            form: RForm::RdRtShamt,
        }),
        "add" => Ok(R {
            shamt: 0,
            funct: 0x20,
            form: RForm::RdRsRt,
        }),
        "sub" => Ok(R {
            shamt: 0,
            funct: 0x22,
            form: RForm::RdRsRt,
        }),
        "xor" => Ok(R {
            shamt: 0,
            funct: 0x26,
            form: RForm::RdRsRt,
        }),
        _ => Err("Failed to match R-instr mnemonic"),
    }
}

/// Parses an I-type instruction mnemonic into an [I]
pub fn i_operation(mnemonic: &str) -> Result<I, &'static str> {
    match mnemonic {
        "beq" => Ok(I {
            opcode: 0x4,
            form: IForm::RsRtLabel,
        }),
        "bne" => Ok(I {
            opcode: 0x5,
            form: IForm::RsRtLabel,
        }),
        "ori" => Ok(I {
            opcode: 0x0d,
            form: IForm::RtRsImm,
        }),
        "lui" => Ok(I {
            opcode: 0xf,
            form: IForm::RtImm,
        }),
        "lb" => Ok(I {
            opcode: 0x20,
            form: IForm::RtImmRs,
        }),
        "lh" => Ok(I {
            opcode: 0x21,
            form: IForm::RtImmRs,
        }),
        "lw" => Ok(I {
            opcode: 0x23,
            form: IForm::RtImmRs,
        }),
        "lbu" => Ok(I {
            opcode: 0x24,
            form: IForm::RtImmRs,
        }),
        "lhu" => Ok(I {
            opcode: 0x25,
            form: IForm::RtImmRs,
        }),
        "sb" => Ok(I {
            opcode: 0x28,
            form: IForm::RtImmRs,
        }),
        "sh" => Ok(I {
            opcode: 0x29,
            form: IForm::RtImmRs,
        }),
        "sw" => Ok(I {
            opcode: 0x2b,
            form: IForm::RtImmRs,
        }),
        "ll" => Ok(I {
            opcode: 0x30,
            form: IForm::RtImmRs,
        }),
        "sc" => Ok(I {
            opcode: 0x38,
            form: IForm::RtImmRs,
        }),
        _ => Err("Failed to match I-instr mnemonic"),
    }
}

/// Parses a J-type instruction mnemonic into a [J]
fn j_operation(mnemonic: &str) -> Result<J, &'static str> {
    match mnemonic {
        "j" => Ok(J { opcode: 0x2 }),
        "jal" => Ok(J { opcode: 0x3 }),
        _ => Err("Failed to match J-instr mnemonic"),
    }
}

/// Write a u32 into a file, zero-padded to 32 bits (4 bytes)
pub fn write_u32(mut file: &File, data: u32) -> std::io::Result<()> {
    fn convert_endianness(input: u32) -> u32 {
        ((input & 0x000000FF) << 24)
            | ((input & 0x0000FF00) << 8)
            | ((input & 0x00FF0000) >> 8)
            | ((input & 0xFF000000) >> 24)
    }

    const PADDED_LENGTH: usize = 4;

    // Create a 4-length buffer
    let mut padded_buffer: [u8; PADDED_LENGTH] = [0; PADDED_LENGTH];

    // Convert data into bytes
    let bytes: [u8; PADDED_LENGTH] = (convert_endianness(data)).to_be_bytes();

    // Copy bytes into buffer at offset s.t. value is left-padded with 0s
    let copy_index = PADDED_LENGTH - bytes.len();
    padded_buffer[copy_index..].copy_from_slice(&bytes);

    // Write to file
    file.write_all(&padded_buffer)
}

/// Converts a numbered mnemonic ($t0, $s8, etc) or literal (55, 67, etc) to its integer representation
fn reg_number(mnemonic: &String) -> Result<u8, &'static str> {
    if mnemonic.len() != 3 {
        return Err("Mnemonic out of bounds");
    }

    match mnemonic.chars().nth(2) {
        Some(c) => match c.to_digit(10) {
            Some(digit) => {
                if digit <= 31 {
                    Ok(digit as u8)
                } else {
                    Err("Expected u8")
                }
            }
            _ => Err("Invalid register index"),
        },
        _ => Err("Malformed mnemonic"),
    }
}

/// Given a register or number, assemble it into its integer representation
/// Look, I'm sure this works, but it's not exactly good. Using a hashmap is better.
fn assemble_reg(mnemonic: &String) -> Result<u8, &'static str> {
    // match on everything after $
    match &mnemonic[1..] {
        "zero" => Ok(0),
        "at" => Ok(1),
        "gp" => Ok(28),
        "sp" => Ok(29),
        "fp" => Ok(30),
        "ra" => Ok(31),
        _ => {
            let n = reg_number(&mnemonic)?;
            let reg = match mnemonic.chars().nth(1) {
                Some('v') => n + 2,
                Some('a') => n + 4,
                Some('t') => {
                    if n <= 7 {
                        n + 8
                    } else {
                        // t8, t9 = 24, 25
                        // 24 - 8 + n
                        n + 16
                    }
                }
                Some('s') => n + 16,
                _ => {
                    // Catch registers like $0
                    mnemonic.parse::<u8>().unwrap_or(99)
                }
            };
            if reg <= 31 {
                Ok(reg)
            } else {
                Err("Register out of bounds")
            }
        }
    }
}

/// Enforce a specific length for a given vector
fn enforce_length(arr: &Vec<String>, len: usize) -> Result<u32, &'static str> {
    if arr.len() != len {
        Err("Failed length enforcement")
    } else {
        Ok(0)
    }
}

/// Assembles an R-type instruction
fn assemble_r(r_struct: R, r_args: &Vec<String>) -> Result<u32, &'static str> {
    let mut rs: u8;
    let mut rt: u8;
    let mut rd: u8;
    let mut shamt: u8;

    match r_struct.form {
        RForm::RdRsRt => {
            enforce_length(&r_args, 3)?;
            rd = assemble_reg(&r_args[0])?;
            rs = assemble_reg(&r_args[1])?;
            rt = assemble_reg(&r_args[2])?;
            shamt = r_struct.shamt;
        }
        RForm::RdRtShamt => {
            enforce_length(&r_args, 3)?;
            rd = assemble_reg(&r_args[0])?;
            rs = 0;
            rt = assemble_reg(&r_args[1])?;
            shamt = match base_parse(&r_args[2]) {
                Ok(v) => v as u8,
                Err(_) => return Err("Failed to parse shamt"),
            }
        }
    };

    let mut funct = r_struct.funct;

    // Mask
    rs = mask_u8(rs, 5)?;
    rt &= mask_u8(rt, 5)?;
    rd &= mask_u8(rd, 5)?;
    shamt &= mask_u8(shamt, 5)?;
    funct &= mask_u8(funct, 6)?;

    // opcode : 31 - 26
    let mut result = 0x000000;

    // rs :     25 - 21
    println!("rs: {}", rs);
    result = (result << 6) | u32::from(rs);

    // rt :     20 - 16
    println!("rt: {}", rt);
    result = (result << 5) | u32::from(rt);

    // rd :     15 - 11
    println!("rd: {}", rd);
    result = (result << 5) | u32::from(rd);

    // shamt : 10 - 6
    println!("shamt: {}", shamt);
    result = (result << 5) | u32::from(shamt);

    // funct : 5 - 0
    println!("funct: {}", funct);
    result = (result << 6) | u32::from(funct);

    println!(
        "0x{:0shortwidth$x} {:0width$b}",
        result,
        result,
        shortwidth = 8,
        width = 32
    );
    Ok(result)
}

// Assembles an R-type instruction that takes no arguments (syscall)
fn assemble_r_keyword(r_struct: R) -> Result<u32, &'static str> {
    let result: u32;
    println!("funct: {}", r_struct.funct);
    result = r_struct.funct as u32;
    
    println!(
        "0x{:0shortwidth$x} {:0width$b}",
        result,
        result,
        shortwidth = 8,
        width = 32
    );
    Ok(result)
}

/// Assembles an I-type instruction
fn assemble_i(
    i_struct: I,
    i_args: &Vec<String>,
    labels: &HashMap<&str, u32>,
    instr_address: u32,
) -> Result<u32, &'static str> {
    let mut rs: u8;
    let mut rt: u8;
    let imm: u16;

    match i_struct.form {
        IForm::RtImm => {
            enforce_length(&i_args, 2)?;
            rs = 0;
            rt = assemble_reg(&i_args[0])?;
            imm = match base_parse(&i_args[1]) {
                Ok(v) => v as u16,
                Err(_) => return Err("Failed to parse imm"),
            }
        }
        IForm::RtImmRs => {
            // Immediate can default to 0 if not included in instructions
            // such as 'ld $t0, ($t1)'
            let i_args_catch = if i_args.len() == 2 {
                let mut c = i_args.clone();
                c.insert(1, format!("0"));
                c
            } else {
                let c = i_args.clone();
                c
            };
            enforce_length(&i_args_catch, 3)?;
            rt = assemble_reg(&i_args_catch[0])?;
            imm = match base_parse(&i_args_catch[1]) {
                Ok(v) => v as u16,
                Err(_) => return Err("Failed to parse imm"),
            };
            rs = assemble_reg(&i_args_catch[2])?;
        }
        IForm::RsRtLabel => {
            enforce_length(&i_args, 3)?;
            rs = assemble_reg(&i_args[0])?;
            rt = assemble_reg(&i_args[1])?;
            match labels.get(i_args[2].as_str()) {
                // Subtract byte width due to branch delay
                Some(v) => imm = ((*v) - instr_address - elf_utils::MIPS_ADDRESS_ALIGNMENT) as u16,
                None => return Err("Undeclared label"),
            }
        }
        IForm::RtRsImm => {
            enforce_length(&i_args, 3)?;
            rt = assemble_reg(&i_args[0])?;
            rs = assemble_reg(&i_args[1])?;
            imm = match base_parse(&i_args[2]) {
                Ok(v) => v as u16,
                Err(_) => return Err("Failed to parse imm"),
            };
        }
    };

    let mut opcode = i_struct.opcode;

    // Mask
    println!("Masking rs");
    rs = mask_u8(rs, 5)?;
    println!("Masking rt");
    rt = mask_u8(rt, 5)?;
    println!("Masking opcode");
    opcode = mask_u8(opcode, 6)?;
    // No need to mask imm, it's already a u16

    // opcode : 31 - 26
    let mut result: u32 = opcode.into();

    // rs :     25 - 21
    println!("rs: {}", rs);
    result = (result << 5) | u32::from(rs);

    // rt :     20 - 16
    println!("rt: {}", rt);
    result = (result << 5) | u32::from(rt);

    // imm :    15 - 0
    println!("imm: {}", imm);
    result = (result << 16) | u32::from(imm);

    println!(
        "0x{:0shortwidth$x} {:0width$b}",
        result,
        result,
        shortwidth = 8,
        width = 32
    );
    Ok(result)
}

/// Assembles a J-type instruction
fn assemble_j(
    j_struct: J,
    j_args: &Vec<String>,
    labels: &HashMap<&str, u32>,
) -> Result<u32, &'static str> {
    enforce_length(&j_args, 1)?;

    let jump_address: u32 = labels[j_args[0].as_str()];
    println!("Masking jump address");
    println!("Jump address original: {}", jump_address);
    let mut masked_jump_address = mask_u32(jump_address, 28)?;
    println!("Jump address masked: {}", masked_jump_address);
    if jump_address != masked_jump_address {
        return Err("Tried to assemble illegal jump address");
    }

    // Byte-align jump address
    masked_jump_address >>= 2;

    let mut opcode = j_struct.opcode;

    // Mask
    println!("Masking opcode");
    opcode = mask_u8(opcode, 6)?;
    // No need to mask imm, it's already a u16

    // opcode : 31 - 26
    let mut result: u32 = opcode.into();

    // imm :    25 - 0
    println!("imm: {}", masked_jump_address);
    result = (result << 26) | masked_jump_address;

    println!(
        "0x{:0shortwidth$x} {:0width$b}",
        result,
        result,
        shortwidth = 8,
        width = 32
    );
    Ok(result)
}

fn assemble_instruction(instr: MipsCST, labels: &HashMap<&str, u32>, current_addr: u32) -> Result<u32, String> {
    // First break instr down (should never fail)
    if let Some((mnemonic, args)) = instr.unpack_instruction() {
        // Try packing the instruction into each type, starting with R (most common)
        if let Ok(instr_info) = r_operation(&mnemonic) {
            println!("-----------------------------------");
            println!(
                "[R] {} - shamt [{:x}] - funct [{:x}] - args [{:?}]",
                mnemonic, instr_info.shamt, instr_info.funct, args
            );
            match assemble_r(instr_info, &args) {
                Ok(assembled_r) => {
                    return Ok(assembled_r);
                },
                Err(_) => return Err(format!("Malformed R-type instruction.")),
            }
        } else if let Ok(instr_info) = i_operation(&mnemonic) {
            println!("-----------------------------------");
            println!(
                "[I] {} - opcode [{:x}] - args [{:?}]",
                mnemonic, instr_info.opcode, args
            );

            match assemble_i(instr_info, &args, &labels, current_addr) {
                Ok(assembled_i) => {
                    return Ok(assembled_i);
                }
                Err(e) => return Err(e.to_string()),
            }
        } else if let Ok(instr_info) = j_operation(&mnemonic) {
            println!("-----------------------------------");
            println!(
                "[J] {} - opcode [{:x}] - args [{:?}]",
                mnemonic, instr_info.opcode, args
            );

            match assemble_j(instr_info, &args, &labels) {
                Ok(assembled_j) => {
                    return Ok(assembled_j);
                }
                Err(e) => return Err(e.to_string()),
            }
        } else {
            return Err("Failed to match instruction".to_string());
        }
    }
    else {
        return Err(format!("Failed to pack instruction"));
    };
}

fn follow_directive(directive: &str, section_tracker: &mut [bool; NUM_OF_SECTIONS], current_address: &mut u32, current_section: &mut usize, _data_section_vec: &mut Vec<u8>) -> Result<(), String> {
    match directive {
        ".text" => {
            if *current_section ==  DOT_TEXT || section_tracker[DOT_TEXT] {
                return Err(format!("Tried to declare section .text twice"));
            } else {
                *current_address = elf_utils::MIPS_TEXT_START_ADDR;
                *current_section = DOT_TEXT;
            }
        },
        ".data" => {
            if *current_section == DOT_DATA || section_tracker[DOT_DATA] {
                return Err(format!("Tried to declare section .data twice"));
            } else {
                *current_address = elf_utils::MIPS_DATA_START_ADDR;
                *current_section = DOT_TEXT;
            }
        },
        _ => {
            return Err(format!("Provided directive did not match (in directive handler)"));
        },
    }
    Ok(())
}