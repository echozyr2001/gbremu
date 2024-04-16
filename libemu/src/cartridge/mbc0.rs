use super::MemoryBankController;

pub struct MBC0 {
  rom: Vec<u8>,
}

impl MemoryBankController for MBC0 {
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

#[cfg(test)]
mod tests {
  use super::*;
  use crate::cartridge::MemoryBankController;
  use dotenv::dotenv;
  use std::{env, fs::File, io::Read};

  #[test]
  fn test_mbc0() {
    dotenv().ok();
    let filepath = env::var("ROM_PATH").expect("ROM_PATH must be set");
    let mut rom = vec![];

    let file = File::open(filepath);
    match file.and_then(|mut f| f.read_to_end(&mut rom)) {
      Ok(_) => {},
      Err(e) => assert!(false, "Error reading file: {}", e),
    };

    let mbc0 = MBC0 { rom: rom };
    println!("{:?}", mbc0.rom_title());
    assert_eq!(mbc0.rom_title(), "YUGIOUDM2");
  }
}
