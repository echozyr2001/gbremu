//! Generic memory mapped device.
//!
//! The [`Device`] trait is useful in combination with [`Bus`].  
//! Together, they can be used to emulate the behaviour of [memory-mapped I/O].
//!
//! [memory-mapped I/O]: https://en.wikipedia.org/wiki/Memory-mapped_I/O

use super::{arch::value::Value, share::Shared};

/// Memory-mapped I/O device.
pub trait Device<Idx, V>
where
  Idx: Value,
  V: Value,
{
  /// Construct a [`Shared`] device.
  fn to_shared(&self) -> Shared<Self>
  where
    Self: 'static + Sized,
  {
    todo!()
  }

  /// Construct a [`Shared`] dynamic device.
  fn to_dynamic(&self) -> Dynamic<Idx, V>
  where
    Self: 'static + Sized,
  {
    todo!()
  }
}

/// Runtime generic shared device.
pub type Dynamic<Idx, V> = Shared<dyn Device<Idx, V>>;
