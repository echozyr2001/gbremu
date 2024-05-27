use log::warn;

use crate::generic::address::Address;

use super::Cartridge;

pub struct Mbc {
  pub name: &'static str,
  pub read_rom: fn(rom: &Cartridge, addr: u16) -> u8,
  pub write_rom: fn(rom: &mut Cartridge, addr: u16, value: u8),
  pub read_ram: fn(rom: &Cartridge, addr: u16) -> u8,
  pub write_ram: fn(rom: &mut Cartridge, addr: u16, value: u8),
}

pub static NO_MBC: Mbc = Mbc {
  name: "No MBC",
  read_rom: |cart: &Cartridge, addr: u16| -> u8 { cart.rom.read(addr) },
  write_rom: |_cart: &mut Cartridge, addr: u16, _value: u8| {
    panic!("Writing to unknown Cartridge ROM location 0x{:04x}", addr);
  },
  read_ram: |cart: &Cartridge, addr: u16| -> u8 { cart.ram.read(addr - 0xA000) },
  write_ram: |cart: &mut Cartridge, addr: u16, value: u8| {
    cart.ram.write(addr - 0xA000, value);
  },
};

pub static MBC1: Mbc = Mbc {
  name: "MBC1",
  read_rom: |cart: &Cartridge, addr: u16| -> u8 {
    match addr & 0xf000 {
      0x0000 | 0x1000 | 0x2000 | 0x3000 => cart.rom.read(addr),
      0x4000 | 0x5000 | 0x6000 | 0x7000 => cart.rom.read(cart.rom_offset as u16 + (addr - 0x4000)),
      _ => {
        warn!("Reading from unknown Cartridge ROM location 0x{:04x}", addr);
        0xff
      },
    }
  },
  write_rom: |cart: &mut Cartridge, addr: u16, value: u8| {
    match addr & 0xf000 {
      // RAM enabled flag
      0x0000 | 0x1000 => {
        cart.ram_enabled = (value & 0x0f) == 0x0a;
      },
      // ROM bank selection 5 lower bits
      0x2000 | 0x3000 => {
        let mut rom_bank = value as u16 & 0x1f;
        rom_bank &= cart.rom_bank_count * 2 - 1;
        if rom_bank == 0 {
          rom_bank = 1;
        }
        cart.set_rom_bank(rom_bank);
      },
      // RAM bank selection and ROM bank selection upper bits
      0x4000 | 0x5000 => {},
      // ROM mode selection
      0x6000 | 0x7000 => {
        if value == 0x1 && cart.rom_bank_count > 32 {
          unimplemented!("Advanced ROM banking mode for MBC1 is not implemented");
        }
      },
      _ => warn!("Writing to unknown Cartridge ROM location 0x{:04x}", addr),
    }
  },
  read_ram: |cart: &Cartridge, addr: u16| -> u8 {
    if !cart.ram_enabled {
      return 0xff;
    }
    cart.ram.read(addr - 0xa000)
  },
  write_ram: |rom: &mut Cartridge, addr: u16, value: u8| {
    if !rom.ram_enabled {
      warn!("Attempt to write to ERAM while write protect is active");
      return;
    }
    rom.ram.write(addr - 0xa000, value);
  },
};
