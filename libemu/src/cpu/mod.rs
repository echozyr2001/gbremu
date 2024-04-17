use crate::mmu::MMU;

use self::{instructions::InstType, register::Register};

pub mod instructions;
pub mod register;

pub struct CPU {
  regs: Register,
  halted: bool,
  /// Memory Management Unit
  mmu: MMU,
  /// Interrupt Master Enable
  _ime: bool,
}

impl CPU {
  /// Fetch, decode, and execute the next instruction
  /// Returns the number of ticks
  pub fn step(&mut self) -> u8 {
    if self.halted {
      // Check for interrupts
      // If an interrupt is found, unhalt the CPU
      // and jump to the interrupt handler
    }

    // Fetch the next instruction
    let opcode = self.fetch();

    // Decode the instruction
    let inst = self.decode(opcode);

    //  Execute the instruction
    self.execute(inst);

    unimplemented!()
  }

  fn fetch(&mut self) -> u8 {
    // Read the byte at the program counter
    let pc = self.regs.get_pc();
    let opcode = self.mmu.read(pc);
    // Increment the program counter
    self.regs.set_pc(self.regs.get_pc() + 1);
    opcode
  }

  fn decode(&self, _op: u8) -> InstType {
    todo!()
  }

  fn execute(&self, _inst: InstType) {
    todo!()
  }
}
