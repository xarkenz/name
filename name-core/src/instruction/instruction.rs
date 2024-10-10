#[derive(Debug, Copy, Clone)]
pub struct RawInstruction {
    pub raw: u32,
}

impl RawInstruction {
    pub fn new(raw: u32) -> RawInstruction {
        RawInstruction { raw }
    }

    pub fn get_opcode(self) -> u32 {
        self.raw >> 26
    }

    pub fn get_funct(self) -> u32 {
        self.raw & 0x3F
    }

    pub fn is_rtype(self) -> bool {
        let op = self.get_opcode();
        op == 0x00 || op == 0x1C
    }

    pub fn is_jtype(self) -> bool {
        let op = self.get_opcode();
        op == 0x02 || op == 0x03
    }

    pub fn is_itype(self) -> bool {
        !self.is_rtype() && !self.is_jtype()
    }

    pub fn is_regimm(self) -> bool {
        self.get_opcode() == 0x01
    }

    pub fn get_rs(self) -> u32 {
        self.raw >> 21 & 0x1F
    }

    pub fn get_rt(self) -> u32 {
        self.raw >> 16 & 0x1F
    }

    pub fn get_rd(self) -> u32 {
        self.raw >> 11 & 0x1F
    }

    pub fn get_shamt(self) -> u32 {
        self.raw >> 6 & 0x1F
    }

    pub fn get_immediate(self) -> u32 {
        self.raw & 0xFFFF
    }

    pub fn get_jump(self) -> u32 {
        self.raw & 0x3FFFFFF
    }

    pub fn get_lookup(self) -> u32 {
        let base = self.get_opcode() << 6;
        if self.is_rtype() {
            base | self.get_funct()
        } else if self.is_regimm() {
            base | self.get_rt()
        } else {
            base
        }
    }
}

pub struct IArgs {
    pub opcode: u32,
    pub rs: u32,
    pub rt: u32,
    pub imm: u32,
}

pub struct JArgs {
    pub opcode: u32,
    pub address: u32,
}

pub struct RArgs {
    pub opcode: u32,
    pub rs: u32,
    pub rt: u32,
    pub rd: u32,
    pub shamt: u32,
    pub funct: u32,
}

pub enum Instruction {
    IType(IArgs),
    JType(JArgs),
    RType(RArgs),
}

impl From<RawInstruction> for Instruction {
    fn from(raw: RawInstruction) -> Instruction {
        let opcode = raw.get_opcode();
        if raw.is_jtype() {
            Self::JType(JArgs {
                opcode,
                address: raw.get_jump(),
            })
        } else if raw.is_rtype() {
            Self::RType(RArgs {
                opcode,
                rs: raw.get_rs(),
                rt: raw.get_rt(),
                rd: raw.get_rd(),
                shamt: raw.get_shamt(),
                funct: raw.get_funct(),
            })
        } else {
            Self::IType(IArgs {
                opcode,
                rs: raw.get_rs(),
                rt: raw.get_rt(),
                imm: raw.get_immediate(),
            })
        }
    }
}

impl Instruction {
    pub fn jtype(self) -> Option<JArgs> {
        match self {
            Self::JType(jargs) => Some(jargs),
            _ => None,
        }
    }
    pub fn itype(self) -> Option<IArgs> {
        match self {
            Self::IType(iargs) => Some(iargs),
            _ => None,
        }
    }
    pub fn rtype(self) -> Option<RArgs> {
        match self {
            Self::RType(rargs) => Some(rargs),
            _ => None,
        }
    }
}
