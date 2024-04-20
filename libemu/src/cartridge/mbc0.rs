use super::MemoryBankController;

pub struct MBC0 {
  rom: Vec<u8>,
}

impl MemoryBankController for MBC0 {
  fn new(rom: Vec<u8>) -> Self {
    Self { rom }
  }

  fn rom(&self) -> &[u8] {
    &self.rom
  }

  fn read_rom(&self, addr: u16) -> u8 {
    self.rom[addr as usize]
  }

  // MBC0 does not have RAM
  fn read_ram(&self, _addr: u16) -> u8 {
    unreachable!()
  }

  fn write_ram(&mut self, _addr: u16, _val: u8) {
    unreachable!()
  }
}
