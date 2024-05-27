use super::{Palette, PALETTE_SIZE, RGB_SIZE};

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

  pub fn colors(&self) -> &Palette {
    &self.colors
  }
}
