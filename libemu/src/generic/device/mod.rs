//! Generic memory mapped device.
//!
//! # Usage
//!
//! The [`Device`] trait is useful in combination with [`Bus`].  
//! Together, they can be used to emulate the behaviour of [memory-mapped I/O].
//!
//! [memory-mapped I/O]: https://en.wikipedia.org/wiki/Memory-mapped_I/O

use std::fmt::Debug;

use super::{
  arch::{address::Address, value::Value},
  share::Shared,
};

mod null;

/// Memory-mapped I/O device.
pub trait Device<Idx, V>: Address<Idx, V> + Debug
where
  Idx: Value,
  V: Value,
{
  /// Construct a [`Shared`] device.
  fn to_shared(self) -> Shared<Self>
  where
    Self: 'static + Sized,
  {
    self.into()
  }

  /// Construct a [`Dynamic`] device.
  fn to_dynamic(self) -> Dynamic<Idx, V>
  where
    Self: 'static + Sized,
  {
    self.to_shared().into()
  }
}

/// Runtime generic shared device.
pub type Dynamic<Idx, V> = Shared<dyn Device<Idx, V>>;

impl<T, Idx, V> From<Shared<T>> for Dynamic<Idx, V>
where
  T: Device<Idx, V> + 'static,
  Idx: Value,
  V: Value,
{
  fn from(shared: Shared<T>) -> Self {
    Self(shared.0.clone())
  }
}
