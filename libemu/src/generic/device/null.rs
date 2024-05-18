// use crate::generic::arch::{address::Address, value::Value};

// use super::Device;

// #[derive(Debug, Default)]
// pub struct Null<V, const N: usize = 0>(V)
// where
//   V: Value;

// impl<V, const N: usize> Null<V, N>
// where
//   V: Value,
// {
//   pub fn new() -> Self {
//     Self::default()
//   }

//   pub fn with(val: V) -> Self {
//     Self(val)
//   }

//   pub fn read_as(&mut self, val: V) {
//     self.0 = val;
//   }
// }

// impl<Idx, V, const N: usize> Device<Idx, V> for Null<V, N>
// where
//   Idx: Value,
//   V: Value,
// {
// }

// impl<Idx, V, const N: usize> Address<Idx, V> for Null<V, N>
// where
//   Idx: Value,
//   V: Value,
// {
//   fn read(&self, idx: Idx) -> V {
//     todo!()
//   }

//   fn write(&mut self, idx: Idx, val: V) {
//     todo!()
//   }
// }
