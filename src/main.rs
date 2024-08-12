use riscv_simulator::{cpu::*, memory::Memory};

fn main() {
    let mut rv32i:Cpu = Cpu::new();

    let mut mem_rv:Memory = Memory::new();
    rv32i.run(&mut mem_rv);
    rv32i.print_registers();
}


