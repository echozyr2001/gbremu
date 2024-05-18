use crate::generic::{
  arch::address::Address,
  bus::adapt::bank::Bank,
  device::{Device, Dynamic},
  memory,
  share::Shared,
};

use super::Mbc;

#[derive(Debug)]
struct Rom {
  rom_0: Shared<memory::rom::Rom<u8, 0x4000>>,
  rom_n: Shared<Bank<u16, u8>>,
}
impl Device<u16, u8> for Rom {}
impl Address<u16, u8> for Rom {
  fn read(&self, idx: u16) -> u8 {
    match idx {
      0x0000..=0x3FFF => self.rom_0.read(idx),
      0x4000..=0x7FFF => {
        let idx = idx - 0x4000;
        self.rom_n.read(idx)
      },
      _ => unreachable!(),
    }
  }

  fn write(&mut self, _idx: u16, _val: u8) {
    //     match idx {
    //       0x0000..=0x1FFF => {
    //         // Enable and disable RAM
    //         unimplemented!();
    //       },
    //       0x2000..=0x3FFF => {
    //         // Set ROM bank
    //         let mut val = val & 0x1F;
    //         val = if val == 0 { 1 } else { val };
    //         todo!()
    //       },
    //       0x4000..=0x5FFF => {
    //         // Set RAM bank
    //         let mut val = val & 0x03;
    //         todo!()
    //       },
    //       0x6000..=0x7FFF => {
    //         // Set Banking mode
    //         todo!()
    //       },
    //       0x8000..=0x9FFF => {
    //         // Write to VRAM
    //         unreachable!()
    //       },
    //       0xA000..=0xBFFF => {
    //         // Write to RAM
    //         let bank = self.ram.borrow().select;
    //         self.ram.borrow_mut().banks[bank].write(idx, val);
    //       },
    //       _ => unreachable!(),
    //     }
    todo!()
  }
}
type Ram = Bank<u16, u8>;

#[derive(Debug)]
pub struct MBC1 {
  rom: Shared<Rom>,
  ram: Shared<Ram>,
}

impl MBC1 {
  pub fn with(rom_part: (Vec<u8>, Vec<Dynamic<u16, u8>>), ram_part: Vec<Dynamic<u16, u8>>) -> Self {
    Self {
      rom: Shared::new(Rom {
        rom_0: Shared::new(memory::rom::Rom::<u8, 0x4000>::from(
          // TODO: resolve unwrap
          &*Box::<[_; 0x4000]>::try_from(rom_part.0.into_boxed_slice()).unwrap(),
        )),
        rom_n: Shared::new(Bank::from(&rom_part.1[..])),
      }),
      ram: Shared::new(Bank::from(&ram_part[..])),
    }
  }
}

impl Mbc for MBC1 {
  fn rom(&self) -> Dynamic<u16, u8> {
    self.rom.clone().to_dynamic()
  }

  fn ram(&self) -> Dynamic<u16, u8> {
    self.ram.clone().to_dynamic()
  }
}
