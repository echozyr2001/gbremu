use std::fmt::Debug;

mod ram;
mod rom;

pub use ram::Ram;
pub use rom::Rom;

use super::{address::Address, device::Device};

pub struct Unuse;

impl Address for Unuse {
  fn read(&self, _addr: u16) -> u8 {
    0xFF
  }

  fn write(&mut self, _addr: u16, _value: u8) {}
}

impl Device for Unuse {}

pub enum Error {
  InvalidAddress(u16),
  Write,
}

impl Debug for Error {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Error::InvalidAddress(addr) => write!(f, "Invalid address: {:#06x}", addr),
      Error::Write => write!(f, "Cannot write to ROM"),
    }
  }
}
