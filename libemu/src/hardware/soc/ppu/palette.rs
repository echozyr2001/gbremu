use super::{Palette, PALETTE_SIZE, RGB_SIZE};

/// Represents a palette together with the metadata
/// that is associated with it.
#[derive(Clone, PartialEq, Eq)]
pub struct PaletteInfo {
  name: String,
  colors: Palette,
}

impl PaletteInfo {
  pub fn new(name: &str, colors: Palette) -> Self {
    Self {
      name: String::from(name),
      colors,
    }
  }

  pub fn from_colors_hex(name: &str, colors_hex: &str) -> Self {
    let colors = Self::parse_colors_hex(colors_hex);
    Self::new(name, colors)
  }

  pub fn parse_colors_hex(colors_hex: &str) -> Palette {
    let mut colors = [[0u8; RGB_SIZE]; PALETTE_SIZE];
    for (index, color) in colors_hex.split(',').enumerate() {
      let color = color.trim();
      let color = u32::from_str_radix(color, 16).unwrap_or(0);
      let r = ((color >> 16) & 0xff) as u8;
      let g = ((color >> 8) & 0xff) as u8;
      let b = (color & 0xff) as u8;
      colors[index] = [r, g, b];
    }
    colors
  }

  pub fn name(&self) -> &String {
    &self.name
  }

  /// Returns the colors in RGB format.
  pub fn colors(&self) -> &Palette {
    &self.colors
  }

  /// Returns the colors in hex format, separated by comma.
  pub fn colors_hex(&self) -> String {
    let mut buffer = String::new();
    let mut is_first = true;
    for color in self.colors.iter() {
      let r = color[0];
      let g = color[1];
      let b = color[2];
      let color = (r as u32) << 16 | (g as u32) << 8 | b as u32;
      if is_first {
        is_first = false;
      } else {
        buffer.push(',');
      }
      buffer.push_str(format!("{:06x}", color).as_str());
    }
    buffer
  }
}
