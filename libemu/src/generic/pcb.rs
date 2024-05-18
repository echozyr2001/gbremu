use super::{arch::value::Value, bus::Bus};

pub trait Board<Idx, V>
where
  Idx: Value,
  V: Value,
{
  /// Connect the board to the bus.
  fn connect(&self, bus: &mut Bus<Idx, V>);

  /// Disconnect the board from the bus.
  fn disconnect(&self) {
    unimplemented!()
  }
}
