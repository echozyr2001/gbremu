#![allow(dead_code)]

use crate::generic::{
  arch::{address::Address, value::Value},
  device::{Device, Dynamic},
};

#[derive(Debug, Default)]
pub struct Bank<Idx, V>
where
  Idx: Value,
  V: Value,
{
  pub select: usize,
  pub banks: Vec<Dynamic<Idx, V>>,
}

impl<Idx, V> Bank<Idx, V>
where
  Idx: Value,
  V: Value,
{
  pub fn new() -> Self {
    Self::default()
  }

  pub fn add(&mut self, dev: Dynamic<Idx, V>) {
    self.banks.push(dev);
  }

  pub fn insert(&mut self, idx: usize, dev: Dynamic<Idx, V>) {
    self.banks.insert(idx, dev);
  }

  pub fn remove(&mut self, idx: usize) -> Dynamic<Idx, V> {
    self.banks.remove(idx)
  }

  pub fn size(&self) -> usize {
    self.banks.len()
  }
}

impl<Idx, V> From<&[Dynamic<Idx, V>]> for Bank<Idx, V>
where
  Idx: Value,
  V: Value,
{
  fn from(banks: &[Dynamic<Idx, V>]) -> Self {
    Self {
      banks: Vec::from(banks),
      ..Default::default()
    }
  }
}

impl<Idx, V> Device<Idx, V> for Bank<Idx, V>
where
  Idx: Value,
  V: Value,
{
}

impl<Idx, V> Address<Idx, V> for Bank<Idx, V>
where
  Idx: Value,
  V: Value,
{
  fn read(&self, idx: Idx) -> V {
    self.banks[self.select].read(idx)
  }

  fn write(&mut self, idx: Idx, val: V) {
    self.banks[self.select].write(idx, val)
  }
}
