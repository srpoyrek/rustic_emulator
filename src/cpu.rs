use crate::riscv_sim::*;

pub struct Cpu {
    registers: [u32; 32],
    pc: u32,
}


impl Cpu {

    pub fn new() -> Self {
        // Fetch 32-bit instruction from memory at the PC location
        Cpu {
            registers: [0; 32],
            pc: 0,
        }
    }

    pub fn print_registers(&self) {
        let mut i = 0;
        for r in self.registers {
            print!("R{} : {}\t", i, r);
            i += 1;
        }
    }

    fn fetch(&self, memory: &Memory) -> u32 {
        // Fetch 32-bit instruction from memory at the PC location
        let addr: usize = self.pc as usize;
        let instruction: u32 = memory.load_word(addr);
        return instruction;
    }

    fn decode(&self, instruction: u32) -> Instruction {
        let base_inst: BaseInstruction = BaseInstruction::decode_instruction_by_opcode(instruction);
        Instruction::decode_instruction_funct3_funct7(base_inst)
    }

    fn execute(&mut self, instruction: Instruction, memory: &mut Memory) {
        match instruction {
            // U-type instructions
            Instruction::Lui(u) => {
                self.registers[u.rd as usize] = u.imm as u32;
            }
            
            Instruction::Auipc(u) => {
                self.registers[u.rd as usize] = self.pc.wrapping_add(u.imm as u32);
            }
    
            // J-type instructions
            Instruction::Jal(j) => {
                let ret_addr = self.pc.wrapping_add(4);
                self.pc = self.pc.wrapping_add(j.imm as u32);
                self.registers[j.rd as usize] = ret_addr;
            }
    
            // I-type instructions
            Instruction::Jalr(i) => {
                let ret_addr = self.pc.wrapping_add(4);
                self.pc = (self.registers[i.rs1 as usize].wrapping_add(i.imm as u32)) & !1;
                self.registers[i.rd as usize] = ret_addr;
            }
            Instruction::Lb(i) => {
                let addr = self.registers[i.rs1 as usize].wrapping_add(i.imm as u32) as usize;
                self.registers[i.rd as usize] = memory.load_byte(addr);
            }
            Instruction::Lh(i) => {
                let addr = self.registers[i.rs1 as usize].wrapping_add(i.imm as u32) as usize;
                self.registers[i.rd as usize] = memory.load_halfword(addr);
            }
            Instruction::Lw(i) => {
                let addr = self.registers[i.rs1 as usize].wrapping_add(i.imm as u32) as usize;
                self.registers[i.rd as usize] = memory.load_word(addr);
            }
            Instruction::Lbu(i) => {
                let addr = self.registers[i.rs1 as usize].wrapping_add(i.imm as u32) as usize;
                self.registers[i.rd as usize] = memory.load_byte(addr);
            }
            Instruction::Lhu(i) => {
                let addr = self.registers[i.rs1 as usize].wrapping_add(i.imm as u32) as usize;
                self.registers[i.rd as usize] = memory.load_halfword(addr);
            }
            Instruction::Addi(i) => {
                self.registers[i.rd as usize] = self.registers[i.rs1 as usize].wrapping_add(i.imm as u32);
            }
            Instruction::Slti(i) => {
                self.registers[i.rd as usize] = if (self.registers[i.rs1 as usize] as i32) < i.imm as i32 { 1 } else { 0 };
            }
            Instruction::Sltiu(i) => {
                self.registers[i.rd as usize] = if (self.registers[i.rs1 as usize] as u32) < (i.imm as u32) { 1 } else { 0 };
            }
            Instruction::Xori(i) => {
                self.registers[i.rd as usize] = self.registers[i.rs1 as usize] ^ (i.imm as u32);
            }
            Instruction::Ori(i) => {
                self.registers[i.rd as usize] = self.registers[i.rs1 as usize] | (i.imm as u32);
            }
            Instruction::Andi(i) => {
                self.registers[i.rd as usize] = self.registers[i.rs1 as usize] & (i.imm as u32);
            }
            Instruction::Slli(i) => {
                self.registers[i.rd as usize] = self.registers[i.rs1 as usize] << (i.imm & 0x1F);
            }
            Instruction::Srli(i) => {
                self.registers[i.rd as usize] = ((self.registers[i.rs1 as usize] as u32 >> (i.imm & 0x1F))) as u32;
            }
            Instruction::Srai(i) => {
                self.registers[i.rd as usize] = (self.registers[i.rs1 as usize] >> (i.imm & 0x1F)) as u32;
            }
    
            // S-type instructions
            Instruction::Sb(s) => {
                let addr = self.registers[s.rs1 as usize].wrapping_add(s.imm as u32) as usize;
                memory.store_byte(addr, self.registers[s.rs2 as usize] as u8);
            }

            Instruction::Sh(s) => {
                let addr = self.registers[s.rs1 as usize].wrapping_add(s.imm as u32) as usize;
                memory.store_halfword(addr, self.registers[s.rs2 as usize] as u16);
            }

            Instruction::Sw(s) => {
                let addr = self.registers[s.rs1 as usize].wrapping_add(s.imm as u32) as usize;
                memory.store_word(addr, self.registers[s.rs2 as usize]);
            }
    
            // B-type instructions
            Instruction::Beq(b) => {
                if self.registers[b.rs1 as usize] == self.registers[b.rs2 as usize] {
                    self.pc = self.pc.wrapping_add(b.imm as u32);
                }
            }
            
            Instruction::Bne(b) => {
                if self.registers[b.rs1 as usize] != self.registers[b.rs2 as usize] {
                    self.pc = self.pc.wrapping_add(b.imm as u32);
                }
            }
            Instruction::Blt(b) => {
                if (self.registers[b.rs1 as usize] as i32) < (self.registers[b.rs2 as usize] as i32) {
                    self.pc = self.pc.wrapping_add(b.imm as u32);
                }
            }
            Instruction::Bge(b) => {
                if (self.registers[b.rs1 as usize] as i32) >= (self.registers[b.rs2 as usize] as i32) {
                    self.pc = self.pc.wrapping_add(b.imm as u32);
                }
            }
            Instruction::Bltu(b) => {
                if (self.registers[b.rs1 as usize] as u32) < (self.registers[b.rs2 as usize] as u32) {
                    self.pc = self.pc.wrapping_add(b.imm as u32);
                }
            }
            Instruction::Bgeu(b) => {
                if (self.registers[b.rs1 as usize] as u32) >= (self.registers[b.rs2 as usize] as u32) {
                    self.pc = self.pc.wrapping_add(b.imm as u32);
                }
            }
    
            // R-type instructions
            Instruction::Add(r) => {
                self.registers[r.rd as usize] = self.registers[r.rs1 as usize].wrapping_add(self.registers[r.rs2 as usize]);
            }

            Instruction::Sub(r) => {
                self.registers[r.rd as usize] = self.registers[r.rs1 as usize].wrapping_sub(self.registers[r.rs2 as usize]);
            }
            Instruction::Sll(r) => {
                self.registers[r.rd as usize] = self.registers[r.rs1 as usize] << (self.registers[r.rs2 as usize] & 0x1F);
            }
            Instruction::Slt(r) => {
                self.registers[r.rd as usize] = if (self.registers[r.rs1 as usize] as i32) < (self.registers[r.rs2 as usize] as i32) { 1 } else { 0 };
            }
            Instruction::Sltu(r) => {
                self.registers[r.rd as usize] = if (self.registers[r.rs1 as usize] as u32) < (self.registers[r.rs2 as usize] as u32) { 1 } else { 0 };
            }
            Instruction::Xor(r) => {
                self.registers[r.rd as usize] = self.registers[r.rs1 as usize] ^ self.registers[r.rs2 as usize];
            }
            Instruction::Srl(r) => {
                self.registers[r.rd as usize] = (self.registers[r.rs1 as usize] as u32 >> (self.registers[r.rs2 as usize] & 0x1F)) as u32;
            }
            Instruction::Sra(r) => {
                self.registers[r.rd as usize] = self.registers[r.rs1 as usize] >> (self.registers[r.rs2 as usize] & 0x1F);
            }
            Instruction::Or(r) => {
                self.registers[r.rd as usize] = self.registers[r.rs1 as usize] | self.registers[r.rs2 as usize];
            }
            Instruction::And(r) => {
                self.registers[r.rd as usize] = self.registers[r.rs1 as usize] & self.registers[r.rs2 as usize];
            }
    
            // Default case for unhandled instructions
            _ => {
                eprintln!("Unknown instruction: {:?}", instruction);
            }
        }
    }
    

    pub fn run(&mut self, memory: &mut Memory) {
        loop {
            let instruction = self.fetch(memory);
            let decoded = self.decode(instruction);
            self.execute(decoded, memory);
            self.pc = self.pc.wrapping_add(4);

            // Break if necessary, e.g., upon a specific condition or instruction
            if self.pc as usize >= memory.size() {  // Example condition
                break;
            }
        }   
    }

}
