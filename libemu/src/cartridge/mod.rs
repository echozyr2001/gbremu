mod header;
mod licensee;
mod mbc;

use core::fmt;
use std::{
  convert::TryFrom,
  fmt::{Display, Formatter},
};

use log::debug;

use crate::{
  error::Error,
  generic::{
    address::Address,
    device::Device,
    memory::{Ram, Rom},
  },
};

use self::{
  header::Header,
  mbc::{Mbc, MBC1, NO_MBC},
};

pub const ROM_BANK_SIZE: usize = 16384;
pub const RAM_BANK_SIZE: usize = 8192;

const LOGO: [u8; 0x30] = [
  0xce, 0xed, 0x66, 0x66, 0xcc, 0x0d, 0x00, 0x0b, 0x03, 0x73, 0x00, 0x83, 0x00, 0x0c, 0x00, 0x0d,
  0x00, 0x08, 0x11, 0x1f, 0x88, 0x89, 0x00, 0x0e, 0xdc, 0xcc, 0x6e, 0xe6, 0xdd, 0xdd, 0xd9, 0x99,
  0xbb, 0xbb, 0x67, 0x63, 0x6e, 0x0e, 0xec, 0xcc, 0xdd, 0xdc, 0x99, 0x9f, 0xbb, 0xb9, 0x33, 0x3e,
];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum MbcType {
  NoMbc = 0x00,
  Mbc1 = 0x01,
  Mbc2 = 0x02,
  Mbc3 = 0x03,
  Mbc5 = 0x04,
  Mbc6 = 0x05,
  Mbc7 = 0x06,
  Unknown = 0x07,
}

impl MbcType {
  pub fn ram_bank_mask(&self) -> u8 {
    match self {
      MbcType::NoMbc => 0x00,
      MbcType::Mbc1 => 0x03,
      MbcType::Mbc2 => unimplemented!(),
      MbcType::Mbc3 => unimplemented!(),
      MbcType::Mbc5 => unimplemented!(),
      MbcType::Mbc6 => unimplemented!(),
      MbcType::Mbc7 => unimplemented!(),
      MbcType::Unknown => unimplemented!(),
    }
  }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CartType {
  RomOnly = 0x00,
  Mbc1 = 0x01,
  Mbc1Ram = 0x02,
  Mbc1RamBattery = 0x03,
  Mbc2 = 0x05,
  Mbc2Battery = 0x06,
  RomRam = 0x08,
  RomRamBattery = 0x09,
  Mmm01 = 0x0b,
  Mmm01Ram = 0x0c,
  Mmm01RamBattery = 0x0d,
  Mbc3TimerBattery = 0x0f,
  Mbc3TimerRamBattery = 0x10,
  Mbc3 = 0x11,
  Mbc3Ram = 0x12,
  Mbc3RamBattery = 0x13,
  Unknown = 0xef,
}

impl TryFrom<u8> for CartType {
  type Error = Error;

  fn try_from(value: u8) -> Result<Self, Self::Error> {
    match value {
      0x00 => Ok(CartType::RomOnly),
      0x01 => Ok(CartType::Mbc1),
      0x02 => Ok(CartType::Mbc1Ram),
      0x03 => Ok(CartType::Mbc1RamBattery),
      0x05 => Ok(CartType::Mbc2),
      0x06 => Ok(CartType::Mbc2Battery),
      0x08 => Ok(CartType::RomRam),
      0x09 => Ok(CartType::RomRamBattery),
      0x0b => Ok(CartType::Mmm01),
      0x0c => Ok(CartType::Mmm01Ram),
      0x0d => Ok(CartType::Mmm01RamBattery),
      0x0f => Ok(CartType::Mbc3TimerBattery),
      0x10 => Ok(CartType::Mbc3TimerRamBattery),
      0x11 => Ok(CartType::Mbc3),
      0x12 => Ok(CartType::Mbc3Ram),
      0x13 => Ok(CartType::Mbc3RamBattery),
      _ => Err(Error::CustomError("Unknown CartType".to_string())),
    }
  }
}

impl CartType {
  pub fn description(&self) -> &'static str {
    match self {
      CartType::RomOnly => "ROM Only",
      CartType::Mbc1 => "MBC1",
      CartType::Mbc1Ram => "MBC1 + RAM",
      CartType::Mbc1RamBattery => "MBC1 + RAM + Battery",
      CartType::Mbc2 => "MBC2",
      CartType::Mbc2Battery => "MBC2 + RAM",
      CartType::RomRam => "ROM + RAM",
      CartType::RomRamBattery => "ROM + RAM + BATTERY",
      CartType::Mmm01 => "MMM01",
      CartType::Mmm01Ram => "MMM01 + RAM",
      CartType::Mmm01RamBattery => "MMM01 + RAM + BATTERY",
      CartType::Mbc3TimerBattery => "MBC3 + TIMER + BATTERY",
      CartType::Mbc3TimerRamBattery => "MBC3 + TIMER + RAM + BATTERY",
      CartType::Mbc3 => "MBC3",
      CartType::Mbc3Ram => "MBC3 + RAM",
      CartType::Mbc3RamBattery => "MBC3 + RAM + BATTERY",
      CartType::Unknown => "Unknown",
    }
  }

  pub fn mbc_type(&self) -> MbcType {
    match self {
      CartType::RomOnly => MbcType::NoMbc,
      CartType::Mbc1 | CartType::Mbc1Ram | CartType::Mbc1RamBattery => MbcType::Mbc1,
      CartType::Mbc2 | CartType::Mbc2Battery => MbcType::Mbc2,
      CartType::Mbc3
      | CartType::Mbc3Ram
      | CartType::Mbc3RamBattery
      | CartType::Mbc3TimerBattery
      | CartType::Mbc3TimerRamBattery => MbcType::Mbc3,
      _ => MbcType::Unknown,
    }
  }
}

impl Display for CartType {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.description())
  }
}

pub enum RomSize {
  Size32K,
  Size64K,
  Size128K,
  Size256K,
  Size512K,
  Size1M,
  Size2M,
  Size4M,
  Size8M,
  SizeUnknown,
}

impl RomSize {
  pub fn description(&self) -> &'static str {
    match self {
      RomSize::Size32K => "32 KB",
      RomSize::Size64K => "64 KB",
      RomSize::Size128K => "128 KB",
      RomSize::Size256K => "256 KB",
      RomSize::Size512K => "512 KB",
      RomSize::Size1M => "1 MB",
      RomSize::Size2M => "2 MB",
      RomSize::Size4M => "4 MB",
      RomSize::Size8M => "8 MB",
      RomSize::SizeUnknown => "Unknown",
    }
  }

  pub fn rom_banks(&self) -> u16 {
    match self {
      RomSize::Size32K => 2,
      RomSize::Size64K => 4,
      RomSize::Size128K => 8,
      RomSize::Size256K => 16,
      RomSize::Size512K => 32,
      RomSize::Size1M => 64,
      RomSize::Size2M => 128,
      RomSize::Size4M => 256,
      RomSize::Size8M => 512,
      RomSize::SizeUnknown => 0,
    }
  }
}

impl Display for RomSize {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.description())
  }
}

pub enum RamSize {
  NoRam,
  Unused,
  Size8K,
  Size16K,
  Size32K,
  Size64K,
  Size128K,
  SizeUnknown,
}

impl RamSize {
  pub fn description(&self) -> &'static str {
    match self {
      RamSize::NoRam => "No RAM",
      RamSize::Unused => "Unused",
      RamSize::Size8K => "8 KB",
      RamSize::Size16K => "16 KB",
      RamSize::Size32K => "32 KB",
      RamSize::Size128K => "128 KB",
      RamSize::Size64K => "64 KB",
      RamSize::SizeUnknown => "Unknown",
    }
  }

  pub fn ram_banks(&self) -> u16 {
    match self {
      RamSize::NoRam => 0,
      RamSize::Unused => 0,
      RamSize::Size8K => 1,
      RamSize::Size16K => 2,
      RamSize::Size32K => 4,
      RamSize::Size64K => 8,
      RamSize::Size128K => 16,
      RamSize::SizeUnknown => 0,
    }
  }
}

impl Display for RamSize {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.description())
  }
}

pub struct Cartridge {
  rom: Rom,
  ram: Ram,
  mbc: &'static Mbc,
  rom_bank_count: u16,
  rom_offset: usize,
  ram_enabled: bool,

  header: Header,
}

impl Cartridge {
  pub fn new() -> Self {
    Self {
      rom: Rom::new(),
      ram: Ram::new(),
      mbc: &NO_MBC,
      rom_bank_count: 0,
      rom_offset: 0x4000,
      ram_enabled: false,
      header: Header::default(),
    }
  }

  pub fn from_data(data: &[u8]) -> Result<Self, Error> {
    let mut cartridge = Cartridge::new();
    cartridge.set_data(data)?;
    Ok(cartridge)
  }

  pub fn reset(&mut self) {
    self.rom = Rom::new();
    self.ram = Ram::new();
    self.mbc = &NO_MBC;
    self.rom_bank_count = 0;
    self.rom_offset = 0x4000;
    self.ram_enabled = false;
  }

  pub fn set_cart_type(&mut self, rom_type: CartType) -> Result<(), Error> {
    self.rom.write(
      0x0147,
      match rom_type {
        CartType::RomOnly => 0x00,
        CartType::Mbc1 => 0x01,
        CartType::Mbc1Ram => 0x02,
        CartType::Mbc1RamBattery => 0x03,
        CartType::Mbc2 => 0x05,
        CartType::Mbc2Battery => 0x06,
        CartType::RomRam => 0x08,
        CartType::RomRamBattery => 0x09,
        CartType::Mmm01 => 0x0b,
        CartType::Mmm01Ram => 0x0c,
        CartType::Mmm01RamBattery => 0x0d,
        CartType::Mbc3TimerBattery => 0x0f,
        CartType::Mbc3TimerRamBattery => 0x10,
        CartType::Mbc3 => 0x11,
        CartType::Mbc3Ram => 0x12,
        CartType::Mbc3RamBattery => 0x13,
        CartType::Unknown => return Err(Error::CustomError(String::from("Unknown ROM type"))),
      },
    );
    Ok(())
  }

  pub fn mbc(&self) -> Result<&'static Mbc, Error> {
    Ok(match self.cart_type() {
      CartType::RomOnly => &NO_MBC,
      CartType::Mbc1 => &MBC1,
      CartType::Mbc1Ram => &MBC1,
      CartType::Mbc1RamBattery => &MBC1,
      CartType::Mbc3TimerBattery => unimplemented!(),
      CartType::Mbc3TimerRamBattery => unimplemented!(),
      CartType::Mbc3 => unimplemented!(),
      CartType::Mbc3Ram => unimplemented!(),
      CartType::Mbc3RamBattery => unimplemented!(),
      rom_type => {
        return Err(Error::CustomError(format!(
          "No MBC controller available for {}",
          rom_type
        )))
      },
    })
  }

  pub fn ram_enabled(&self) -> bool {
    self.ram_enabled
  }

  pub fn set_ram_enabled(&mut self, ram_enabled: bool) {
    self.ram_enabled = ram_enabled
  }

  pub fn rom_bank(&self) -> u16 {
    (self.rom_offset / ROM_BANK_SIZE) as u16
  }

  pub fn set_rom_bank(&mut self, rom_bank: u16) {
    self.rom_offset = rom_bank as usize * ROM_BANK_SIZE;
  }

  fn set_data(&mut self, data: &[u8]) -> Result<(), Error> {
    self.ensure_data(data)?;
    self.rom.set_data(data);
    self.set_header(Header::parse(data)?);
    self.rom_offset = 0x4000;
    self.set_mbc()?;
    self.set_computed();
    self.set_rom_bank(1);
    Ok(())
  }

  fn set_header(&mut self, header: Header) {
    self.header = header;
  }

  fn set_mbc(&mut self) -> Result<(), Error> {
    self.mbc = self.mbc()?;
    Ok(())
  }

  fn set_computed(&mut self) {
    self.rom_bank_count = self.rom_size().rom_banks();
  }

  fn ensure_data(&self, data: &[u8]) -> Result<(), Error> {
    if data.len() < 0x7fff {
      return Err(Error::RomSize);
    }
    if data.len() % (16 * 1024) != 0 {
      return Err(Error::RomSize);
    }
    Ok(())
  }
}

impl Cartridge {
  pub fn header(&self) -> &Header {
    &self.header
  }

  pub fn title(&self) -> String {
    self.header.title.clone().unwrap_or_default()
  }

  pub fn cart_type(&self) -> CartType {
    match self.rom.read(0x0147) {
      0x00 => CartType::RomOnly,
      0x01 => CartType::Mbc1,
      0x02 => CartType::Mbc1Ram,
      0x03 => CartType::Mbc1RamBattery,
      0x05 => CartType::Mbc2,
      0x06 => CartType::Mbc2Battery,
      0x08 => CartType::RomRam,
      0x09 => CartType::RomRamBattery,
      0x0b => CartType::Mmm01,
      0x0c => CartType::Mmm01Ram,
      0x0d => CartType::Mmm01RamBattery,
      0x0f => CartType::Mbc3TimerBattery,
      0x10 => CartType::Mbc3TimerRamBattery,
      0x11 => CartType::Mbc3,
      0x12 => CartType::Mbc3Ram,
      0x13 => CartType::Mbc3RamBattery,
      _ => CartType::Unknown,
    }
  }

  pub fn rom_size(&self) -> RomSize {
    match self.rom.read(0x0148) {
      0x00 => RomSize::Size32K,
      0x01 => RomSize::Size64K,
      0x02 => RomSize::Size128K,
      0x03 => RomSize::Size256K,
      0x04 => RomSize::Size512K,
      0x05 => RomSize::Size1M,
      0x06 => RomSize::Size2M,
      0x07 => RomSize::Size4M,
      0x08 => RomSize::Size8M,
      _ => RomSize::SizeUnknown,
    }
  }

  pub fn ram_size(&self) -> RamSize {
    match self.rom.read(0x0149) {
      0x00 => RamSize::NoRam,
      0x01 => RamSize::Unused,
      0x02 => RamSize::Size8K,
      0x03 => RamSize::Size32K,
      0x04 => RamSize::Size128K,
      0x05 => RamSize::Size64K,
      _ => RamSize::SizeUnknown,
    }
  }

  pub fn has_battery(&self) -> bool {
    matches!(
      self.cart_type(),
      CartType::Mbc1RamBattery
        | CartType::Mbc2Battery
        | CartType::RomRamBattery
        | CartType::Mmm01RamBattery
        | CartType::Mbc3TimerBattery
        | CartType::Mbc3TimerRamBattery
        | CartType::Mbc3RamBattery
    )
  }

  pub fn set_ram_data(&mut self, data: &[u8]) {
    self.ram = Ram::from(data)
  }
}

impl Cartridge {
  pub fn rom(&self) -> &Rom {
    &self.rom
  }

  pub fn rom_mut(&mut self) -> &mut Rom {
    &mut self.rom
  }

  pub fn ram(&self) -> &Ram {
    &self.ram
  }
}

impl Default for Cartridge {
  fn default() -> Self {
    Self::new()
  }
}

impl Address for Cartridge {
  fn read(&self, addr: u16) -> u8 {
    match addr & 0xf000 {
      0x0000 | 0x1000 | 0x2000 | 0x3000 | 0x4000 | 0x5000 | 0x6000 | 0x7000 => {
        (self.mbc.read_rom)(self, addr)
      },
      0xa000 | 0xb000 => (self.mbc.read_ram)(self, addr),
      _ => {
        debug!("Reading from unknown Cartridge control 0x{:04x}", addr);
        0x00
      },
    }
  }

  fn write(&mut self, addr: u16, value: u8) {
    match addr & 0xf000 {
      0x0000 | 0x1000 | 0x2000 | 0x3000 | 0x4000 | 0x5000 | 0x6000 | 0x7000 => {
        (self.mbc.write_rom)(self, addr, value)
      },
      0xa000 | 0xb000 => (self.mbc.write_ram)(self, addr, value),
      _ => debug!("Writing to unknown Cartridge address 0x{:04x}", addr),
    }
  }
}

impl Device for Cartridge {}
