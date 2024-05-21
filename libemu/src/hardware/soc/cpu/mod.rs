#![allow(dead_code)]

use log::trace;

use crate::{
  generic::{arch::address::Address, share::Shared},
  hardware::{pic::Pic, soc::cpu::instructions::INSTRUCTIONS, Bus},
};

use self::{instructions::Instruction, register::Register};

pub mod instructions;
pub mod register;

#[derive(Debug)]
pub struct Cpu {
  // state
  stage: Stages,
  statue: Status,
  ime: Ime,
  halted: bool,
  // registers
  regs: Register,
  bus: Shared<Bus>,
  // pic: Shared<Pic>,
}

impl Cpu {
  /// Constructs a new `Cpu`.
  pub fn new(bus: Shared<Bus>) -> Self {
    // let bus = Bus::new();
    // bus.map(0x0000..=0xffff, dev);
    Self {
      stage: Stages::default(),
      statue: Status::default(),
      ime: Ime::default(),
      halted: bool::default(),

      regs: Register::default(),
      bus,
      // pic,
    }
  }

  /// Read byte at addr.
  pub fn read(&self, addr: u16) -> u8 {
    self.bus.read(addr)
  }

  /// Write byte to addr.
  fn write(&mut self, addr: u16, val: u8) {
    self.bus.write(addr, val);
  }

  /// Fetch the next byte after pc.
  fn fetch_byte(&mut self) -> u8 {
    // Load pc
    let pc = self.regs.get_pc();
    // Read byte at pc
    let byte = self.read(pc);
    // Increment pc
    self.regs.set_pc(self.regs.get_pc().wrapping_add(1));
    byte
  }

  /// Read the byte at hl.
  fn read_byte(&mut self) -> u8 {
    // Load hl
    let hl = self.regs.get_hl();
    // Read byte at hl
    self.read(hl)
  }

  /// Write byte to hl.
  fn write_byte(&mut self, byte: u8) {
    // Load hl
    let hl = self.regs.get_hl();
    // Write byte to hl
    self.write(hl, byte);
  }

  /// Pop byte at sp.
  fn pop_byte(&mut self) -> u8 {
    // Load sp
    let sp = self.regs.get_sp();
    // Read byte at sp
    let byte = self.read(sp);
    // Increment sp
    self.regs.set_sp(self.regs.get_sp().wrapping_add(1));
    byte
  }

  /// Push byte to sp.
  fn push_byte(&mut self, byte: u8) {
    // Decrement sp
    self.regs.set_sp(self.regs.get_sp().wrapping_sub(1));
    // Load sp
    let sp = self.regs.get_sp();
    // Write byte to sp
    self.write(sp, byte);
  }

  fn enable_interrupts(&mut self) {
    self.ime = Ime::Enabled;
  }

  fn disable_interrupts(&mut self) {
    self.ime = Ime::Disabled;
  }
}

impl Cpu {
  pub fn cycle(&mut self) {
    self.stage = std::mem::take(&mut self.stage).exec(self);
  }
}

/// CPU execution stage
#[derive(Default, Debug)]
enum Stages {
  /// Fetch the next instruction
  #[default]
  Fetch,
  /// Execute the current instruction
  Execute(Instruction),
  /// Done executing the current instruction
  Done,
}

impl Stages {
  fn exec(mut self, cpu: &mut Cpu) -> Self {
    if let Stages::Done = self {
      trace!("register:\n{}", cpu.regs);

      // let int = match cpu.ime {
      //   Ime::Enabled => cpu.pic.borrow().int(),
      //   _ => None,
      // };

      // if let Some(int) = int {
      //   cpu.pic.borrow_mut().ack(int);
      // }

      self = Stages::Fetch;
    }

    if let Stages::Fetch = self {
      let pc = cpu.regs.get_pc();
      let opcode = cpu.fetch_byte();

      trace!("pc: {:#06x}, opcode: {:#04x}", pc, opcode);

      let inst = INSTRUCTIONS[opcode as usize];

      self = Stages::Execute(inst);
    }

    if let Stages::Execute(inst) = self {
      let inst = inst.exec(cpu);
      self = Stages::Done;
    }

    self
  }
}

/// CPU status
#[derive(Default, Debug)]
enum Status {
  /// Enable, normal execution
  #[default]
  Enable,
  /// Halted, waiting for an interrupt
  Halted,
  /// Stopped, very low-power mode
  Stopped,
}

/// CPU Interrupt Master Enable
#[derive(Default, Debug)]
enum Ime {
  #[default]
  Disabled,
  Enabled,
  WillEnable,
}
