#![allow(dead_code)]
use std::cell::{Ref, RefMut};

use crate::{
  bus::Bus,
  generic::{memory::Ram, shared::Shared},
};

use self::{boot::Boot, cpu::Cpu, dma::Dma, ppu::Ppu};

pub mod boot;
pub mod cpu;
pub mod dma;
pub mod ppu;

type Hram = Ram;

pub struct Soc {
  cpu: Cpu,
  ppu: Shared<Ppu>,
  dma: Shared<Dma>,

  boot: Shared<Boot>,
  hram: Shared<Hram>,

  bus: Shared<Bus>,
}

impl Soc {
  pub fn new(bus: Shared<Bus>) -> Self {
    let boot = Shared::new(Boot::new());
    let ppu = Shared::new(Ppu::new());
    let dma = Shared::new(Dma::default());
    let hram = Shared::new(Hram::new());
    bus.borrow_mut().set_boot(boot.clone());
    bus.borrow_mut().set_ppu(ppu.clone());
    bus.borrow_mut().set_hram(hram.clone());
    bus.borrow_mut().set_dma(dma.clone());

    Self {
      cpu: Cpu::new(bus.clone()),
      ppu,
      dma,
      boot,
      hram,
      bus,
    }
  }

  pub fn reset(&mut self) {
    self.cpu.reset();
    self.ppu.borrow_mut().reset();
  }
}

impl Soc {
  pub fn ppu(&self) -> Ref<Ppu> {
    self.ppu.borrow()
  }

  pub fn ppu_mut(&mut self) -> RefMut<Ppu> {
    self.ppu.borrow_mut()
  }
}

impl Soc {
  pub fn clock_cpu(&mut self) -> u8 {
    let cycles = self.cpu.clock();
    if self.cpu.pc() == 0x00FE {
      self.boot.borrow_mut().set_active(false);
    }
    cycles
  }

  pub fn clock_ppu(&mut self, cycles: u16) {
    self.ppu_mut().clock(cycles)
  }

  pub fn clock_dma(&mut self, cycles: u16) {
    if !self.dma.borrow().active() {
      return;
    }

    if self.dma.borrow().active_dma() {
      let cycles_dma = self.dma.borrow().cycles_dma().saturating_sub(cycles);
      if cycles_dma == 0x0 {
        let data = self
          .bus
          .borrow()
          .read_many((self.dma.borrow().value_dma() as u16) << 8, 160);
        self.bus.borrow_mut().write_many(0xfe00, &data);
        self.dma.borrow_mut().set_active_dma(false);
      }
      self.dma.borrow_mut().set_cycles_dma(cycles_dma);
    }
  }
}
