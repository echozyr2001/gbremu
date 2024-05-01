use super::value::Value;

/// Addressable read and write interfiace.
pub trait Address<Idx, V>
where
  Idx: Value,
  V: Value,
{
  /// Read a value from the address.
  fn read(&self, idx: Idx) -> V;
  /// Write a value to the address.
  fn write(&mut self, idx: Idx, val: V);
}

/// Addressable read and write interfiace with error handling.
pub trait TryAddress<Idx, V>
where
  Idx: Value,
  V: Value,
{
  type Error;

  /// Read a value from the address, errors if read is not successful.
  fn try_read(&self, idx: Idx) -> Result<V, Self::Error>;
  /// Write a value to the address, errors if write is not successful.
  fn try_write(&mut self, idx: Idx, val: V) -> Result<(), Self::Error>;
}
