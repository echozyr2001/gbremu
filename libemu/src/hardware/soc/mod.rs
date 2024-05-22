use crate::generic::{device::Device, memory::ram::Ram, pcb::Board, share::Shared};

use self::{cpu::Cpu, ppu::Ppu};

use super::noc::NoC;

mod boot;
mod cpu;
pub mod ppu;

type HRam = Ram<u8, 0x007f>;

// #[derive(Debug)]
pub struct SoC {
  // apu: Apu,
  pub cpu: Cpu,
  pub ppu: Ppu,
  //
  // Memory
  // boot: Boot,
  hram: Shared<HRam>,
}

impl SoC {
  pub fn new(noc: &NoC) -> Self {
    let cpu = noc.cpu().to_shared();
    Self {
      cpu: Cpu::new(cpu),
      ppu: Ppu::new(),
      // boot,
      hram: Shared::new(HRam::default()),
    }
  }
}

impl Board<u16, u8> for SoC {
  fn connect(&self, bus: &mut crate::generic::bus::Bus<u16, u8>) {
    let hram = self.hram.clone().to_dynamic();
    // self.cpu.connect(bus);
    self.ppu.connect(bus);

    bus.map(0xff80..=0xfffe, hram);
    // self.boot.connect(bus);
  }
}
