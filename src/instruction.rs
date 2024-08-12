
use crate::riscv_sim::opcode::*;

const MASK_OPCODE: u32 = 0x7f;
const MASK_FUNCT3: u32 = 0x7000;
const MASK_FUNCT7: u32 = 0xfe000000;
const MASK_RD: u32 = 0xf80;
const MASK_RS1: u32 = 0x0f8000;
const MASK_RS2: u32 = 0x01f00000;
const MASK_IMM_I: u32 = 0xfff00000;
const MASK_IMM_S: u32 = 0xfe000000;
const MASK_IMM_B: u32 = 0x7e000000;
const MASK_IMM_U: u32 = 0xfffff000;
const MASK_IMM_J: u32 = 0xff000000;

// Define shifts for different instruction fields
const SHIFT_OPCODE: u32 = 0;
const SHIFT_FUNCT3: u32 = 12;
const SHIFT_FUNCT7: u32 = 25;
const SHIFT_RD: u32 = 7;
const SHIFT_RS1: u32 = 15;
const SHIFT_RS2: u32 = 20;
const SHIFT_IMM_I: u32 = 20;
const SHIFT_IMM_S: u32 = 25;
const SHIFT_IMM_B: u32 = 19;
const SHIFT_IMM_U: u32 = 12;
const SHIFT_IMM_J: u32 = 12;

pub fn sign_extend(value: u32, bit_width: u8) -> u32 {
    let shift = 32 - bit_width ;
    ((value << shift) ) >> shift
}

fn extract_field(instruction: u32, mask: u32, shift: u32) -> u32 {
    (instruction & mask) >> shift
}

#[derive(Debug)]
pub enum Instruction {
    // R-type instructions
    Add(RTypeInstruction),
    Sub(RTypeInstruction),
    Sll(RTypeInstruction),
    Slt(RTypeInstruction),
    Sltu(RTypeInstruction),
    Xor(RTypeInstruction),
    Srl(RTypeInstruction),
    Sra(RTypeInstruction), 
    Or(RTypeInstruction),
    And(RTypeInstruction),

    // I-type instructions
    Addi(ITypeInstruction),
    Slti(ITypeInstruction),
    Sltiu(ITypeInstruction),
    Xori(ITypeInstruction),
    Ori(ITypeInstruction),
    Andi(ITypeInstruction),
    Slli(ITypeInstruction),
    Srli(ITypeInstruction),
    Srai(ITypeInstruction),
    Lb(ITypeInstruction),
    Lh(ITypeInstruction),
    Lw(ITypeInstruction),
    Lbu(ITypeInstruction),
    Lhu(ITypeInstruction),
    Jalr(ITypeInstruction),

    // S-type instructions
    Sb(STypeInstruction),
    Sh(STypeInstruction),
    Sw(STypeInstruction),

    // B-type instructions
    Beq(BTypeInstruction),
    Bne(BTypeInstruction),
    Blt(BTypeInstruction),
    Bge(BTypeInstruction),
    Bltu(BTypeInstruction),
    Bgeu(BTypeInstruction),

    // U-type instructions
    Lui(UTypeInstruction),
    Auipc(UTypeInstruction),

    // J-type instructions
    Jal(JTypeInstruction),
    

    // Special case for undefined instructions
    Unknown,
}

/// Enum to represent the various instruction types
#[derive(Debug)]
pub enum BaseInstruction {
    RType(RTypeInstruction),
    IType(ITypeInstruction),
    SType(STypeInstruction),
    BType(BTypeInstruction),
    UType(UTypeInstruction),
    JType(JTypeInstruction),
    // Add more instruction types as needed
}

/// R-type BaseInstruction
#[derive(Debug)]
pub struct RTypeInstruction {
    pub opcode: u8,
    pub rd: u8,
    pub funct3: u8,
    pub rs1: u8,
    pub rs2: u8,
    pub funct7: u8,
}

/// I-type BaseInstruction
#[derive(Debug)]
pub struct ITypeInstruction {
    pub opcode: u8,
    pub rd: u8,
    pub funct3: u8,
    pub rs1: u8,
    pub imm: i32,
}

/// S-type BaseInstruction
#[derive(Debug)]
pub struct STypeInstruction {
    pub opcode: u8,
    pub funct3: u8,
    pub rs1: u8,
    pub rs2: u8,
    pub imm: i32,
}

/// B-type BaseInstruction
#[derive(Debug)]
pub struct BTypeInstruction {
    pub opcode: u8,
    pub funct3: u8,
    pub rs1: u8,
    pub rs2: u8,
    pub imm: i32,
}

/// U-type BaseInstruction
#[derive(Debug)]
pub struct UTypeInstruction {
    pub opcode: u8,
    pub rd: u8,
    pub imm: i32,
}

/// J-type BaseInstruction
#[derive(Debug)]
pub struct JTypeInstruction {
    pub opcode: u8,
    pub rd: u8,
    pub imm: i32,
}

impl BaseInstruction {

    pub fn decode_instruction_by_opcode(instruction: u32) -> BaseInstruction {
        let opcode: u8 = extract_field(instruction, MASK_OPCODE, SHIFT_OPCODE) as u8;
        let funct3: u8 = extract_field(instruction, MASK_FUNCT3, SHIFT_FUNCT3) as u8;
        let funct7: u8 = extract_field(instruction, MASK_FUNCT7, SHIFT_FUNCT7) as u8;
        let rd: u8 = extract_field(instruction, MASK_RD, SHIFT_RD) as u8;
        let rs1: u8 = extract_field(instruction, MASK_RS1, SHIFT_RS1) as u8;
        let rs2: u8 = extract_field(instruction, MASK_RS2, SHIFT_RS2) as u8;
        let imm_i: i32 = extract_field(instruction, MASK_IMM_I, SHIFT_IMM_I) as i32; // Immediate for I-type
        let imm_s: i32 = extract_field(instruction, MASK_IMM_S, SHIFT_IMM_S) as i32; // Immediate for S-type
        let imm_b: i32 = extract_field(instruction, MASK_IMM_B, SHIFT_IMM_B) as i32; // Immediate for B-type
        let imm_u: i32 = extract_field(instruction, MASK_IMM_U, SHIFT_IMM_U) as i32; // Immediate for U-type
        let imm_j: i32 = sign_extend(extract_field(instruction, MASK_IMM_J, SHIFT_IMM_J), 20) as i32;  // Sign-extend for J-type
        
        println!("{} {} {} ", rd, rs1, rs2);

        match opcode {
            OPCODE_LUI | OPCODE_AUIPC => BaseInstruction::UType(UTypeInstruction { opcode, rd, imm: imm_u }),
            OPCODE_JAL => BaseInstruction::JType(JTypeInstruction { opcode, rd, imm: imm_j }),
            OPCODE_JALR => BaseInstruction::IType(ITypeInstruction { opcode, rd, funct3, rs1, imm: imm_i }),
            // | OPCODE_BNE | OPCODE_BLT | OPCODE_BGE | OPCODE_BLTU | OPCODE_BGEU
            OPCODE_BEQ => {
                BaseInstruction::BType(BTypeInstruction { opcode, funct3, rs1, rs2, imm: imm_b })
            }
            // | OPCODE_LH | OPCODE_LW | OPCODE_LBU | OPCODE_LHU
            OPCODE_LB => {
                BaseInstruction::IType(ITypeInstruction { opcode, rd, funct3, rs1, imm: imm_i })
            }
            // |  OPCODE_SH | OPCODE_SW
            OPCODE_SB => {
                BaseInstruction::SType(STypeInstruction { opcode, funct3, rs1, rs2, imm: imm_s })
            }
            // | OPCODE_SLTI | OPCODE_SLTIU | OPCODE_XORI | OPCODE_ORI | OPCODE_ANDI | OPCODE_SLLI | OPCODE_SRLI | OPCODE_SRAI
            OPCODE_ADDI => {
                BaseInstruction::IType(ITypeInstruction { opcode, rd, funct3, rs1, imm: imm_i })
            }
            // | OPCODE_SUB | OPCODE_SLL | OPCODE_SLT | OPCODE_SLTU | OPCODE_XOR | OPCODE_SRL | OPCODE_SRA | OPCODE_OR | OPCODE_AND
            OPCODE_ADD => {
                BaseInstruction::RType(RTypeInstruction { opcode, rd, funct3, rs1, rs2, funct7 })
            }
            _ => unimplemented!("Unknown opcode: {}", opcode),
        }
    }
    
}

impl Instruction {
    pub fn decode_instruction_funct3_funct7(instr: BaseInstruction) -> Instruction {
        match instr {
            BaseInstruction::RType(r) => match (r.funct3, r.funct7) {
                (FUNCT3_ADD, FUNCT7_ALL) => Instruction::Add(r),
                (FUNCT3_SUB, FUNCT7_SUB) => Instruction::Sub(r),
                (FUNCT3_SLL, FUNCT7_ALL) => Instruction::Sll(r),
                (FUNCT3_SLT, FUNCT7_ALL) => Instruction::Slt(r),
                (FUNCT3_SLTU, FUNCT7_ALL) => Instruction::Sltu(r),
                (FUNCT3_XOR, FUNCT7_ALL) => Instruction::Xor(r),
                (FUNCT3_SRL, FUNCT7_ALL) => Instruction::Srl(r),
                (FUNCT3_SRA, FUNCT7_SRA) => Instruction::Sra(r),
                (FUNCT3_OR, FUNCT7_ALL) => Instruction::Or(r),
                (FUNCT3_AND, FUNCT7_ALL) => Instruction::And(r),
                _ => Instruction::Unknown,
            },
            BaseInstruction::IType(i) => 
            match i.opcode {
                // | OPCODE_SLTI | OPCODE_SLTIU | OPCODE_XORI | OPCODE_ORI | OPCODE_ANDI | OPCODE_SLLI | OPCODE_SRLI | OPCODE_SRAI
                OPCODE_ADDI =>
                    match i.funct3 {
                        FUNCT3_ADD => Instruction::Addi(i),
                        FUNCT3_SLL => Instruction::Slli(i),
                        FUNCT3_SLT => Instruction::Slti(i),
                        FUNCT3_SLTU => Instruction::Sltiu(i),
                        FUNCT3_XOR => Instruction::Xori(i),
                        FUNCT3_OR => Instruction::Ori(i),
                        FUNCT3_AND => Instruction::Andi(i),
                        OPCODE_JALR => Instruction::Jalr(i),
                        _ => Instruction::Unknown,
                    },
                // | OPCODE_LH | OPCODE_LW | OPCODE_LBU | OPCODE_LHU
                OPCODE_LB =>
                    match i.funct3 {
                        FUNCT3_LB => Instruction::Lb(i),
                        FUNCT3_LH => Instruction::Lh(i),
                        FUNCT3_LW => Instruction::Lw(i),
                        FUNCT3_LBU => Instruction::Lbu(i),
                        FUNCT3_LHU => Instruction::Lhu(i),
                        _ => Instruction::Unknown,
                    },
                OPCODE_JALR => Instruction::Jalr(i),
                _ => Instruction::Unknown,    
            },

            BaseInstruction::SType(s) => match s.funct3 {
                FUNCT3_SB => Instruction::Sb(s),
                FUNCT3_SH => Instruction::Sh(s),
                FUNCT3_SW => Instruction::Sw(s),
                _ => Instruction::Unknown,
            },

            BaseInstruction::BType(b) => match b.funct3 {
                FUNCT3_BEQ => Instruction::Beq(b),
                FUNCT3_BNE => Instruction::Bne(b),
                FUNCT3_BLT => Instruction::Blt(b),
                FUNCT3_BGE => Instruction::Bge(b),
                FUNCT3_BLTU => Instruction::Bltu(b),
                FUNCT3_BGEU => Instruction::Bgeu(b),
                _ => Instruction::Unknown,
            },
            BaseInstruction::UType(u) => match u.opcode {
                OPCODE_LUI => Instruction::Lui(u),
                OPCODE_AUIPC => Instruction::Auipc(u),
                _ => Instruction::Unknown,
            },
            BaseInstruction::JType(j) => match j.opcode {
                OPCODE_JAL => Instruction::Jal(j),
                _ => Instruction::Unknown,
            },
        }
    }
    

            
}
