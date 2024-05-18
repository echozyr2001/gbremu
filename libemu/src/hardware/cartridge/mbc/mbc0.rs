use crate::generic::{
  device::{Device, Dynamic},
  share::Shared,
};

use super::Mbc;

#[derive(Debug)]
pub struct MBC0 {
  rom: Shared<Dynamic<u16, u8>>,
}

#[allow(dead_code)]
impl MBC0 {
  pub fn with(rom: Dynamic<u16, u8>) -> Self {
    Self {
      rom: Shared::new(rom),
    }
  }
}

impl Mbc for MBC0 {
  fn rom(&self) -> Dynamic<u16, u8> {
    todo!()
    // self.rom.clone().to_dynamic()
  }

  fn ram(&self) -> Dynamic<u16, u8> {
    todo!()
  }
}
