use crate::{
  error::MemoryError,
  generic::{
    arch::{
      address::{Address, TryAddress},
      value::Value,
    },
    device::Device,
  },
};

/// Random Access Memory.
#[derive(Debug)]
pub struct RAM<V, const N: usize>(Box<[V; N]>)
where
  V: Value;

impl<V, const N: usize> RAM<V, N>
where
  V: Value,
{
  /// Constructs a new and empty [`RAM`] resource.
  pub fn new() -> Self {
    Self::default()
  }
}

impl<V, const N: usize> Default for RAM<V, N>
where
  V: Value,
{
  fn default() -> Self {
    Self(
      vec![Default::default(); N]
        .into_boxed_slice()
        .try_into()
        .unwrap(),
    )
  }
}

impl<V, const N: usize> From<&[V; N]> for RAM<V, N>
where
  V: Value,
{
  fn from(arr: &[V; N]) -> Self {
    Self(Vec::from(&arr[..]).into_boxed_slice().try_into().unwrap())
  }
}

impl<Idx, V, const N: usize> Device<Idx, V> for RAM<V, N>
where
  Idx: Value,
  V: Value,
  usize: From<Idx>,
{
}

impl<Idx, V, const N: usize> Address<Idx, V> for RAM<V, N>
where
  Idx: Value,
  V: Value,
  usize: From<Idx>,
{
  fn read(&self, idx: Idx) -> V {
    self.try_read(idx).unwrap()
  }

  fn write(&mut self, idx: Idx, val: V) {
    self.try_write(idx, val).unwrap()
  }
}

impl<Idx, V, const N: usize> TryAddress<Idx, V> for RAM<V, N>
where
  Idx: Value,
  V: Value,
  usize: From<Idx>,
{
  type Error = MemoryError<Idx>;

  fn try_read(&self, idx: Idx) -> Result<V, Self::Error> {
    self
      .0
      .get(usize::from(idx))
      .copied()
      .ok_or(MemoryError::Bounds(idx))
  }

  fn try_write(&mut self, idx: Idx, val: V) -> Result<(), Self::Error> {
    self
      .0
      .get_mut(usize::from(idx))
      .map(|it| *it = val)
      .ok_or(MemoryError::Bounds(idx))
  }
}
