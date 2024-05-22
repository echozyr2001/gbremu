use core::fmt;
use std::fmt::{Display, Formatter};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ObjectData {
  pub(crate) x: i16,
  pub(crate) y: i16,
  pub(crate) tile: u8,
  pub(crate) tile_bank: u8,
  pub(crate) palette: u8,
  pub(crate) xflip: bool,
  pub(crate) yflip: bool,
  pub(crate) bg_over: bool,
  pub(crate) index: u8,
}

impl ObjectData {
  pub fn new() -> Self {
    Self {
      x: 0,
      y: 0,
      tile: 0,
      tile_bank: 0,
      palette: 0,
      xflip: false,
      yflip: false,
      bg_over: false,
      index: 0,
    }
  }
}

impl Default for ObjectData {
  fn default() -> Self {
    Self::new()
  }
}

impl Display for ObjectData {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "Index: {}, X: {}, Y: {}, Tile: {}",
      self.index, self.x, self.y, self.tile
    )
  }
}
