use super::arch::value::Value;

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
