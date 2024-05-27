use crate::generic::{
  address::{Address, TryAddress},
  device::Device,
};

use super::Error;

#[derive(Default)]
pub struct Rom(Vec<u8>);

impl Rom {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn set_data(&mut self, data: &[u8]) {
    self.0 = data.to_vec();
  }

  pub fn reset(&mut self) {
    self.0.clear();
  }
}

impl From<&[u8]> for Rom {
  fn from(arr: &[u8]) -> Self {
    Self(arr.to_vec())
  }
}

impl Device for Rom {}

impl Address for Rom {
  fn read(&self, addr: u16) -> u8 {
    self.try_read(addr).unwrap()
  }

  fn write(&mut self, addr: u16, value: u8) {
    self.try_write(addr, value).unwrap()
  }
}

impl TryAddress for Rom {
  type Error = Error;

  fn try_read(&self, addr: u16) -> Result<u8, Self::Error> {
    self
      .0
      .get(usize::from(addr))
      .copied()
      .ok_or(Error::InvalidAddress(addr))
  }

  fn try_write(&mut self, addr: u16, _value: u8) -> Result<(), Self::Error> {
    match self.0.get_mut(usize::from(addr)) {
      Some(_) => Err(Error::Write),
      None => Err(Error::InvalidAddress(addr)),
    }
  }
}
