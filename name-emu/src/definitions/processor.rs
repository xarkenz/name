use crate::definitions::lookup_tables::SYSCALL_TABLE;
use crate::definitions::structs::ExecutionStatus;

use name_core::{
    instruction::{Immediate, Instruction, Jump, Regimm, Register, Special, SpecialOp},
    operation::{ImmediateOp, RegimmOp, Special1Op, Special2Op},
    structs::Memory,
};

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
    pub fn process_immediate(
        &mut self,
        memory: &mut Memory,
        instr: Immediate,
    ) -> Result<ExecutionStatus, String> {
        match instr.opcode {
            ImmediateOp::Beq => self.beq(memory, instr),
            ImmediateOp::Bne => self.bne(memory, instr),
            ImmediateOp::Blez => self.blez(memory, instr),
            ImmediateOp::Bgtz => self.bgtz(memory, instr),
            ImmediateOp::Addi => self.addi(instr),
            ImmediateOp::Addiu => self.addiu(instr),
            ImmediateOp::Slti => self.slti(instr),
            ImmediateOp::Sltiu => self.sltiu(instr),
            ImmediateOp::Andi => self.andi(instr),
            ImmediateOp::Ori => self.ori(instr),
            ImmediateOp::Xori => self.xori(instr),
            ImmediateOp::Lui => self.lui(instr),
            ImmediateOp::Beql => todo!(),
            ImmediateOp::Bnel => todo!(),
            ImmediateOp::Blezl => todo!(),
            ImmediateOp::Bgtzl => todo!(),
            ImmediateOp::Lb => self.lb(memory, instr),
            ImmediateOp::Lh => todo!(),
            ImmediateOp::Lwl => todo!(),
            ImmediateOp::Lw => self.lw(memory, instr),
            ImmediateOp::Lbu => todo!(),
            ImmediateOp::Lhu => todo!(),
            ImmediateOp::Lwr => todo!(),
            ImmediateOp::Sb => self.sb(memory, instr),
            ImmediateOp::Sh => todo!(),
            ImmediateOp::Swl => todo!(),
            ImmediateOp::Sw => self.sw(memory, instr),
            ImmediateOp::Swr => todo!(),
            ImmediateOp::Cache => todo!(),
            ImmediateOp::Ll => todo!(),
            ImmediateOp::Lwc1 => todo!(),
            ImmediateOp::Lwc2 => todo!(),
            ImmediateOp::Pref => todo!(),
            ImmediateOp::Ldc1 => todo!(),
            ImmediateOp::Ldc2 => todo!(),
            ImmediateOp::Sc => todo!(),
            ImmediateOp::Swc1 => todo!(),
            ImmediateOp::Swc2 => todo!(),
            ImmediateOp::Sdc1 => todo!(),
            ImmediateOp::Sdc2 => todo!(),
            _ => Err("Jump operation called with IType arguments".to_string()),
        }
    }

    pub fn process_jump(
        &mut self,
        memory: &mut Memory,
        instr: Jump,
    ) -> Result<ExecutionStatus, String> {
        match instr.opcode {
            ImmediateOp::J => self.j(memory, instr),
            ImmediateOp::Jal => self.jal(memory, instr),
            ImmediateOp::Jalx => todo!(),
            _ => Err("Operation does not take jump arguments".to_string()),
        }
    }
    pub fn process_special(
        &mut self,
        memory: &mut Memory,
        instr: Special,
    ) -> Result<ExecutionStatus, String> {
        match instr.special_opcode {
            SpecialOp::Special1(special1) => match special1 {
                Special1Op::Sll => self.sll(instr),
                Special1Op::Movci => todo!(),
                Special1Op::Srl => self.srl(instr),
                Special1Op::Sra => todo!(),
                Special1Op::Sllv => todo!(),
                Special1Op::Lsa => todo!(),
                Special1Op::Srlv => todo!(),
                Special1Op::Srav => todo!(),
                Special1Op::Jr => self.jr(memory, instr),
                Special1Op::Jalr => self.jalr(memory, instr),
                Special1Op::Movz => todo!(),
                Special1Op::Movn => todo!(),
                Special1Op::Syscall => self.syscall(memory),
                Special1Op::Break => todo!(),
                Special1Op::Sdbbp => todo!(),
                Special1Op::Sync => todo!(),
                Special1Op::Mfhi => todo!(),
                Special1Op::Mthi => todo!(),
                Special1Op::Mflo => todo!(),
                Special1Op::Mtlo => todo!(),
                Special1Op::Mult => todo!(),
                Special1Op::Multu => todo!(),
                Special1Op::Div => todo!(),
                Special1Op::Divu => todo!(),
                Special1Op::Add => self.add(instr),
                Special1Op::Addu => self.addu(instr),
                Special1Op::Sub => self.sub(instr),
                Special1Op::Subu => self.subu(instr),
                Special1Op::And => self.and(instr),
                Special1Op::Or => self.or(instr),
                Special1Op::Xor => self.xor(instr),
                Special1Op::Nor => self.nor(instr),
                Special1Op::Slt => self.slt(instr),
                Special1Op::Sltu => self.sltu(instr),
                Special1Op::Tge => todo!(),
                Special1Op::Tgeu => todo!(),
                Special1Op::Tlt => todo!(),
                Special1Op::Tltu => todo!(),
                Special1Op::Teq => todo!(),
                Special1Op::Tne => todo!(),
            },
            SpecialOp::Special2(special2) => match special2 {
                Special2Op::Madd => todo!(),
                Special2Op::Maddu => todo!(),
                Special2Op::Mul => todo!(),
                Special2Op::Msub => todo!(),
                Special2Op::Msubu => todo!(),
                Special2Op::Clz => todo!(),
                Special2Op::Clo => todo!(),
                Special2Op::Sdbbp => todo!(),
            },
        }
    }
    pub fn process_regimm(
        &mut self,
        _memory: &mut Memory,
        instr: Regimm,
    ) -> Result<ExecutionStatus, String> {
        match instr.regimm_op {
            RegimmOp::Bltz => todo!(),
            RegimmOp::Bgez => todo!(),
            RegimmOp::Bltzl => todo!(),
            RegimmOp::Bgezl => todo!(),
            RegimmOp::Tgei => todo!(),
            RegimmOp::Tgeiu => todo!(),
            RegimmOp::Tlti => todo!(),
            RegimmOp::Tltiu => todo!(),
            RegimmOp::Teqi => todo!(),
            RegimmOp::Tnei => todo!(),
            RegimmOp::Bltzal => todo!(),
            RegimmOp::Bgezal => todo!(),
            RegimmOp::Bltzall => todo!(),
            RegimmOp::Bgezall => todo!(),
            RegimmOp::Synci => todo!(),
        }
    }
    pub fn process_instruction(
        &mut self,
        memory: &mut Memory,
        instr: Instruction,
    ) -> Result<ExecutionStatus, String> {
        match instr {
            Instruction::Imm(imm) => self.process_immediate(memory, imm),
            Instruction::J(jump) => self.process_jump(memory, jump),
            Instruction::Spec(spec) => self.process_special(memory, spec),
            Instruction::Regimm(regimm) => self.process_regimm(memory, regimm),
        }
    }

    // 0x00 - sll
    pub fn sll(&mut self, rargs: Special) -> Result<ExecutionStatus, String> {
        self.general_purpose_registers[rargs.rd as usize] =
            self.general_purpose_registers[rargs.rt as usize] << rargs.shamt;
        Ok(ExecutionStatus::Continue)
    }

    // 0x02 - srl
    pub fn srl(&mut self, rargs: Special) -> Result<ExecutionStatus, String> {
        self.general_purpose_registers[rargs.rd as usize] =
            self.general_purpose_registers[rargs.rt as usize] >> rargs.shamt;
        Ok(ExecutionStatus::Continue)
    }

    // 0x08 - jr
    pub fn jr(&mut self, memory: &mut Memory, rargs: Special) -> Result<ExecutionStatus, String> {
        if self.general_purpose_registers[rargs.rs as usize] >= memory.text_end
            || self.general_purpose_registers[rargs.rs as usize] < memory.text_start
        {
            return Err(format!(
                "Attempted to jump to unowned address 0x{:x}",
                self.general_purpose_registers[rargs.rs as usize]
            ));
        }

        self.pc = self.general_purpose_registers[rargs.rs as usize];

        Ok(ExecutionStatus::Continue)
    }

    // 0x09 - jalr
    pub fn jalr(&mut self, memory: &mut Memory, rargs: Special) -> Result<ExecutionStatus, String> {
        let rd = match rargs.rd as usize {
            0 => 31,
            _ => rargs.rd as usize,
        };

        if self.general_purpose_registers[rargs.rs as usize] >= memory.text_end
            || self.general_purpose_registers[rargs.rs as usize] < memory.text_start
        {
            return Err(format!(
                "Attempted to jump to unowned address 0x{:x}",
                self.general_purpose_registers[rargs.rs as usize]
            ));
        }
        self.general_purpose_registers[rd] = self.pc;
        self.pc = self.general_purpose_registers[rargs.rs as usize];

        Ok(ExecutionStatus::Continue)
    }

    // 0x0A - slti
    pub fn slti(&mut self, iargs: Immediate) -> Result<ExecutionStatus, String> {
        if (self.general_purpose_registers[iargs.rs as usize] as i32) < (iargs.imm as i32) {
            self.general_purpose_registers[iargs.rt as usize] = 1 as u32;
        } else {
            self.general_purpose_registers[iargs.rt as usize] = 0 as u32;
        }

        Ok(ExecutionStatus::Continue)
    }

    // 0x0B - sltiu
    pub fn sltiu(&mut self, iargs: Immediate) -> Result<ExecutionStatus, String> {
        if self.general_purpose_registers[iargs.rs as usize] < iargs.imm {
            self.general_purpose_registers[iargs.rt as usize] = 1 as u32;
        } else {
            self.general_purpose_registers[iargs.rt as usize] = 0 as u32;
        }

        Ok(ExecutionStatus::Continue)
    }

    // 0x0C - syscall
    pub fn syscall(&mut self, memory: &mut Memory) -> Result<ExecutionStatus, String> {
        let syscall_num: usize = self.general_purpose_registers[Register::V0 as usize] as usize;
        match SYSCALL_TABLE[syscall_num] {
            Some(fun) => fun(self, memory),
            None => return Err(format!("Syscall {} is not implemented", syscall_num)),
        }
    }

    /// 0x20 - add
    pub fn add(&mut self, rargs: Special) -> Result<ExecutionStatus, String> {
        self.general_purpose_registers[rargs.rd as usize] = self.general_purpose_registers
            [rargs.rs as usize]
            + self.general_purpose_registers[rargs.rt as usize];

        Ok(ExecutionStatus::Continue)
    }

    // 0x21 - addu
    pub fn addu(&mut self, rargs: Special) -> Result<ExecutionStatus, String> {
        // check that below works
        self.general_purpose_registers[rargs.rd as usize] = self.general_purpose_registers
            [rargs.rs as usize]
            .overflowing_add(self.general_purpose_registers[rargs.rt as usize])
            .0;
        Ok(ExecutionStatus::Continue)
    }

    // 0x22 - sub
    pub fn sub(&mut self, rargs: Special) -> Result<ExecutionStatus, String> {
        let temp: (u32, bool) = self.general_purpose_registers[rargs.rs as usize]
            .overflowing_sub(self.general_purpose_registers[rargs.rt as usize]);

        self.general_purpose_registers[Register::At as usize] = temp.0;

        if temp.1 {
            // TODO: Implement coprocessor 0 and signal integer overflow

            return Err(format!("Integer underflow occurred in subtraction."));
        } else {
            self.general_purpose_registers[rargs.rd as usize] =
                self.general_purpose_registers[Register::At as usize];
        }

        Ok(ExecutionStatus::Continue)
    }

    // 0x23 - subu
    pub fn subu(&mut self, rargs: Special) -> Result<ExecutionStatus, String> {
        let temp: (u32, bool) = self.general_purpose_registers[rargs.rs as usize]
            .overflowing_sub(self.general_purpose_registers[rargs.rt as usize]);

        self.general_purpose_registers[rargs.rd as usize] = temp.0;

        Ok(ExecutionStatus::Continue)
    }

    // 0x24 - and
    pub fn and(&mut self, rargs: Special) -> Result<ExecutionStatus, String> {
        self.general_purpose_registers[rargs.rd as usize] = self.general_purpose_registers
            [rargs.rs as usize]
            & self.general_purpose_registers[rargs.rt as usize];
        Ok(ExecutionStatus::Continue)
    }

    // 0x25 - or
    pub fn or(&mut self, rargs: Special) -> Result<ExecutionStatus, String> {
        self.general_purpose_registers[rargs.rd as usize] = self.general_purpose_registers
            [rargs.rs as usize]
            | self.general_purpose_registers[rargs.rt as usize];
        Ok(ExecutionStatus::Continue)
    }

    // 0x26 - xor
    pub fn xor(&mut self, rargs: Special) -> Result<ExecutionStatus, String> {
        self.general_purpose_registers[rargs.rd as usize] = self.general_purpose_registers
            [rargs.rs as usize]
            ^ self.general_purpose_registers[rargs.rt as usize];
        Ok(ExecutionStatus::Continue)
    }

    // 0x27 - nor
    pub fn nor(&mut self, rargs: Special) -> Result<ExecutionStatus, String> {
        self.general_purpose_registers[rargs.rd as usize] = !(self.general_purpose_registers
            [rargs.rs as usize]
            | self.general_purpose_registers[rargs.rt as usize]);
        Ok(ExecutionStatus::Continue)
    }

    // 0x2A - slt
    pub fn slt(&mut self, rargs: Special) -> Result<ExecutionStatus, String> {
        if (self.general_purpose_registers[rargs.rs as usize] as i32)
            < (self.general_purpose_registers[rargs.rt as usize] as i32)
        {
            self.general_purpose_registers[rargs.rd as usize] = 1 as u32;
        } else {
            self.general_purpose_registers[rargs.rd as usize] = 0 as u32;
        }
        Ok(ExecutionStatus::Continue)
    }

    // 0x2A - sltu
    pub fn sltu(&mut self, rargs: Special) -> Result<ExecutionStatus, String> {
        if self.general_purpose_registers[rargs.rs as usize as usize]
            < self.general_purpose_registers[rargs.rt as usize as usize]
        {
            self.general_purpose_registers[rargs.rd as usize] = 1; // check if this is kosher or if i need to do 00..001 for some reason
        } else {
            self.general_purpose_registers[rargs.rd as usize] = 0;
        }
        Ok(ExecutionStatus::Continue)
    }

    // 0x02 - j
    pub fn j(&mut self, memory: &mut Memory, jargs: Jump) -> Result<ExecutionStatus, String> {
        let address: u32 = (jargs.address << 2) | (self.pc & 0xF0000000);

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
    pub fn jal(&mut self, memory: &mut Memory, jargs: Jump) -> Result<ExecutionStatus, String> {
        let address: u32 = (jargs.address << 2) | (self.pc & 0xF0000000);

        if address >= memory.text_end || address < memory.text_start {
            return Err(format!(
                "Attempted to jump to unowned address 0x{:x}",
                address
            ));
        }

        self.general_purpose_registers[Register::Ra as usize] = self.pc;
        self.pc = address;

        Ok(ExecutionStatus::Continue)
    }

    // 0x04 - beq
    pub fn beq(
        &mut self,
        memory: &mut Memory,
        iargs: Immediate,
    ) -> Result<ExecutionStatus, String> {
        // Sign extend offset
        let offset: i32 = ((iargs.imm & 0xFFFF) as i16 as i32) << 2;

        if self.general_purpose_registers[iargs.rs as usize]
            != self.general_purpose_registers[iargs.rt as usize]
        {
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
    pub fn bne(
        &mut self,
        memory: &mut Memory,
        iargs: Immediate,
    ) -> Result<ExecutionStatus, String> {
        // Sign extend offset
        let offset: i32 = ((iargs.imm & 0xFFFF) as i16 as i32) << 2;

        if self.general_purpose_registers[iargs.rs as usize]
            == self.general_purpose_registers[iargs.rt as usize]
        {
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
    pub fn blez(
        &mut self,
        memory: &mut Memory,
        iargs: Immediate,
    ) -> Result<ExecutionStatus, String> {
        let offset: i32 = ((iargs.imm & 0xFFFF) as i16 as i32) << 2;

        if (self.general_purpose_registers[iargs.rs as usize] as i32) > 0 {
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
    pub fn bgtz(
        &mut self,
        memory: &mut Memory,
        iargs: Immediate,
    ) -> Result<ExecutionStatus, String> {
        // Sign extend offset
        let offset: i32 = (iargs.imm as i16 as i32) << 2;

        if self.general_purpose_registers[iargs.rs as usize] as i32 <= 0 {
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
    pub fn addi(&mut self, iargs: Immediate) -> Result<ExecutionStatus, String> {
        self.general_purpose_registers[iargs.rt as usize] =
            (self.general_purpose_registers[iargs.rs as usize] as i32 + (iargs.imm as i16 as i32))
                as u32;
        Ok(ExecutionStatus::Continue)
    }

    // 0x09 - addiu
    pub fn addiu(&mut self, iargs: Immediate) -> Result<ExecutionStatus, String> {
        self.general_purpose_registers[iargs.rt as usize] = self.general_purpose_registers
            [iargs.rs as usize]
            .overflowing_add(iargs.imm)
            .0;
        Ok(ExecutionStatus::Continue)
    }

    // 0x0C - andi
    pub fn andi(&mut self, iargs: Immediate) -> Result<ExecutionStatus, String> {
        self.general_purpose_registers[iargs.rt as usize] =
            self.general_purpose_registers[iargs.rs as usize] & iargs.imm;
        Ok(ExecutionStatus::Continue)
    }

    // 0x0D - ori
    pub fn ori(&mut self, iargs: Immediate) -> Result<ExecutionStatus, String> {
        self.general_purpose_registers[iargs.rt as usize] =
            self.general_purpose_registers[iargs.rs as usize] | iargs.imm;
        Ok(ExecutionStatus::Continue)
    }

    // 0x0E - xori
    pub fn xori(&mut self, iargs: Immediate) -> Result<ExecutionStatus, String> {
        self.general_purpose_registers[iargs.rt as usize] =
            self.general_purpose_registers[iargs.rs as usize] ^ iargs.imm;
        Ok(ExecutionStatus::Continue)
    }

    // 0x0F - lui
    pub fn lui(&mut self, iargs: Immediate) -> Result<ExecutionStatus, String> {
        // SUPER DUPER PROBLEM SPOT
        self.general_purpose_registers[iargs.rt as usize] = iargs.imm << 16;
        Ok(ExecutionStatus::Continue)
    }

    // 0x20 - lb
    pub fn lb(&mut self, memory: &mut Memory, iargs: Immediate) -> Result<ExecutionStatus, String> {
        self.general_purpose_registers[Register::At as usize] =
            (self.general_purpose_registers[iargs.rs as usize] as i32 + iargs.imm as i32) as u32;

        if self.general_purpose_registers[Register::At as usize] >= memory.data_end
            || self.general_purpose_registers[Register::At as usize] < memory.data_start
        {
            return Err(format!(
                "Attempted to access unowned address 0x{:x}",
                self.general_purpose_registers[Register::At as usize]
            ));
        } else {
            self.general_purpose_registers[iargs.rt as usize] =
                memory.data[(self.general_purpose_registers[Register::At as usize]
                    - memory.data_start) as usize] as u32;
        }

        Ok(ExecutionStatus::Continue)
    }

    // 0x23 - lw
    pub fn lw(&mut self, memory: &mut Memory, iargs: Immediate) -> Result<ExecutionStatus, String> {
        let temp =
            (self.general_purpose_registers[iargs.rs as usize] as i32 + iargs.imm as i32) as u32;

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

        self.general_purpose_registers[iargs.rt as usize] =
            u32::from_be_bytes(memory.data[start_idx..end_idx].try_into().unwrap());

        Ok(ExecutionStatus::Continue)
    }

    // 0x28 - sb
    pub fn sb(&mut self, memory: &mut Memory, iargs: Immediate) -> Result<ExecutionStatus, String> {
        let temp =
            (self.general_purpose_registers[iargs.rs as usize] as i32 + iargs.imm as i32) as u32;

        if temp >= memory.data_end || temp < memory.data_start {
            return Err(format!("Attempted to access unowned address 0x{:x}", temp));
        }

        memory.data[(temp - memory.data_start) as usize] =
            self.general_purpose_registers[iargs.rt as usize] as u8;

        Ok(ExecutionStatus::Continue)
    }

    // 0x2b - sw
    pub fn sw(&mut self, memory: &mut Memory, iargs: Immediate) -> Result<ExecutionStatus, String> {
        let temp =
            (self.general_purpose_registers[iargs.rs as usize] as i32 + iargs.imm as i32) as u32;

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
            self.general_purpose_registers[iargs.rt as usize].to_be_bytes(),
        );

        // println!("Storing {} at 0x{:x} from ${}", self.general_purpose_registers[rt], temp, rt);

        Ok(ExecutionStatus::Continue)
    }
}
