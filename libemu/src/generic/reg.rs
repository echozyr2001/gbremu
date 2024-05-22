use super::{
  arch::{address::Address, value::Value},
  device::Device,
};

#[derive(Default, Debug)]
pub struct Register<V>(V)
where
  V: Value;

impl<V> Register<V>
where
  V: Value,
{
  pub fn new() -> Self {
    Self::default()
  }
}

impl<Idx, V> Device<Idx, V> for Register<V>
where
  Idx: Value,
  V: Value,
{
}

impl<Idx, V> Address<Idx, V> for Register<V>
where
  Idx: Value,
  V: Value,
{
  fn read(&self, _idx: Idx) -> V {
    self.load()
  }

  fn write(&mut self, _idx: Idx, val: V) {
    self.store(val)
  }
}

pub trait Cell<V>
where
  V: Value,
{
  fn load(&self) -> V;

  fn store(&mut self, value: V);
}

impl<V> Cell<V> for Register<V>
where
  V: Value,
{
  fn load(&self) -> V {
    self.0
  }

  fn store(&mut self, value: V) {
    self.0 = value;
  }
}
