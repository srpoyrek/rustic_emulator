
pub mod opcode {

    // Opcode Definitions
    pub const OPCODE_LUI: u8 = 0b0110111;   // LUI (Load Upper Immediate)
    pub const OPCODE_AUIPC: u8 = 0b0010111; // AUIPC (Add Upper Immediate to PC)
    pub const OPCODE_JAL: u8 = 0b1101111;   // JAL (Jump and Link)
    pub const OPCODE_JALR: u8 = 0b1100111;  // JALR (Jump and Link Register)
    pub const OPCODE_BEQ: u8 = 0b1100011;   // BEQ (Branch if Equal)
    pub const OPCODE_BNE: u8 = 0b1100011;   // BNE (Branch if Not Equal)
    pub const OPCODE_BLT: u8 = 0b1100011;   // BLT (Branch if Less Than)
    pub const OPCODE_BGE: u8 = 0b1100011;   // BGE (Branch if Greater or Equal)
    pub const OPCODE_BLTU: u8 = 0b1100011;  // BLTU (Branch if Less Than Unsigned)
    pub const OPCODE_BGEU: u8 = 0b1100011;  // BGEU (Branch if Greater or Equal Unsigned)
    pub const OPCODE_LB: u8 = 0b0000011;    // LB (Load Byte)
    pub const OPCODE_LH: u8 = 0b0000011;    // LH (Load Halfword)
    pub const OPCODE_LW: u8 = 0b0000011;    // LW (Load Word)
    pub const OPCODE_LBU: u8 = 0b0000011;   // LBU (Load Byte Unsigned)
    pub const OPCODE_LHU: u8 = 0b0000011;   // LHU (Load Halfword Unsigned)
    pub const OPCODE_SB: u8 = 0b0100011;    // SB (Store Byte)
    pub const OPCODE_SH: u8 = 0b0100011;    // SH (Store Halfword)
    pub const OPCODE_SW: u8 = 0b0100011;    // SW (Store Word)
    pub const OPCODE_ADDI: u8 = 0b0010011;  // ADDI (Add Immediate)
    pub const OPCODE_SLTI: u8 = 0b0010011;  // SLTI (Set Less Than Immediate)
    pub const OPCODE_SLTIU: u8 = 0b0010011; // SLTIU (Set Less Than Immediate Unsigned)
    pub const OPCODE_XORI: u8 = 0b0010011;  // XORI (XOR Immediate)
    pub const OPCODE_ORI: u8 = 0b0010011;   // ORI (OR Immediate)
    pub const OPCODE_ANDI: u8 = 0b0010011;  // ANDI (AND Immediate)
    pub const OPCODE_SLLI: u8 = 0b0010011;  // SLLI (Shift Left Logical Immediate)
    pub const OPCODE_SRLI: u8 = 0b0010011;  // SRLI (Shift Right Logical Immediate)
    pub const OPCODE_SRAI: u8 = 0b0010011;  // SRAI (Shift Right Arithmetic Immediate)
    pub const OPCODE_ADD: u8 = 0b0110011;   // ADD (Add)
    pub const OPCODE_SUB: u8 = 0b0110011;   // SUB (Subtract)
    pub const OPCODE_SLL: u8 = 0b0110011;   // SLL (Shift Left Logical)
    pub const OPCODE_SLT: u8 = 0b0110011;   // SLT (Set Less Than)
    pub const OPCODE_SLTU: u8 = 0b0110011;  // SLTU (Set Less Than Unsigned)
    pub const OPCODE_XOR: u8 = 0b0110011;   // XOR (XOR)
    pub const OPCODE_SRL: u8 = 0b0110011;   // SRL (Shift Right Logical)
    pub const OPCODE_SRA: u8 = 0b0110011;   // SRA (Shift Right Arithmetic)
    pub const OPCODE_OR: u8 = 0b0110011;    // OR (OR)
    pub const OPCODE_AND: u8 = 0b0110011;   // AND (AND)

    // funct3 Definitions for R-type, I-type, and S-type
    pub const FUNCT3_ADD: u8 = 0b000;  // Add
    pub const FUNCT3_SUB: u8 = 0b000;  // Subtract
    pub const FUNCT3_SLL: u8 = 0b001;  // Shift Left Logical
    pub const FUNCT3_SLT: u8 = 0b010;  // Set Less Than
    pub const FUNCT3_SLTU: u8 = 0b011; // Set Less Than Unsigned
    pub const FUNCT3_XOR: u8 = 0b100;  // XOR
    pub const FUNCT3_SRL: u8 = 0b101;  // Shift Right Logical
    pub const FUNCT3_SRA: u8 = 0b101;  // Shift Right Arithmetic
    pub const FUNCT3_OR: u8 = 0b110;   // OR
    pub const FUNCT3_AND: u8 = 0b111;  // AND

    // funct7 Definitions for R-type
    pub const FUNCT7_SUB: u8 = 0b0100000; // SUB
    pub const FUNCT7_ALL: u8 = 0b0000000; // ALL
    pub const FUNCT7_SRA: u8 = 0b0100000; // SRA
    
    // funct3 Definitions for B-type
    pub const FUNCT3_BEQ: u8 = 0b000;  // Branch if Equal
    pub const FUNCT3_BNE: u8 = 0b001;  // Branch if Not Equal
    pub const FUNCT3_BLT: u8 = 0b100;  // Branch if Less Than
    pub const FUNCT3_BGE: u8 = 0b101;  // Branch if Greater or Equal
    pub const FUNCT3_BLTU: u8 = 0b110; // Branch if Less Than Unsigned
    pub const FUNCT3_BGEU: u8 = 0b111; // Branch if Greater or Equal Unsigned

    // funct3 Definitions for Load Instructions
    pub const FUNCT3_LB: u8 = 0b000;  // Load Byte
    pub const FUNCT3_LH: u8 = 0b001;  // Load Halfword
    pub const FUNCT3_LW: u8 = 0b010;  // Load Word
    pub const FUNCT3_LBU: u8 = 0b100; // Load Byte Unsigned
    pub const FUNCT3_LHU: u8 = 0b101; // Load Halfword Unsigned

    // funct3 Definitions for Store Instructions
    pub const FUNCT3_SB: u8 = 0b000;  // Store Byte
    pub const FUNCT3_SH: u8 = 0b001;  // Store Halfword
    pub const FUNCT3_SW: u8 = 0b010;  // Store Word

}