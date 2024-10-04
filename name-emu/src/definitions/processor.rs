use crate::definitions::lookup_tables::SYSCALL_TABLE;
use crate::definitions::structs::ExecutionStatus;

use name_core::{
    arguments::{IArgs, JArgs, RArgs},
    instruction::Instruction,
    structs::Memory,
};

const AS_TEMP: usize = 1; // Want to access assembler temporary without remembering what register 1 is? Boy, do I have a solution for you!
const V0: usize = 2;
const RA: usize = 31;

#[derive(Debug)]
pub struct Processor {
    pub pc: u32,
    pub general_purpose_registers: [u32; 32],
}

impl Processor {
    pub fn new(entry: u32) -> Self {
        Processor {
            pc: entry,
            general_purpose_registers: [0; 32],
        }
    }

    pub fn process_instruction(
        &mut self,
        memory: &mut Memory,
        instr: Instruction,
    ) -> Result<ExecutionStatus, String> {
        match instr {
            Instruction::Add(args) => self.add(args),
            Instruction::Addi(args) => self.addi(args),
            Instruction::Addiu(args) => self.addiu(args),
            Instruction::Addu(args) => self.addu(args),
            Instruction::And(args) => self.and(args),
            Instruction::Andi(args) => self.andi(args),
            Instruction::Beq(args) => self.beq(memory, args),
            Instruction::Bgtz(args) => self.bgtz(memory, args),
            Instruction::Blez(args) => self.blez(memory, args),
            Instruction::Bne(args) => self.bne(memory, args),
            Instruction::J(jargs) => self.j(memory, jargs),
            Instruction::Jal(jargs) => self.jal(memory, jargs),
            Instruction::Jalr(jargs) => self.jalr(memory, jargs),
            Instruction::Jr(jargs) => self.jr(memory, jargs),
            Instruction::Lb(args) => self.lb(memory, args),
            Instruction::Lui(args) => self.lui(args),
            Instruction::Lw(args) => self.lw(memory, args),
            Instruction::Nor(args) => self.nor(args),
            Instruction::Nop(_args) => todo!(),
            Instruction::Or(args) => self.or(args),
            Instruction::Ori(args) => self.ori(args),
            Instruction::Sb(args) => self.sb(memory, args),
            Instruction::Sll(args) => self.sll(args),
            Instruction::Slt(args) => self.slt(args),
            Instruction::Slti(args) => self.slti(args),
            Instruction::Sltiu(args) => self.sltiu(args),
            Instruction::Sltu(args) => self.sltu(args),
            Instruction::Srl(args) => self.srl(args),
            Instruction::Sub(args) => self.sub(args),
            Instruction::Subu(args) => self.subu(args),
            Instruction::Sw(args) => self.sw(memory, args),
            Instruction::Syscall(_) => self.syscall(memory),
            Instruction::Xor(args) => self.xor(args),
            Instruction::Xori(args) => self.xori(args),
        }
    }

    // 0x00 - sll
    pub fn sll(&mut self, rargs: RArgs) -> Result<ExecutionStatus, String> {
        self.general_purpose_registers[rargs.rd] =
            self.general_purpose_registers[rargs.rt] << rargs.shamt;
        Ok(ExecutionStatus::Continue)
    }

    // 0x02 - srl
    pub fn srl(&mut self, rargs: RArgs) -> Result<ExecutionStatus, String> {
        self.general_purpose_registers[rargs.rd] =
            self.general_purpose_registers[rargs.rt] >> rargs.shamt;
        Ok(ExecutionStatus::Continue)
    }

    // 0x08 - jr
    pub fn jr(&mut self, memory: &mut Memory, rargs: RArgs) -> Result<ExecutionStatus, String> {
        if self.general_purpose_registers[rargs.rs] >= memory.text_end
            || self.general_purpose_registers[rargs.rs] < memory.text_start
        {
            return Err(format!(
                "Attempted to jump to unowned address 0x{:x}",
                self.general_purpose_registers[rargs.rs]
            ));
        }

        self.pc = self.general_purpose_registers[rargs.rs];

        Ok(ExecutionStatus::Continue)
    }

    // 0x09 - jalr
    pub fn jalr(&mut self, memory: &mut Memory, rargs: RArgs) -> Result<ExecutionStatus, String> {
        let rd = match rargs.rd {
            0 => 31,
            _ => rargs.rd,
        };

        if self.general_purpose_registers[rargs.rs] >= memory.text_end
            || self.general_purpose_registers[rargs.rs] < memory.text_start
        {
            return Err(format!(
                "Attempted to jump to unowned address 0x{:x}",
                self.general_purpose_registers[rargs.rs]
            ));
        }
        self.general_purpose_registers[rd] = self.pc;
        self.pc = self.general_purpose_registers[rargs.rs];

        Ok(ExecutionStatus::Continue)
    }

    // 0x0A - slti
    pub fn slti(&mut self, iargs: IArgs) -> Result<ExecutionStatus, String> {
        if (self.general_purpose_registers[iargs.rs] as i32) < (iargs.imm as i32) {
            self.general_purpose_registers[iargs.rt] = 1 as u32;
        } else {
            self.general_purpose_registers[iargs.rt] = 0 as u32;
        }

        Ok(ExecutionStatus::Continue)
    }

    // 0x0B - sltiu
    pub fn sltiu(&mut self, iargs: IArgs) -> Result<ExecutionStatus, String> {
        if self.general_purpose_registers[iargs.rs] < iargs.imm {
            self.general_purpose_registers[iargs.rt] = 1 as u32;
        } else {
            self.general_purpose_registers[iargs.rt] = 0 as u32;
        }

        Ok(ExecutionStatus::Continue)
    }

    // 0x0C - syscall
    pub fn syscall(&mut self, memory: &mut Memory) -> Result<ExecutionStatus, String> {
        let syscall_num: usize = self.general_purpose_registers[V0] as usize;
        match SYSCALL_TABLE[syscall_num] {
            Some(fun) => fun(self, memory),
            None => return Err(format!("Syscall {} is not implemented", syscall_num)),
        }
    }

    /// 0x20 - add
    pub fn add(&mut self, rargs: RArgs) -> Result<ExecutionStatus, String> {
        self.general_purpose_registers[rargs.rd] =
            self.general_purpose_registers[rargs.rs] + self.general_purpose_registers[rargs.rt];

        Ok(ExecutionStatus::Continue)
    }

    // 0x21 - addu
    pub fn addu(&mut self, rargs: RArgs) -> Result<ExecutionStatus, String> {
        // check that below works
        self.general_purpose_registers[rargs.rd] = self.general_purpose_registers[rargs.rs]
            .overflowing_add(self.general_purpose_registers[rargs.rt])
            .0;
        Ok(ExecutionStatus::Continue)
    }

    // 0x22 - sub
    pub fn sub(&mut self, rargs: RArgs) -> Result<ExecutionStatus, String> {
        let temp: (u32, bool) = self.general_purpose_registers[rargs.rs]
            .overflowing_sub(self.general_purpose_registers[rargs.rt]);

        self.general_purpose_registers[AS_TEMP] = temp.0;

        if temp.1 {
            // TODO: Implement coprocessor 0 and signal integer overflow

            return Err(format!("Integer underflow occurred in subtraction."));
        } else {
            self.general_purpose_registers[rargs.rd] = self.general_purpose_registers[AS_TEMP];
        }

        Ok(ExecutionStatus::Continue)
    }

    // 0x23 - subu
    pub fn subu(&mut self, rargs: RArgs) -> Result<ExecutionStatus, String> {
        let temp: (u32, bool) = self.general_purpose_registers[rargs.rs]
            .overflowing_sub(self.general_purpose_registers[rargs.rt]);

        self.general_purpose_registers[rargs.rd] = temp.0;

        Ok(ExecutionStatus::Continue)
    }

    // 0x24 - and
    pub fn and(&mut self, rargs: RArgs) -> Result<ExecutionStatus, String> {
        self.general_purpose_registers[rargs.rd] =
            self.general_purpose_registers[rargs.rs] & self.general_purpose_registers[rargs.rt];
        Ok(ExecutionStatus::Continue)
    }

    // 0x25 - or
    pub fn or(&mut self, rargs: RArgs) -> Result<ExecutionStatus, String> {
        self.general_purpose_registers[rargs.rd] =
            self.general_purpose_registers[rargs.rs] | self.general_purpose_registers[rargs.rt];
        Ok(ExecutionStatus::Continue)
    }

    // 0x26 - xor
    pub fn xor(&mut self, rargs: RArgs) -> Result<ExecutionStatus, String> {
        self.general_purpose_registers[rargs.rd] =
            self.general_purpose_registers[rargs.rs] ^ self.general_purpose_registers[rargs.rt];
        Ok(ExecutionStatus::Continue)
    }

    // 0x27 - nor
    pub fn nor(&mut self, rargs: RArgs) -> Result<ExecutionStatus, String> {
        self.general_purpose_registers[rargs.rd] =
            !(self.general_purpose_registers[rargs.rs] | self.general_purpose_registers[rargs.rt]);
        Ok(ExecutionStatus::Continue)
    }

    // 0x2A - slt
    pub fn slt(&mut self, rargs: RArgs) -> Result<ExecutionStatus, String> {
        if (self.general_purpose_registers[rargs.rs] as i32)
            < (self.general_purpose_registers[rargs.rt] as i32)
        {
            self.general_purpose_registers[rargs.rd] = 1 as u32;
        } else {
            self.general_purpose_registers[rargs.rd] = 0 as u32;
        }
        Ok(ExecutionStatus::Continue)
    }

    // 0x2A - sltu
    pub fn sltu(&mut self, rargs: RArgs) -> Result<ExecutionStatus, String> {
        if self.general_purpose_registers[rargs.rs] < self.general_purpose_registers[rargs.rt] {
            self.general_purpose_registers[rargs.rd] = 1; // check if this is kosher or if i need to do 00..001 for some reason
        } else {
            self.general_purpose_registers[rargs.rd] = 0;
        }
        Ok(ExecutionStatus::Continue)
    }

    /*

       ____  _____   _____ ____  _____  ______
      / __ \|  __ \ / ____/ __ \|  __ \|  ____|
     | |  | | |__) | |   | |  | | |  | | |__
     | |  | |  ___/| |   | |  | | |  | |  __|
     | |__| | |    | |___| |__| | |__| | |____
      \____/|_|     \_____\____/|_____/|______|



    */

    // 0x02 - j
    pub fn j(&mut self, memory: &mut Memory, jargs: JArgs) -> Result<ExecutionStatus, String> {
        let address: u32 = (jargs.imm << 2) | (self.pc & 0xF0000000);

        if address >= memory.text_end || address < memory.text_start {
            return Err(format!(
                "Attempted to jump to unowned address 0x{:x}",
                address
            ));
        }

        self.pc = address;

        Ok(ExecutionStatus::Continue)
    }

    // 0x03 - jal
    pub fn jal(&mut self, memory: &mut Memory, jargs: JArgs) -> Result<ExecutionStatus, String> {
        let address: u32 = (jargs.imm << 2) | (self.pc & 0xF0000000);

        if address >= memory.text_end || address < memory.text_start {
            return Err(format!(
                "Attempted to jump to unowned address 0x{:x}",
                address
            ));
        }

        self.general_purpose_registers[RA] = self.pc;
        self.pc = address;

        Ok(ExecutionStatus::Continue)
    }

    // 0x04 - beq
    pub fn beq(&mut self, memory: &mut Memory, iargs: IArgs) -> Result<ExecutionStatus, String> {
        // Sign extend offset
        let offset: i32 = ((iargs.imm & 0xFFFF) as i16 as i32) << 2;

        if self.general_purpose_registers[iargs.rs] != self.general_purpose_registers[iargs.rt] {
            return Ok(ExecutionStatus::Continue);
        }

        let temp = (self.pc as i32 + offset) as u32;

        if temp >= memory.text_end || temp < memory.text_start {
            return Err(format!("Attempted to access unowned address 0x{:x}", temp));
        }

        // Bro forgot the actual jump logic
        self.pc = temp;

        Ok(ExecutionStatus::Continue)
    }

    // 0x05 - bne
    pub fn bne(&mut self, memory: &mut Memory, iargs: IArgs) -> Result<ExecutionStatus, String> {
        // Sign extend offset
        let offset: i32 = ((iargs.imm & 0xFFFF) as i16 as i32) << 2;

        if self.general_purpose_registers[iargs.rs] == self.general_purpose_registers[iargs.rt] {
            return Ok(ExecutionStatus::Continue);
        }

        let temp = (self.pc as i32 + offset) as u32;

        if temp >= memory.text_end || temp < memory.text_start {
            return Err(format!("Attempted to access unowned address 0x{:x}", temp));
        }

        // Bro once again forgot the actual jump logic
        self.pc = temp;

        Ok(ExecutionStatus::Continue)
    }

    // 0x06 - blez
    pub fn blez(&mut self, memory: &mut Memory, iargs: IArgs) -> Result<ExecutionStatus, String> {
        let offset: i32 = ((iargs.imm & 0xFFFF) as i16 as i32) << 2;

        if (self.general_purpose_registers[iargs.rs] as i32) > 0 {
            return Ok(ExecutionStatus::Continue);
        }

        let temp = (self.pc as i32 + offset) as u32;

        if temp >= memory.text_end || temp < memory.text_start {
            return Err(format!("Attempted to access unowned address 0x{:x}", temp));
        }

        // BRO HAS ONCE AGAIN FORGOTTEN THE ACTUAL JUMP
        self.pc = temp;

        Ok(ExecutionStatus::Continue)
    }

    // 0x07 - bgtz
    pub fn bgtz(&mut self, memory: &mut Memory, iargs: IArgs) -> Result<ExecutionStatus, String> {
        // Sign extend offset
        let offset: i32 = (iargs.imm as i16 as i32) << 2;

        if self.general_purpose_registers[iargs.rs] as i32 <= 0 {
            return Ok(ExecutionStatus::Continue);
        }

        let temp = (self.pc as i32 + offset) as u32;

        if temp >= memory.text_end || temp < memory.text_start {
            return Err(format!("Attempted to access unowned address 0x{:x}", temp));
        }

        self.pc = temp;

        Ok(ExecutionStatus::Continue)
    }

    /// 0x08 - addi
    pub fn addi(&mut self, iargs: IArgs) -> Result<ExecutionStatus, String> {
        self.general_purpose_registers[iargs.rt] =
            (self.general_purpose_registers[iargs.rs] as i32 + (iargs.imm as i16 as i32)) as u32;
        Ok(ExecutionStatus::Continue)
    }

    // 0x09 - addiu
    pub fn addiu(&mut self, iargs: IArgs) -> Result<ExecutionStatus, String> {
        self.general_purpose_registers[iargs.rt] = self.general_purpose_registers[iargs.rs]
            .overflowing_add(iargs.imm)
            .0;
        Ok(ExecutionStatus::Continue)
    }

    // 0x0C - andi
    pub fn andi(&mut self, iargs: IArgs) -> Result<ExecutionStatus, String> {
        self.general_purpose_registers[iargs.rt] =
            self.general_purpose_registers[iargs.rs] & iargs.imm;
        Ok(ExecutionStatus::Continue)
    }

    // 0x0D - ori
    pub fn ori(&mut self, iargs: IArgs) -> Result<ExecutionStatus, String> {
        self.general_purpose_registers[iargs.rt] =
            self.general_purpose_registers[iargs.rs] | iargs.imm;
        Ok(ExecutionStatus::Continue)
    }

    // 0x0E - xori
    pub fn xori(&mut self, iargs: IArgs) -> Result<ExecutionStatus, String> {
        self.general_purpose_registers[iargs.rt] =
            self.general_purpose_registers[iargs.rs] ^ iargs.imm;
        Ok(ExecutionStatus::Continue)
    }

    // 0x0F - lui
    pub fn lui(&mut self, iargs: IArgs) -> Result<ExecutionStatus, String> {
        // SUPER DUPER PROBLEM SPOT
        self.general_purpose_registers[iargs.rt] = iargs.imm << 16;
        Ok(ExecutionStatus::Continue)
    }

    // 0x20 - lb
    pub fn lb(&mut self, memory: &mut Memory, iargs: IArgs) -> Result<ExecutionStatus, String> {
        self.general_purpose_registers[AS_TEMP] =
            (self.general_purpose_registers[iargs.rs] as i32 + iargs.imm as i32) as u32;

        if self.general_purpose_registers[AS_TEMP] >= memory.data_end
            || self.general_purpose_registers[AS_TEMP] < memory.data_start
        {
            return Err(format!(
                "Attempted to access unowned address 0x{:x}",
                self.general_purpose_registers[AS_TEMP]
            ));
        } else {
            self.general_purpose_registers[iargs.rt] = memory.data
                [(self.general_purpose_registers[AS_TEMP] - memory.data_start) as usize]
                as u32;
        }

        Ok(ExecutionStatus::Continue)
    }

    // 0x23 - lw
    pub fn lw(&mut self, memory: &mut Memory, iargs: IArgs) -> Result<ExecutionStatus, String> {
        let temp = (self.general_purpose_registers[iargs.rs] as i32 + iargs.imm as i32) as u32;

        if temp % 4 != 0 {
            return Err(format!(
                "Incorrect alignment for word access at 0x{:x}",
                temp
            ));
        }

        if temp + 4 >= memory.data_end || temp < memory.data_start {
            return Err(format!(
                "Attempted to access unowned address 0x{:x} (possible alignment issue)",
                temp
            ));
        }

        let start_idx: usize = (temp - memory.data_start) as usize;
        let end_idx: usize = (start_idx + 4) as usize;

        self.general_purpose_registers[iargs.rt] =
            u32::from_be_bytes(memory.data[start_idx..end_idx].try_into().unwrap());

        Ok(ExecutionStatus::Continue)
    }

    // 0x28 - sb
    pub fn sb(&mut self, memory: &mut Memory, iargs: IArgs) -> Result<ExecutionStatus, String> {
        let temp = (self.general_purpose_registers[iargs.rs] as i32 + iargs.imm as i32) as u32;

        if temp >= memory.data_end || temp < memory.data_start {
            return Err(format!("Attempted to access unowned address 0x{:x}", temp));
        }

        memory.data[(temp - memory.data_start) as usize] =
            self.general_purpose_registers[iargs.rt] as u8;

        Ok(ExecutionStatus::Continue)
    }

    // 0x2b - sw
    pub fn sw(&mut self, memory: &mut Memory, iargs: IArgs) -> Result<ExecutionStatus, String> {
        let temp = (self.general_purpose_registers[iargs.rs] as i32 + iargs.imm as i32) as u32;

        if temp % 4 != 0 {
            return Err(format!(
                "Incorrect alignment for word access at 0x{:x}",
                temp
            ));
        }

        if temp + 4 >= memory.data_end || temp < memory.data_start {
            return Err(format!(
                "Attempted to access unowned address 0x{:x} (possible alignment issue)",
                temp
            ));
        }

        let start_idx: usize = (temp - memory.data_start) as usize;
        let end_idx: usize = (start_idx + 4) as usize;

        memory.data.splice(
            start_idx..end_idx,
            self.general_purpose_registers[iargs.rt].to_be_bytes(),
        );

        // println!("Storing {} at 0x{:x} from ${}", self.general_purpose_registers[rt], temp, rt);

        Ok(ExecutionStatus::Continue)
    }
}
