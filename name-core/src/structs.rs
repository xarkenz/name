use std::io::{stdin, stdout, Stdin, Stdout};

use crate::{
    elf_def::{MIPS_DATA_START_ADDR, MIPS_TEXT_START_ADDR},
    syscalls::*,
};

#[derive(Debug)]
pub struct Symbol {
    pub symbol_type: u8,
    pub identifier: String,
    pub value: u32,
    pub size: u32,
    pub visibility: Visibility,
    pub section: Section,
}

#[derive(Debug)]
pub struct Processor {
    pub pc: u32,
    pub general_purpose_registers: [u32; 32],
}

impl Default for Processor {
    fn default() -> Self {
        Self {
            pc: MIPS_TEXT_START_ADDR,
            general_purpose_registers: [0u32; 32],
        }
    }
}

#[derive(Debug, Default)]
pub struct Coprocessor0 {
    pub registers: [u32; 32],
}

#[derive(Debug)]
pub struct Memory {
    pub data: Vec<u8>,
    pub text: Vec<u8>,
    pub data_start: u32,
    pub data_end: u32,
    pub text_start: u32,
    pub text_end: u32,
}

impl Default for Memory {
    fn default() -> Self {
        Memory {
            data: Vec::new(),
            text: Vec::new(),
            data_start: MIPS_DATA_START_ADDR,
            data_end: MIPS_DATA_START_ADDR,
            text_start: MIPS_TEXT_START_ADDR,
            text_end: MIPS_TEXT_START_ADDR,
        }
    }
}

#[derive(Debug, Default)]
pub struct ProgramState {
    pub should_continue_execution: bool,
    pub cpu: Processor,
    pub cp0: Coprocessor0,
    pub memory: Memory,
}

#[derive(Debug, Clone, Copy)]
#[repr(usize)]
pub enum Register {
    Zero,
    At,
    V0,
    V1,
    A0,
    A1,
    A2,
    A3,
    T0,
    T1,
    T2,
    T3,
    T4,
    T5,
    T6,
    T7,
    S0,
    S1,
    S2,
    S3,
    S4,
    S5,
    S6,
    S7,
    T8,
    T9,
    K0,
    K1,
    Gp,
    Sp,
    Fp,
    Ra,
}

#[derive(Debug, Default)]
pub enum Visibility {
    #[default]
    Local,
    Global,
    Weak,
}

#[derive(Debug, Clone)]
pub enum Section {
    Null,
    Text,
    Data,
}

#[derive(Debug)]
pub struct LineInfo {
    pub content: String,
    pub line_number: u32,
    pub start_address: u32,
    pub end_address: u32,
}

/// Handler for outside world. Operating System interprets syscalls.
/// Still WIP will grow to include other non processor peripheries
#[derive(Debug)]
pub struct OperatingSystem {
    stdin: Stdin,
    stdout: Stdout,
}

impl OperatingSystem {
    pub fn new() -> OperatingSystem {
        OperatingSystem {
            stdin: stdin(),
            stdout: stdout(),
        }
    }

    /// Contains the logic for handling syscalls.
    /// I nvoked by the exception handler.
    pub fn handle_syscall(&mut self, program_state: &mut ProgramState) -> Result<(), String> {
        let syscall_num: usize =
            program_state.cpu.general_purpose_registers[Register::V0 as usize] as usize;

        match syscall_num {
            0x01 => sys_print_int(program_state, &mut self.stdout.lock()),
            0x04 => sys_print_string(program_state, &mut self.stdout.lock()),
            0x05 => sys_read_int(program_state, &mut self.stdin.lock()),
            0x0A => sys_exit(program_state),
            0x0B => sys_print_char(program_state, &mut self.stdout.lock()),
            0x0C => sys_read_char(program_state, &mut self.stdin.lock()),
            _ => Err(format!("{} is not a recognized syscall.", syscall_num)),
        }
    }
}
