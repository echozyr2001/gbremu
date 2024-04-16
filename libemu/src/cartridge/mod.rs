pub mod header;
pub mod mbc0;

pub trait MemoryBankController {
  fn read_rom(&self, addr: u16) -> u8;
  fn read_ram(&self, addr: u16) -> u8;

  fn write_ram(&mut self, addr: u16, val: u8);

  fn rom_title(&self) -> String {
    const TITLE_START: u16 = 0x134;
    const CGB_FLAG: u16 = 0x143;

    let title_size = match self.read_rom(CGB_FLAG) & 0x80 {
      0x80 => 11,
      _ => 16,
    };

    let mut result = String::with_capacity(title_size as usize);

    for i in 0..title_size {
      match self.read_rom(TITLE_START + i) {
        0 => break,
        v => result.push(v as char),
      }
    }

    result
  }
}
