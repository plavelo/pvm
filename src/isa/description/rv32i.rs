use crate::isa::{
    description::{
        format3, format_immediate, format_offset, format_upper_immediate, Describer, Description,
        MASK_6BIT,
    },
    instruction::{
        rv32i::{
            Rv32iOpcodeB, Rv32iOpcodeI, Rv32iOpcodeJ, Rv32iOpcodeR, Rv32iOpcodeS, Rv32iOpcodeU,
        },
        Instruction,
    },
};

impl Describer
    for Instruction<
        Rv32iOpcodeR,
        Rv32iOpcodeI,
        Rv32iOpcodeS,
        Rv32iOpcodeB,
        Rv32iOpcodeU,
        Rv32iOpcodeJ,
    >
{
    type OpcodeR = Rv32iOpcodeR;
    type OpcodeI = Rv32iOpcodeI;
    type OpcodeS = Rv32iOpcodeS;
    type OpcodeB = Rv32iOpcodeB;
    type OpcodeU = Rv32iOpcodeU;
    type OpcodeJ = Rv32iOpcodeJ;

    fn describe(&self) -> Description {
        let (description, assembly, signature, pseudocode) = match self {
            Self::TypeR {
                opcode,
                rd,
                funct3: _,
                rs1,
                rs2,
                funct7: _,
            } => match opcode {
                Rv32iOpcodeR::Sll => (
                    "Shift Left Logical",
                    format3(opcode.to_string(), rd, rs1, rs2),
                    "sll rd,rs1,rs2",
                    "x[rd] = x[rs1] << x[rs2]",
                ),
                Rv32iOpcodeR::Srl => (
                    "Shift Right Logical",
                    format3(opcode.to_string(), rd, rs1, rs2),
                    "srl rd,rs1,rs2",
                    "x[rd] = x[rs1] >>u x[rs2]",
                ),
                Rv32iOpcodeR::Sra => (
                    "Shift Right Arithmetic",
                    format3(opcode.to_string(), rd, rs1, rs2),
                    "sra rd,rs1,rs2",
                    "x[rd] = x[rs1] >>s x[rs2]",
                ),
                Rv32iOpcodeR::Add => (
                    "Add",
                    format3(opcode.to_string(), rd, rs1, rs2),
                    "add rd,rs1,rs2",
                    "x[rd] = x[rs1] + x[rs2]",
                ),
                Rv32iOpcodeR::Sub => (
                    "Subtract",
                    format3(opcode.to_string(), rd, rs1, rs2),
                    "sub rd,rs1,rs2",
                    "x[rd] = x[rs1] - x[rs2]",
                ),
                Rv32iOpcodeR::Xor => (
                    "Exclusive-OR",
                    format3(opcode.to_string(), rd, rs1, rs2),
                    "xor rd,rs1,rs2",
                    "x[rd] = x[rs1] ^ x[rs2]",
                ),
                Rv32iOpcodeR::Or => (
                    "OR",
                    format3(opcode.to_string(), rd, rs1, rs2),
                    "or rd,rs1,rs2",
                    "x[rd] = x[rs1] | x[rs2]",
                ),
                Rv32iOpcodeR::And => (
                    "AND",
                    format3(opcode.to_string(), rd, rs1, rs2),
                    "and rd,rs1,rs2",
                    "x[rd] = x[rs1] & x[rs2]",
                ),
                Rv32iOpcodeR::Slt => (
                    "Set if Less Than",
                    format3(opcode.to_string(), rd, rs1, rs2),
                    "slt rd,rs1,rs2",
                    "x[rd] = x[rs1] <s x[rs2]",
                ),
                Rv32iOpcodeR::Sltu => (
                    "Set if Less Than, Unsigned",
                    format3(opcode.to_string(), rd, rs1, rs2),
                    "sltu rd,rs1,rs2",
                    "x[rd] = x[rs1] <u x[rs2]",
                ),
            },
            Self::TypeI {
                opcode,
                rd,
                funct3: _,
                rs1,
                imm,
            } => match opcode {
                Rv32iOpcodeI::Slli => (
                    "Shift Left Logical Immediate",
                    format_immediate(opcode.to_string(), rd, rs1, &(imm & MASK_6BIT)),
                    "slli rd,rs1,shamt",
                    "x[rd] = x[rs1] << shamt",
                ),
                Rv32iOpcodeI::Srli => (
                    "Shift Right Logical Immediate",
                    format_immediate(opcode.to_string(), rd, rs1, &(imm & MASK_6BIT)),
                    "srli rd,rs1,shamt",
                    "x[rd] = x[rs1] >>u shamt",
                ),
                Rv32iOpcodeI::Srai => (
                    "Shift Right Arithmetic Immediate",
                    format_immediate(opcode.to_string(), rd, rs1, &(imm & MASK_6BIT)),
                    "srai rd,rs1,shamt",
                    "x[rd] = x[rs1] >>s shamt",
                ),
                Rv32iOpcodeI::Addi => (
                    "Add Immediate",
                    format_immediate(opcode.to_string(), rd, rs1, imm),
                    "addi rd,rs1,imm",
                    "x[rd] = x[rs1] + sext(imm)",
                ),
                Rv32iOpcodeI::Xori => (
                    "Exclusive-OR Immediate",
                    format_immediate(opcode.to_string(), rd, rs1, imm),
                    "xori rd,rs1,imm",
                    "x[rd] = x[rs1] ^ sext(imm)",
                ),
                Rv32iOpcodeI::Ori => (
                    "OR Immediate",
                    format_immediate(opcode.to_string(), rd, rs1, imm),
                    "ori rd,rs1,imm",
                    "x[rd] = x[rs1] | sext(imm)",
                ),
                Rv32iOpcodeI::Andi => (
                    "AND Immediate",
                    format_immediate(opcode.to_string(), rd, rs1, imm),
                    "andi rd,rs1,imm",
                    "x[rd] = x[rs1] & sext(imm)",
                ),
                Rv32iOpcodeI::Slti => (
                    "Set if Less Than Immediate",
                    format_immediate(opcode.to_string(), rd, rs1, imm),
                    "slti rd,rs1,imm",
                    "x[rd] = x[rs1] <s sext(imm)",
                ),
                Rv32iOpcodeI::Sltiu => (
                    "Set if Less Than Immediate, Unsigned",
                    format_immediate(opcode.to_string(), rd, rs1, imm),
                    "sltiu rd,rs1,imm",
                    "x[rd] = x[rs1] <u sext(imm)",
                ),
                Rv32iOpcodeI::Jalr => (
                    "Jump and Link Register",
                    format_immediate(opcode.to_string(), rd, rs1, imm),
                    "jalr rd,offset(rs1)",
                    "t=pc+4; pc=(x[rs1]+sext(offset))&~1; x[rd]=t",
                ),

                Rv32iOpcodeI::Fence => (
                    "Fence Memory and I/O",
                    format_immediate(opcode.to_string(), rd, rs1, imm),
                    "fence pred,succ",
                    "fence(pred, succ)",
                ),
                Rv32iOpcodeI::Ecall => (
                    "Environment Call",
                    format_immediate(opcode.to_string(), rd, rs1, imm),
                    "ecall",
                    "raise EnvirnmentCall",
                ),
                Rv32iOpcodeI::Ebreak => (
                    "Environment Breakpoint",
                    format_immediate(opcode.to_string(), rd, rs1, imm),
                    "ebreak",
                    "raise Breakpoint",
                ),
                Rv32iOpcodeI::Lb => (
                    "Load Byte",
                    format_immediate(opcode.to_string(), rd, rs1, imm),
                    "lb rd,offset(rs1)",
                    "x[rd] = sext(mem[x[rs1] + sext(offset)][7:0])",
                ),

                Rv32iOpcodeI::Lh => (
                    "Load Halfword",
                    format_immediate(opcode.to_string(), rd, rs1, imm),
                    "lh rd,offset(rs1)",
                    "x[rd] = sext(mem[x[rs1] + sext(offset)][15:0])",
                ),
                Rv32iOpcodeI::Lbu => (
                    "Load Byte, Unsigned",
                    format_immediate(opcode.to_string(), rd, rs1, imm),
                    "lbu rd,offset(rs1)",
                    "x[rd] = mem[x[rs1] + sext(offset)][7:0]",
                ),
                Rv32iOpcodeI::Lhu => (
                    "Load Halfword, Unsigned",
                    format_immediate(opcode.to_string(), rd, rs1, imm),
                    "lhu rd,offset(rs1)",
                    "x[rd] = mem[x[rs1] + sext(offset)][15:0]",
                ),
                Rv32iOpcodeI::Lw => (
                    "Load Word",
                    format_immediate(opcode.to_string(), rd, rs1, imm),
                    "lw rd,offset(rs1)",
                    "x[rd] = sext(mem[x[rs1] + sext(offset)][31:0])",
                ),
            },
            Self::TypeS {
                opcode,
                funct3: _,
                rs1,
                rs2,
                imm,
            } => match opcode {
                Rv32iOpcodeS::Sb => (
                    "Store Byte",
                    format_offset(opcode.to_string(), rs2, imm, rs1),
                    "sb rs2,offset(rs1)",
                    "x[rd] = mem[x[rs1] + sext(offset)] = x[rs2][7:0]",
                ),
                Rv32iOpcodeS::Sh => (
                    "Store Halfword",
                    format_offset(opcode.to_string(), rs2, imm, rs1),
                    "sh rs2,offset(rs1)",
                    "x[rd] = mem[x[rs1] + sext(offset)] = x[rs2][15:0]",
                ),
                Rv32iOpcodeS::Sw => (
                    "Store Word",
                    format_offset(opcode.to_string(), rs2, imm, rs1),
                    "sw rs2,offset(rs1)",
                    "x[rd] = mem[x[rs1] + sext(offset)] = x[rs2][31:0]",
                ),
            },
            Self::TypeB {
                opcode,
                funct3: _,
                rs1,
                rs2,
                imm,
            } => match opcode {
                Rv32iOpcodeB::Beq => (
                    "Branch if Equal",
                    format_immediate(opcode.to_string(), rs1, rs2, imm),
                    "beq rs1,rs2,offset",
                    "if (rs1 == rs2) pc += sext(offset)",
                ),
                Rv32iOpcodeB::Bne => (
                    "Branch if Not Equal",
                    format_immediate(opcode.to_string(), rs1, rs2, imm),
                    "bne rs1,rs2,offset",
                    "if (rs1 != rs2) pc += sext(offset)",
                ),
                Rv32iOpcodeB::Blt => (
                    "Branch if Less Than",
                    format_immediate(opcode.to_string(), rs1, rs2, imm),
                    "blt rs1,rs2,offset",
                    "if (rs1 >=s rs2) pc += sext(offset)",
                ),
                Rv32iOpcodeB::Bge => (
                    "Branch if Greater Than or Equal",
                    format_immediate(opcode.to_string(), rs1, rs2, imm),
                    "bge rs1,rs2,offset",
                    "if (rs1 >=s rs2) pc += sext(offset)",
                ),
                Rv32iOpcodeB::Bltu => (
                    "Branch if Less Than, Unsigned",
                    format_immediate(opcode.to_string(), rs1, rs2, imm),
                    "bltu rs1,rs2,offset",
                    "if (rs1 <u rs2) pc += sext(offset)",
                ),
                Rv32iOpcodeB::Bgeu => (
                    "Branch if Greater Than or Equal, Unsigned",
                    format_immediate(opcode.to_string(), rs1, rs2, imm),
                    "bgeu rs1,rs2,offset",
                    "if (rs1 >=u rs2) pc += sext(offset)",
                ),
            },
            Self::TypeU { opcode, rd, imm } => match opcode {
                Rv32iOpcodeU::Lui => (
                    "Load Upper Immediate",
                    format_upper_immediate(opcode.to_string(), rd, imm),
                    "lui rd,imm",
                    "x[rd] = sext(imm[31:12] << 12)",
                ),
                Rv32iOpcodeU::Auipc => (
                    "Add Upper Immediate to PC",
                    format_upper_immediate(opcode.to_string(), rd, imm),
                    "auipc rd,imm",
                    "x[rd] = pc + sext(imm[31:12] << 12)",
                ),
            },
            Self::TypeJ { opcode, rd, imm } => match opcode {
                Rv32iOpcodeJ::Jal => (
                    "Jump and Link",
                    format_upper_immediate(opcode.to_string(), rd, imm),
                    "jal rd,offset",
                    "x[rd] = pc+4; pc += sext(offset)",
                ),
            },
        };
        Description {
            description: description.to_string(),
            assembly,
            singnature: signature.to_string(),
            pseudocode: pseudocode.to_string(),
        }
    }
}
