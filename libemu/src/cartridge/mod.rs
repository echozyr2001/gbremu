use self::header::{parse_cart_header, CartridgeHeader};

pub mod header;
pub mod mbc0;

pub trait MemoryBankController {
  fn new(rom: Vec<u8>) -> Self;
  fn rom(&self) -> &[u8];

  fn read_rom(&self, addr: u16) -> u8;
  fn read_ram(&self, addr: u16) -> u8;

  fn write_ram(&mut self, addr: u16, val: u8);

  fn get_header(&self) -> CartridgeHeader {
    parse_cart_header(self.rom()).unwrap().1
  }
}

pub struct Cartridge<MBC: MemoryBankController> {
  mbc: MBC,
}

impl<MBC: MemoryBankController> Cartridge<MBC> {
  pub fn new(rom: Vec<u8>) -> Self {
    Self { mbc: MBC::new(rom) }
  }

  pub fn get_header(&self) -> CartridgeHeader {
    self.mbc.get_header()
  }
}
