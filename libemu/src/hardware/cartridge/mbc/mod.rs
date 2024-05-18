#![allow(dead_code)]

use std::fmt::{Debug, Display};

use crate::{error::CartError, generic::device::Dynamic};

mod mbc0;
mod mbc1;
pub use mbc0::MBC0;
pub use mbc1::MBC1;

/// MBC interface
pub trait Mbc: Debug {
  /// Gets a shared reference to the ROM.
  fn rom(&self) -> Dynamic<u16, u8>;

  /// Gets a shared reference to the RAM.
  fn ram(&self) -> Dynamic<u16, u8>;
}

#[derive(Debug)]
pub enum Kind {
  RomOnly,
  MBC1,
  MBC1RAM,
  MBC1RAMBATT,
  MBC2,
  MBC2BATT,
  RomRam,
  RomRamBATT,
  MMM01,
  MMM01RAM,
  MMM01RAMBATT,
  MBC3TIMERBATT,
  MBC3TIMERRAMBATT,
  MBC3,
  MBC3RAM,
  MBC3RAMBATT,
  MBC5,
  MBC5RAM,
  MBC5RAMBATT,
  MBC5RUMBLE,
  MBC5RUMBLERAM,
  MBC5RUMBLERAMBATT,
  MBC6,
  MBC7SENSORRUMBLERAMBATT,
  PocketCamera,
  BANDAITAMA5,
  HUC3,
  HUC1RAMBATT,
}

impl Default for Kind {
  fn default() -> Self {
    Self::RomOnly
  }
}

impl Display for Kind {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::RomOnly => write!(f, "ROM ONLY"),
      Self::MBC1 => write!(f, "MBC1"),
      Self::MBC1RAM => write!(f, "MBC1 + RAM"),
      Self::MBC1RAMBATT => write!(f, "MBC1 + RAM + BATTERY"),
      Self::MBC2 => write!(f, "MBC2"),
      Self::MBC2BATT => write!(f, "MBC2 + BATTERY"),
      Self::RomRam => write!(f, "ROM + RAM"),
      Self::RomRamBATT => write!(f, "ROM + RAM + BATTERY"),
      Self::MMM01 => write!(f, "MMM01"),
      Self::MMM01RAM => write!(f, "MMM01 + RAM"),
      Self::MMM01RAMBATT => write!(f, "MMM01 + RAM + BATTERY"),
      Self::MBC3TIMERBATT => write!(f, "MBC3 + TIMER + BATTERY"),
      Self::MBC3TIMERRAMBATT => write!(f, "MBC3 + TIMER + RAM + BATTERY"),
      Self::MBC3 => write!(f, "MBC3"),
      Self::MBC3RAM => write!(f, "MBC3 + RAM"),
      Self::MBC3RAMBATT => write!(f, "MBC3 + RAM + BATTERY"),
      Self::MBC5 => write!(f, "MBC5"),
      Self::MBC5RAM => write!(f, "MBC5 + RAM"),
      Self::MBC5RAMBATT => write!(f, "MBC5 + RAM + BATTERY"),
      Self::MBC5RUMBLE => write!(f, "MBC5 + RUMBLE"),
      Self::MBC5RUMBLERAM => write!(f, "MBC5 + RUMBLE + RAM"),
      Self::MBC5RUMBLERAMBATT => write!(f, "MBC5 + RUMBLE + RAM + BATTERY"),
      Self::MBC6 => write!(f, "MBC6"),
      Self::MBC7SENSORRUMBLERAMBATT => write!(f, "MBC7 + SENSOR + RUMBLE + RAM + BATTERY"),
      Self::PocketCamera => write!(f, "POCKET CAMERA"),
      Self::BANDAITAMA5 => write!(f, "BANDAI TAMA5"),
      Self::HUC3 => write!(f, "HUC3"),
      Self::HUC1RAMBATT => write!(f, "HUC1 + RAM + BATTERY"),
    }
  }
}

impl TryFrom<u8> for Kind {
  type Error = CartError;

  fn try_from(value: u8) -> Result<Self, Self::Error> {
    match value {
      0x00 => Ok(Self::RomOnly),
      0x01 => Ok(Self::MBC1),
      0x02 => Ok(Self::MBC1RAM),
      0x03 => Ok(Self::MBC1RAMBATT),
      0x05 => Ok(Self::MBC2),
      0x06 => Ok(Self::MBC2BATT),
      0x08 => Ok(Self::RomRam),
      0x09 => Ok(Self::RomRamBATT),
      0x0b => Ok(Self::MMM01),
      0x0c => Ok(Self::MMM01RAM),
      0x0d => Ok(Self::MMM01RAMBATT),
      0x0f => Ok(Self::MBC3TIMERBATT),
      0x10 => Ok(Self::MBC3TIMERRAMBATT),
      0x11 => Ok(Self::MBC3),
      0x12 => Ok(Self::MBC3RAM),
      0x13 => Ok(Self::MBC3RAMBATT),
      0x19 => Ok(Self::MBC5),
      0x1a => Ok(Self::MBC5RAM),
      0x1b => Ok(Self::MBC5RAMBATT),
      0x1c => Ok(Self::MBC5RUMBLE),
      0x1d => Ok(Self::MBC5RUMBLERAM),
      0x1e => Ok(Self::MBC5RUMBLERAMBATT),
      0x20 => Ok(Self::MBC6),
      0x22 => Ok(Self::MBC7SENSORRUMBLERAMBATT),
      0xfc => Ok(Self::PocketCamera),
      0xfd => Ok(Self::BANDAITAMA5),
      0xfe => Ok(Self::HUC3),
      0xff => Ok(Self::HUC1RAMBATT),
      _ => Err(CartError::Kind(value)),
    }
  }
}
