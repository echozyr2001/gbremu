//! Memory interface.
//!
//! # Usage
//!
//! The [`Bus`] struct allows users to mount another [`Device`] to
//! anywhere in the address space. As it itself implements the [`Device`]
//!
//! [memory-mapped I/O]: https://en.wikipedia.org/wiki/Memory-mapped_I/O

pub mod adapt;
mod mapping;

use std::fmt::Display;

use crate::{error::BusError, generic::device::Dynamic};

use self::mapping::{Mapping, Range};

use super::{
  arch::{
    address::{Address, TryAddress},
    value::Value,
  },
  device::Device,
};

#[derive(Debug, Default)]
pub struct Bus<Idx, V>(Vec<Mapping<Idx, Dynamic<Idx, V>>>)
where
  Idx: Value,
  V: Value;

impl<Idx, V> Bus<Idx, V>
where
  Idx: Value,
  V: Value,
{
  pub fn new() -> Self {
    Self::default()
  }

  pub fn clear(&mut self) {
    self.0.clear();
  }

  pub fn map(&mut self, range: Range<Idx>, instance: Dynamic<Idx, V>) {
    self.0.push(Mapping::new(range, instance))
  }

  pub fn unmap(&mut self, _range: Range<Idx>) -> Option<Dynamic<Idx, V>> {
    // self
    //   .0
    //   .iter()
    //   .position(|mapping| mapping.range() == range)
    //   .map(|index| self.0.remove(index))
    //   .map(|mapping| mapping.instance())
    todo!()
  }
}

// impl<Idx, V, const N: usize> From<[(Range<Idx>, Dynamic<Idx, V>); N]> for Bus<Idx, V>
// where
//   Idx: Value,
//   V: Value,
// {
//   fn from(arr: [(Range<Idx>, Dynamic<Idx, V>); N]) -> Self {
//     let mut this = Self::default();
//     for (range, dev) in arr {
//       this.map(range, dev);
//     }
//     this
//   }
// }

impl<Idx, V> Device<Idx, V> for Bus<Idx, V>
where
  Idx: Value,
  V: Value,
{
}

impl<Idx, V> Address<Idx, V> for Bus<Idx, V>
where
  Idx: Value,
  V: Value,
{
  fn read(&self, idx: Idx) -> V {
    self.try_read(idx).unwrap()
  }

  fn write(&mut self, idx: Idx, val: V) {
    self.try_write(idx, val).unwrap()
  }
}

impl<Idx, V> TryAddress<Idx, V> for Bus<Idx, V>
where
  Idx: Value,
  V: Value,
{
  type Error = BusError<Idx>;

  fn try_read(&self, idx: Idx) -> Result<V, Self::Error> {
    self
      .0
      .iter()
      .find(|mapping| mapping.contains(idx))
      .ok_or(BusError::Unmapped(idx))
      .map(|mapping| mapping.instance.read(idx - mapping.base()))
  }

  fn try_write(&mut self, idx: Idx, val: V) -> Result<(), Self::Error> {
    self
      .0
      .iter()
      .find(|mapping| mapping.contains(idx))
      .ok_or(BusError::Unmapped(idx))
      .map(|mapping| {
        mapping
          .instance
          .borrow_mut()
          .write(idx - mapping.base(), val)
      })
  }
}

impl Display for Bus<u16, u8> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    for mapping in &self.0 {
      writeln!(f, "{:?}\n", mapping)?;
    }
    Ok(())
  }
}
