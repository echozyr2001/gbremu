use log::warn;

use crate::generic::{address::Address, device::Device};

pub struct Dma {
  value_dma: u8,
  cycles_dma: u16,
  active_dma: bool,
}

impl Dma {
  pub fn new() -> Self {
    Self {
      value_dma: 0x0,
      cycles_dma: 0x0,
      active_dma: false,
    }
  }

  pub fn reset(&mut self) {
    self.value_dma = 0x0;
    self.cycles_dma = 0x0;
    self.active_dma = false;
  }

  pub fn value_dma(&self) -> u8 {
    self.value_dma
  }

  pub fn set_value_dma(&mut self, value: u8) {
    self.value_dma = value;
  }

  pub fn cycles_dma(&self) -> u16 {
    self.cycles_dma
  }

  pub fn set_cycles_dma(&mut self, value: u16) {
    self.cycles_dma = value;
  }

  pub fn active_dma(&self) -> bool {
    self.active_dma
  }

  pub fn set_active_dma(&mut self, value: bool) {
    self.active_dma = value;
  }

  pub fn active(&self) -> bool {
    self.active_dma
  }
}

impl Default for Dma {
  fn default() -> Self {
    Self::new()
  }
}

impl Address for Dma {
  fn read(&self, addr: u16) -> u8 {
    match addr {
      // 0xFF46 — DMA: OAM DMA source address & start
      0xFF46 => self.value_dma,
      _ => {
        warn!("Reading from unknown DMA location 0x{:04x}", addr);
        0xff
      },
    }
  }

  fn write(&mut self, addr: u16, value: u8) {
    match addr {
      // 0xFF46 — DMA: OAM DMA source address & start
      0xFF46 => {
        self.value_dma = value;
        self.cycles_dma = 640;
        self.active_dma = true;
      },
      _ => warn!("Writing to unknown DMA location 0x{:04x}", addr),
    }
  }
}

impl Device for Dma {}
