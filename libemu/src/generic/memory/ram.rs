use crate::generic::{
  address::{Address, TryAddress},
  device::Device,
};

use super::Error;

#[derive(Default)]
pub struct Ram(Vec<u8>);

impl Ram {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn inner(&self) -> &[u8] {
    &self.0
  }

  pub fn set_data(&mut self, data: &[u8]) {
    self.0 = data.to_vec();
  }

  pub fn reset(&mut self) {
    self.0.clear();
  }
}

impl From<&[u8]> for Ram {
  fn from(arr: &[u8]) -> Self {
    Self(arr.to_vec())
  }
}

impl Device for Ram {}

impl Address for Ram {
  fn read(&self, addr: u16) -> u8 {
    self.try_read(addr).unwrap()
  }

  fn write(&mut self, addr: u16, value: u8) {
    self.try_write(addr, value).unwrap()
  }
}

impl TryAddress for Ram {
  type Error = Error;

  fn try_read(&self, addr: u16) -> Result<u8, Self::Error> {
    self
      .0
      .get(usize::from(addr))
      .copied()
      .ok_or(Error::InvalidAddress(addr))
  }

  fn try_write(&mut self, addr: u16, value: u8) -> Result<(), Self::Error> {
    self
      .0
      .get_mut(usize::from(addr))
      .map(|it| *it = value)
      .ok_or(Error::InvalidAddress(addr))
  }
}
