use crate::generic::{device::Device, pcb::Board};

use self::cpu::Cpu;

use super::noc::NoC;

mod boot;
mod cpu;

#[derive(Debug)]
pub struct SoC {
  // apu: Apu,
  pub cpu: Cpu,
  // ppu: ppu,
  //
  // Memory
  // boot: Boot,
  // hram: Shared<HRam>,
}

impl SoC {
  pub fn new(noc: &NoC) -> Self {
    let cpu = noc.cpu().to_shared();
    Self {
      cpu: Cpu::new(cpu),
      // boot,
      // hram: Shared::new(HRam::default()),
    }
  }
}

impl Board<u16, u8> for SoC {
  fn connect(&self, _bus: &mut crate::generic::bus::Bus<u16, u8>) {
    // self.apu.connect(bus);
    // self.cpu.connect(bus);
    // self.ppu.connect(bus);

    // bus.map(0xff80..=0xfffe, harm);
    // self.boot.connect(bus);
  }
}
