use std::fmt::Display;

use crate::error::CartError;

mod mbc0;

pub trait MBC {}

#[derive(Debug)]
pub enum Kind {
  None { ram: bool, pwr: bool },
  MBC1 { ram: bool, pwr: bool },
  MBC2 { pwr: bool },
  MBC3 { ram: bool, pwr: bool, rtc: bool },
  MBC5 { ram: bool, pwr: bool, rtc: bool },
  MBC6,
  MBC7,
  MMM01 { ram: bool, pwr: bool },
  M161,
  HuC1,
  HuC3,
  Camera,
}

impl Default for Kind {
  fn default() -> Self {
    Self::None {
      ram: false,
      pwr: false,
    }
  }
}

impl Display for Kind {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::None { .. } => "None",
      Self::MBC1 { .. } => "MBC1",
      Self::MBC2 { .. } => "MBC2",
      Self::MBC3 { .. } => "MBC3",
      Self::MBC5 { .. } => "MBC5",
      Self::MBC6 => "MBC6",
      Self::MBC7 => "MBC7",
      Self::MMM01 { .. } => "MMM01",
      Self::M161 => "M161",
      Self::HuC1 => "HuC1",
      Self::HuC3 => "HuC3",
      Self::Camera => "Camera",
    }
    .fmt(f)
  }
}

impl TryFrom<u8> for Kind {
  type Error = CartError;

  fn try_from(value: u8) -> Result<Self, Self::Error> {
    match value {
      0x00 => Ok(Self::None {
        ram: false,
        pwr: false,
      }),
      0x01 => Ok(Self::MBC1 {
        ram: false,
        pwr: false,
      }),
      0x03 => Ok(Kind::MBC1 {
        ram: true,
        pwr: true,
      }),
      0x05 => Ok(Kind::MBC2 { pwr: false }),
      0x06 => Ok(Kind::MBC2 { pwr: true }),
      0x08 => Ok(Kind::None {
        ram: true,
        pwr: false,
      }),
      0x09 => Ok(Kind::None {
        ram: true,
        pwr: true,
      }),
      0x0b => Ok(Kind::MMM01 {
        ram: false,
        pwr: false,
      }),
      0x0c => Ok(Kind::MMM01 {
        ram: true,
        pwr: false,
      }),
      0x0d => Ok(Kind::MMM01 {
        ram: true,
        pwr: true,
      }),
      0x0f => Ok(Kind::MBC3 {
        ram: false,
        pwr: true,
        rtc: true,
      }),
      0x10 => Ok(Kind::MBC3 {
        ram: true,
        pwr: true,
        rtc: true,
      }),
      0x11 => Ok(Kind::MBC3 {
        ram: false,
        pwr: false,
        rtc: false,
      }),
      0x12 => Ok(Kind::MBC3 {
        ram: true,
        pwr: false,
        rtc: false,
      }),
      0x13 => Ok(Kind::MBC3 {
        ram: true,
        pwr: true,
        rtc: false,
      }),
      0x19 => Ok(Kind::MBC5 {
        ram: false,
        pwr: false,
        rtc: false,
      }),
      0x1a => Ok(Kind::MBC5 {
        ram: true,
        pwr: false,
        rtc: false,
      }),
      0x1b => Ok(Kind::MBC5 {
        ram: true,
        pwr: true,
        rtc: false,
      }),
      0x1c => Ok(Kind::MBC5 {
        ram: false,
        pwr: false,
        rtc: true,
      }),
      0x1d => Ok(Kind::MBC5 {
        ram: true,
        pwr: false,
        rtc: true,
      }),
      0x1e => Ok(Kind::MBC5 {
        ram: true,
        pwr: true,
        rtc: true,
      }),
      0x20 => Ok(Kind::MBC6),
      0x22 => Ok(Kind::MBC7),
      0xfc => Ok(Kind::Camera),
      0xfe => Ok(Kind::HuC3),
      0xff => Ok(Kind::HuC1),
      value => Err(Self::Error::Kind(value)),
    }
  }
}
