const MASK_5BIT: u32 = 0b11111;
const MASK_7BIT: u32 = 0b1111111;

pub enum Instruction {
    TypeR {
        opcode: OpcodeR,
        rs1: usize,
        rs2: usize,
        rd: usize,
    },
    TypeI {
        opcode: OpcodeI,
        rs1: usize,
        rd: usize,
        imm: u32,
    },
    TypeS {
        opcode: OpcodeS,
        rs1: usize,
        rs2: usize,
        imm: u32,
    },
    TypeB {
        opcode: OpcodeB,
        rs1: usize,
        rs2: usize,
        imm: u32,
    },
    TypeU {
        opcode: OpcodeU,
        rd: usize,
        imm: u32,
    },
    TypeJ {
        opcode: OpcodeJ,
        rd: usize,
        imm: u32,
    },
}

impl Instruction {
    pub fn decode(instruction: u32) -> Option<Self> {
        let opcode = instruction & MASK_7BIT;
        let funct3 = (instruction >> 12) & MASK_5BIT;
        let funct7 = (instruction >> 25) & MASK_7BIT;
        match opcode {
            0b0110111 => Self::decode_u(Some(OpcodeU::Lui), instruction),
            0b0010111 => Self::decode_u(Some(OpcodeU::Auipc), instruction),
            0b1101111 => Self::decode_j(Some(OpcodeJ::Jal), instruction),
            0b1100111 => Self::decode_i(Some(OpcodeI::Jalr), instruction),
            0b1100011 => Self::decode_b(
                match funct3 {
                    0b000 => Some(OpcodeB::Beq),
                    0b001 => Some(OpcodeB::Bne),
                    0b100 => Some(OpcodeB::Blt),
                    0b101 => Some(OpcodeB::Bge),
                    0b110 => Some(OpcodeB::Bltu),
                    0b111 => Some(OpcodeB::Bgeu),
                    _ => None,
                },
                instruction,
            ),
            0b0000011 => Self::decode_i(
                match funct3 {
                    0b000 => Some(OpcodeI::Lb),
                    0b001 => Some(OpcodeI::Lh),
                    0b010 => Some(OpcodeI::Lw),
                    0b100 => Some(OpcodeI::Lbu),
                    0b101 => Some(OpcodeI::Lhu),
                    _ => None,
                },
                instruction,
            ),
            0b0100011 => Self::decode_s(
                match funct3 {
                    0b000 => Some(OpcodeS::Sb),
                    0b001 => Some(OpcodeS::Sh),
                    0b010 => Some(OpcodeS::Sw),
                    _ => None,
                },
                instruction,
            ),
            0b0010011 => Self::decode_i(
                match funct3 {
                    0b000 => Some(OpcodeI::Addi),
                    0b010 => Some(OpcodeI::Slti),
                    0b011 => Some(OpcodeI::Sltiu),
                    0b100 => Some(OpcodeI::Xori),
                    0b110 => Some(OpcodeI::Ori),
                    0b111 => Some(OpcodeI::Andi),
                    0b001 => Some(OpcodeI::Slli),
                    0b101 => match funct7 {
                        0b0000000 => Some(OpcodeI::Srli),
                        0b0100000 => Some(OpcodeI::Srai),
                        _ => None,
                    },
                    _ => None,
                },
                instruction,
            ),
            0b0110011 => Self::decode_r(
                match funct3 {
                    0b000 => match funct7 {
                        0b0000000 => Some(OpcodeR::Add),
                        0b0100000 => Some(OpcodeR::Sub),
                        _ => None,
                    },
                    0b001 => Some(OpcodeR::Sll),
                    0b010 => Some(OpcodeR::Slt),
                    0b011 => Some(OpcodeR::Sltu),
                    0b100 => Some(OpcodeR::Xor),
                    0b101 => match funct7 {
                        0b0000000 => Some(OpcodeR::Srl),
                        0b0100000 => Some(OpcodeR::Sra),
                        _ => None,
                    },
                    0b110 => Some(OpcodeR::Or),
                    0b111 => Some(OpcodeR::And),
                    _ => None,
                },
                instruction,
            ),
            0b0001111 => Self::decode_i(
                match funct3 {
                    0b000 => Some(OpcodeI::Fence),
                    0b001 => Some(OpcodeI::FenceI),
                    _ => None,
                },
                instruction,
            ),
            0b1110011 => Self::decode_i(
                match funct3 {
                    0b000 => match instruction >> 20 {
                        0b0 => Some(OpcodeI::Ecall),
                        0b1 => Some(OpcodeI::Ebreak),
                        _ => None,
                    },
                    0b001 => Some(OpcodeI::Csrrw),
                    0b010 => Some(OpcodeI::Csrrs),
                    0b011 => Some(OpcodeI::Csrrc),
                    0b101 => Some(OpcodeI::Csrrwi),
                    0b110 => Some(OpcodeI::Csrrsi),
                    0b111 => Some(OpcodeI::Csrrci),
                    _ => None,
                },
                instruction,
            ),
            _ => None,
        }
    }

    fn decode_r(opcode: Option<OpcodeR>, instruction: u32) -> Option<Self> {
        opcode.map(|o| {
            let rs1 = ((instruction >> 15) & MASK_5BIT) as usize;
            let rs2 = ((instruction >> 20) & MASK_5BIT) as usize;
            let rd = ((instruction >> 7) & MASK_5BIT) as usize;
            Self::TypeR {
                opcode: o,
                rs1,
                rs2,
                rd,
            }
        })
    }

    fn decode_i(opcode: Option<OpcodeI>, instruction: u32) -> Option<Self> {
        opcode.map(|o| {
            let rd = ((instruction >> 7) & MASK_5BIT) as usize;
            let rs1 = ((instruction >> 15) & MASK_5BIT) as usize;
            let imm = ((instruction as i32) >> 20) as u32;
            Self::TypeI {
                opcode: o,
                rd,
                rs1,
                imm,
            }
        })
    }

    fn decode_s(opcode: Option<OpcodeS>, instruction: u32) -> Option<Self> {
        opcode.map(|o| {
            let rs1 = ((instruction >> 15) & MASK_5BIT) as usize;
            let rs2 = ((instruction >> 20) & MASK_5BIT) as usize;
            let imm = ((instruction & 0xfe000000) as i32 >> 20) as u32 | (instruction >> 7) & 0x1f;
            Self::TypeS {
                opcode: o,
                rs1,
                rs2,
                imm,
            }
        })
    }

    fn decode_b(opcode: Option<OpcodeB>, instruction: u32) -> Option<Self> {
        opcode.map(|o| {
            let rs1 = ((instruction >> 15) & MASK_5BIT) as usize;
            let rs2 = ((instruction >> 20) & MASK_5BIT) as usize;
            let imm = ((instruction & 0x80000000) as i32 >> 19) as u32
                | (instruction & 0x80) << 4
                | (instruction >> 20) & 0x7e0
                | (instruction >> 8) & 0x1e;
            Self::TypeB {
                opcode: o,
                rs1,
                rs2,
                imm,
            }
        })
    }

    fn decode_u(opcode: Option<OpcodeU>, instruction: u32) -> Option<Self> {
        opcode.map(|o| {
            let rd = ((instruction >> 7) & MASK_5BIT) as usize;
            let imm = instruction & 0xfffff000;
            Self::TypeU { opcode: o, rd, imm }
        })
    }

    fn decode_j(opcode: Option<OpcodeJ>, instruction: u32) -> Option<Self> {
        opcode.map(|o| {
            let rd = ((instruction >> 7) & MASK_5BIT) as usize;
            let imm = ((instruction & 0x80000000) as i32 >> 11) as u32
                | instruction & 0xff000
                | (instruction >> 9) & 0x800
                | (instruction >> 20) & 0x7fe;
            Self::TypeJ { opcode: o, rd, imm }
        })
    }
}

pub enum OpcodeR {
    Sll,
    Srl,
    Sra,
    Add,
    Sub,
    Xor,
    Or,
    And,
    Slt,
    Sltu,
}

pub enum OpcodeI {
    Slli,
    Srli,
    Srai,
    Addi,
    Xori,
    Ori,
    Andi,
    Slti,
    Sltiu,
    Jalr,
    Fence,
    FenceI,
    Ecall,
    Ebreak,
    Csrrw,
    Csrrs,
    Csrrc,
    Csrrwi,
    Csrrsi,
    Csrrci,
    Lb,
    Lh,
    Lbu,
    Lhu,
    Lw,
}

pub enum OpcodeS {
    Sb,
    Sh,
    Sw,
}

pub enum OpcodeB {
    Beq,
    Bne,
    Blt,
    Bge,
    Bltu,
    Bgeu,
}

pub enum OpcodeU {
    Lui,
    Auipc,
}

pub enum OpcodeJ {
    Jal,
}
