use core::fmt;
use std::fmt::{Display, Formatter};

use super::TILE_WIDTH;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct TileData {
  pub(crate) palette: u8,
  pub(crate) vram_bank: u8,
  pub(crate) xflip: bool,
  pub(crate) yflip: bool,
  pub(crate) priority: bool,
}

impl TileData {
  pub fn new() -> Self {
    Self {
      palette: 0,
      vram_bank: 0,
      xflip: false,
      yflip: false,
      priority: false,
    }
  }
}

impl Default for TileData {
  fn default() -> Self {
    Self::new()
  }
}

impl Display for TileData {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "Palette: {}, VRAM Bank: {}, X Flip: {}, Y Flip: {}",
      self.palette, self.vram_bank, self.xflip, self.yflip
    )
  }
}

/// Represents a tile within the Game Boy context,
/// should contain the pixel buffer of the tile.
/// The tiles are always 8x8 pixels in size.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Tile {
  /// The buffer for the tile, should contain a byte
  /// per each pixel of the tile with values ranging
  /// from 0 to 3 (4 colors).
  pub(crate) buffer: [u8; 64],
}

impl Tile {
  pub fn get(&self, x: usize, y: usize) -> u8 {
    self.buffer[y * TILE_WIDTH + x]
  }

  pub fn set(&mut self, x: usize, y: usize, value: u8) {
    self.buffer[y * TILE_WIDTH + x] = value;
  }
}

impl Tile {
  pub fn get_row(&self, y: usize) -> &[u8] {
    &self.buffer[y * TILE_WIDTH..(y + 1) * TILE_WIDTH]
  }
}

impl Display for Tile {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    let mut buffer = String::new();
    for y in 0..8 {
      for x in 0..8 {
        buffer.push_str(format!("{}", self.get(x, y)).as_str());
      }
      buffer.push('\n');
    }
    write!(f, "{}", buffer)
  }
}
