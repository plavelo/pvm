use crate::{
    bitops::{extend_sign, shift_amount},
    isa::{
        description::{xformat3, xformat_immediate, xformat_offset, Describer, Description},
        instruction::{
            rv64i::{
                Rv64iOpcodeB, Rv64iOpcodeI, Rv64iOpcodeJ, Rv64iOpcodeR, Rv64iOpcodeS, Rv64iOpcodeU,
            },
            Instruction,
        },
    },
};

impl Describer
    for Instruction<
        Rv64iOpcodeR,
        Rv64iOpcodeI,
        Rv64iOpcodeS,
        Rv64iOpcodeB,
        Rv64iOpcodeU,
        Rv64iOpcodeJ,
    >
{
    type OpcodeR = Rv64iOpcodeR;
    type OpcodeI = Rv64iOpcodeI;
    type OpcodeS = Rv64iOpcodeS;
    type OpcodeB = Rv64iOpcodeB;
    type OpcodeU = Rv64iOpcodeU;
    type OpcodeJ = Rv64iOpcodeJ;

    fn describe(&self) -> Description {
        let (description, assembly, signature, pseudocode) = match *self {
            Self::TypeR {
                opcode,
                rd,
                funct3: _,
                rs1,
                rs2,
                funct7: _,
            } => match opcode {
                Rv64iOpcodeR::Sllw => (
                    "Shift Left Logical Word",
                    xformat3(opcode.to_string(), rd, rs1, rs2),
                    "sllw rd,rs1,rs2",
                    "x[rd] = sext((x[rs1] << x[rs2][4:0])[31:0])",
                ),
                Rv64iOpcodeR::Srlw => (
                    "Shift Right Logical Word",
                    xformat3(opcode.to_string(), rd, rs1, rs2),
                    "srlw rd,rs1,rs2",
                    "x[rd] = sext(x[rs1][31:0] >>u x[rs2][4:0])",
                ),
                Rv64iOpcodeR::Sraw => (
                    "Shift Right Arithmetic Word",
                    xformat3(opcode.to_string(), rd, rs1, rs2),
                    "sraw rd,rs1,rs2",
                    "x[rd] = sext(x[rs1][31:0] >>s x[rs2][4:0])",
                ),
                Rv64iOpcodeR::Addw => (
                    "Add Word",
                    xformat3(opcode.to_string(), rd, rs1, rs2),
                    "addw rd,rs1,rs2",
                    "x[rd] = sext(x[rs1] + x[rs2])[31:0]",
                ),
                Rv64iOpcodeR::Subw => (
                    "Subtract Word",
                    xformat3(opcode.to_string(), rd, rs1, rs2),
                    "subw rd,rs1,rs2",
                    "x[rd] = sext(x[rs1] - x[rs2])[31:0]",
                ),
            },
            Self::TypeI {
                opcode,
                rd,
                funct3: _,
                rs1,
                imm,
            } => match opcode {
                Rv64iOpcodeI::Slliw => (
                    "Shift Left Logical Word Immediate",
                    xformat_immediate(opcode.to_string(), rd, rs1, shift_amount(imm) as i64),
                    "slliw rd,rs1,shamt",
                    "x[rd] = sext((x[rs1] << shamt)[31:0])",
                ),
                Rv64iOpcodeI::Srliw => (
                    "Shift Right Logical Word Immediate",
                    xformat_immediate(opcode.to_string(), rd, rs1, shift_amount(imm) as i64),
                    "srliw rd,rs1,shamt",
                    "x[rd] = sext(x[rs1][31:0] >>u shamt)",
                ),
                Rv64iOpcodeI::Sraiw => (
                    "Shift Right Arithmetic Word Immediate",
                    xformat_immediate(opcode.to_string(), rd, rs1, shift_amount(imm) as i64),
                    "sraiw rd,rs1,shamt",
                    "x[rd] = sext(x[rs1][31:0] >>s shamt)",
                ),
                Rv64iOpcodeI::Addiw => (
                    "Add Word Immediate",
                    xformat_immediate(opcode.to_string(), rd, rs1, extend_sign(imm, 12)),
                    "addiw rd,rs1,imm",
                    "x[rd] = sext((x[rs1] + sext(imm))[31:0])",
                ),
                Rv64iOpcodeI::Lwu => (
                    "Load Word, Unsigned",
                    xformat_offset(opcode.to_string(), rd, extend_sign(imm, 12), rs1),
                    "lwu rd,offset(rs1)",
                    "x[rd] = mem[x[rs1] + sext(offset)][31:0]",
                ),
                Rv64iOpcodeI::Ld => (
                    "Load Doubleword",
                    xformat_offset(opcode.to_string(), rd, extend_sign(imm, 12), rs1),
                    "ld rd,offset(rs1)",
                    "x[rd] = mem[x[rs1] + sext(offset)][63:0]",
                ),
            },
            Self::TypeS {
                opcode,
                funct3: _,
                rs1,
                rs2,
                imm,
            } => match opcode {
                Rv64iOpcodeS::Sd => (
                    "Store Doubleword",
                    xformat_offset(opcode.to_string(), rs2, extend_sign(imm, 12), rs1),
                    "sd rs2,offset(rs1)",
                    "mem[x[rs1] + sext(offset)] = x[rs2][63:0]",
                ),
            },
            _ => panic!(),
        };
        Description {
            description: description.to_string(),
            assembly,
            singnature: signature.to_string(),
            pseudocode: pseudocode.to_string(),
        }
    }
}
