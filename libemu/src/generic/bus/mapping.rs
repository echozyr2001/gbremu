#![allow(dead_code)]

use std::{fmt::Display, ops::RangeInclusive};

use crate::generic::arch::value::Value;

pub type Range<Idx> = RangeInclusive<Idx>;

#[derive(Debug)]
pub struct Mapping<Idx, T>
where
  Idx: Value,
{
  range: Range<Idx>,
  pub instance: T,
}

impl<Idx, T> Mapping<Idx, T>
where
  Idx: Value,
{
  pub fn new(range: Range<Idx>, instance: T) -> Self {
    Self { range, instance }
  }

  pub fn base(&self) -> Idx {
    *self.range.start()
  }

  pub fn len(&self) -> Idx {
    *self.range.end() - *self.range.start()
  }

  pub fn contains(&self, idx: Idx) -> bool {
    self.range.contains(&idx)
  }

  pub fn range(&self) -> &Range<Idx> {
    &self.range
  }

  pub fn instance(&self) -> &T {
    &self.instance
  }

  pub fn instance_mut(&mut self) -> &mut T {
    &mut self.instance
  }
}

impl Display for Mapping<u16, u8> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:04X}..={:04X}", self.range.start(), self.range.end())
  }
}
