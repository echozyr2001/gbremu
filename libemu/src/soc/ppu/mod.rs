mod object;
pub mod palette;
mod tile;

use std::{borrow::BorrowMut, cmp::max};

use log::warn;

use crate::generic::{address::Address, device::Device};

use self::{object::ObjectData, tile::Tile};

pub const VRAM_SIZE: usize = 0x2000;
pub const HRAM_SIZE: usize = 0x80;
pub const OAM_SIZE: usize = 260;
pub const PALETTE_SIZE: usize = 4;
pub const TILE_WIDTH: usize = 8;
pub const TILE_HEIGHT: usize = 8;
pub const RGB_SIZE: usize = 3;
pub const TILE_DOUBLE_HEIGHT: usize = 16;

pub const TILE_COUNT: usize = 384;

pub const OBJ_COUNT: usize = 40;

pub const DISPLAY_WIDTH: usize = 160;
pub const DISPLAY_HEIGHT: usize = 144;
pub const DISPLAY_SIZE: usize = DISPLAY_WIDTH * DISPLAY_HEIGHT;

pub const COLOR_BUFFER_SIZE: usize = DISPLAY_SIZE;
pub const SHADE_BUFFER_SIZE: usize = DISPLAY_SIZE;
pub const FRAME_BUFFER_SIZE: usize = DISPLAY_SIZE * RGB_SIZE;

pub const PALETTE_COLORS: Palette = [[255, 255, 255], [192, 192, 192], [96, 96, 96], [0, 0, 0]];

pub type Pixel = [u8; RGB_SIZE];
pub type Palette = [Pixel; PALETTE_SIZE];

pub struct PpuRegisters {
  pub lcdc: u8,
  pub stat: u8,
  pub bgp: u8,
  pub obp0: u8,
  pub obp1: u8,
  pub scy: u8,
  pub scx: u8,
  pub wy: u8,
  pub wx: u8,
  pub ly: u8,
  pub lyc: u8,
}

pub struct Ppu {
  shade_buffer: Box<[u8; SHADE_BUFFER_SIZE]>,
  frame_buffer: Box<[u8; FRAME_BUFFER_SIZE]>,

  vram: [u8; VRAM_SIZE],
  oam: [u8; OAM_SIZE],

  tiles: [Tile; TILE_COUNT],
  obj_data: [ObjectData; OBJ_COUNT],
  palette_colors: Palette,
  palette_obj_0: Palette,
  palette_obj_1: Palette,

  window_counter: u8,
  frame_index: u16,
  frame_buffer_index: u16,

  regs: PpuRegisters,
  dot: u16,
  mode: PpuMode,
  int_vblank: bool,
  int_stat: bool,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PpuMode {
  HBlank = 0,
  VBlank = 1,
  OamRead = 2,
  VramRead = 3,
}

impl Ppu {
  pub fn new() -> Self {
    Self {
      shade_buffer: Box::new([0u8; COLOR_BUFFER_SIZE]),
      frame_buffer: Box::new([0u8; FRAME_BUFFER_SIZE]),
      vram: [0u8; VRAM_SIZE],
      oam: [0u8; OAM_SIZE],
      tiles: [Tile { buffer: [0u8; 64] }; TILE_COUNT],
      obj_data: [ObjectData::default(); OBJ_COUNT],
      palette_colors: PALETTE_COLORS,
      palette_obj_0: [[0u8; RGB_SIZE]; PALETTE_SIZE],
      palette_obj_1: [[0u8; RGB_SIZE]; PALETTE_SIZE],
      regs: PpuRegisters {
        lcdc: 0x0,
        stat: 0x0,
        bgp: 0x0,
        obp0: 0x0,
        obp1: 0x0,
        scy: 0x0,
        scx: 0x0,
        wy: 0x0,
        wx: 0x0,
        ly: 0x0,
        lyc: 0x0,
      },
      mode: PpuMode::OamRead,
      dot: 0,
      window_counter: 0x0,
      frame_index: 0,
      frame_buffer_index: std::u16::MAX,
      int_vblank: false,
      int_stat: false,
    }
  }

  pub fn reset(&mut self) {
    self.shade_buffer = Box::new([0u8; SHADE_BUFFER_SIZE]);
    self.frame_buffer = Box::new([0u8; FRAME_BUFFER_SIZE]);
    self.vram = [0u8; VRAM_SIZE];
    self.tiles = [Tile { buffer: [0u8; 64] }; TILE_COUNT];
    self.obj_data = [ObjectData::default(); OBJ_COUNT];
    self.palette_obj_0 = [[0u8; RGB_SIZE]; PALETTE_SIZE];
    self.palette_obj_1 = [[0u8; RGB_SIZE]; PALETTE_SIZE];
    self.regs.lcdc = 0x0;
    self.regs.bgp = 0x0;
    self.regs.obp0 = 0x0;
    self.regs.obp1 = 0x0;
    self.regs.scy = 0x0;
    self.regs.scx = 0x0;
    self.regs.ly = 0x0;
    self.regs.lyc = 0x0;
    self.regs.stat = 0x0;
    self.mode = PpuMode::OamRead;
    self.dot = 0;
    self.window_counter = 0;
    self.frame_buffer_index = std::u16::MAX;
    self.int_vblank = false;
    self.int_stat = false;
  }

  fn lcd_enable(&self) -> bool {
    self.regs.lcdc & 0x80 == 0x80
  }

  fn window_map(&self) -> bool {
    self.regs.lcdc & 0x40 == 0x40
  }

  fn window_enable(&self) -> bool {
    self.regs.lcdc & 0x20 == 0x20
  }

  fn bg_tile(&self) -> bool {
    self.regs.lcdc & 0x10 == 0x10
  }

  fn bg_map(&self) -> bool {
    self.regs.lcdc & 0x08 == 0x08
  }

  fn obj_size(&self) -> bool {
    self.regs.lcdc & 0x04 == 0x04
  }

  fn obj_enable(&self) -> bool {
    self.regs.lcdc & 0x02 == 0x02
  }

  fn bg_enable(&self) -> bool {
    self.regs.lcdc & 0x01 == 0x01
  }

  fn stat_lyc(&self) -> bool {
    self.regs.stat & 0x40 == 0x40
  }

  fn stat_oam(&self) -> bool {
    self.regs.stat & 0x20 == 0x20
  }

  fn stat_vblank(&self) -> bool {
    self.regs.stat & 0x10 == 0x10
  }

  fn stat_hblank(&self) -> bool {
    self.regs.stat & 0x08 == 0x08
  }

  pub fn clock(&mut self, cycles: u16) {
    // in case the LCD is currently off then we skip the current
    // clock operation the PPU should not work
    if !self.lcd_enable() {
      return;
    }

    // 模拟并行？
    self.dot += cycles;

    match self.mode {
      PpuMode::OamRead => {
        if self.dot >= 80 {
          self.mode = PpuMode::VramRead;
          self.dot -= 80;
        }
      },
      PpuMode::VramRead => {
        if self.dot >= 172 {
          self.render_line();

          self.mode = PpuMode::HBlank;
          self.dot -= 172;
          self.update_stat()
        }
      },
      PpuMode::HBlank => {
        if self.dot >= 204 {
          if self.window_enable()
            && self.regs.wx as i16 - 7 < DISPLAY_WIDTH as i16
            && self.regs.wy < DISPLAY_HEIGHT as u8
            && self.regs.ly >= self.regs.wy
          {
            self.window_counter += 1;
          }

          self.regs.ly += 1;

          if self.regs.ly == 144 {
            self.int_vblank = true;
            self.mode = PpuMode::VBlank;
          } else {
            self.mode = PpuMode::OamRead;
          }

          self.dot -= 204;
          self.update_stat()
        }
      },
      PpuMode::VBlank => {
        if self.dot >= 456 {
          self.regs.ly += 1;

          if self.regs.ly == 154 {
            self.mode = PpuMode::OamRead;
            self.regs.ly = 0;
            self.window_counter = 0;
            self.frame_index = self.frame_index.wrapping_add(1);
            self.update_stat()
          }

          self.dot -= 456;
        }
      },
    }
  }

  pub fn frame_buffer(&mut self) -> &mut [u8; FRAME_BUFFER_SIZE] {
    if self.frame_index == self.frame_buffer_index {
      return &mut self.frame_buffer;
    }

    for (index, pixel) in self.frame_buffer.chunks_mut(RGB_SIZE).enumerate() {
      let shade_index = self.shade_buffer[index];
      let color = &self.palette_colors[shade_index as usize];
      pixel[0] = color[0];
      pixel[1] = color[1];
      pixel[2] = color[2];
    }

    self.frame_buffer_index = self.frame_index;
    &mut self.frame_buffer
  }

  pub fn set_palette_colors(&mut self, value: &Palette) {
    self.palette_colors = *value;
  }

  pub fn frame_index(&self) -> u16 {
    self.frame_index
  }

  #[inline(always)]
  pub fn int_vblank(&self) -> bool {
    self.int_vblank
  }

  #[inline(always)]
  pub fn set_int_vblank(&mut self, value: bool) {
    self.int_vblank = value;
  }

  #[inline(always)]
  pub fn ack_vblank(&mut self) {
    self.set_int_vblank(false);
  }

  #[inline(always)]
  pub fn int_stat(&self) -> bool {
    self.int_stat
  }

  #[inline(always)]
  pub fn set_int_stat(&mut self, value: bool) {
    self.int_stat = value;
  }

  #[inline(always)]
  pub fn ack_stat(&mut self) {
    self.set_int_stat(false);
  }

  fn update_tile(&mut self, addr: u16, _value: u8) {
    let addr = (addr & 0x1ffe) as usize;
    let tile_index = (addr >> 4) & 0x01ff;
    let tile = self.tiles[tile_index].borrow_mut();
    let y = (addr >> 1) & 0x0007;

    let mut mask;

    for x in 0..TILE_WIDTH {
      // TODO: 为什么-1
      mask = 1 << (TILE_WIDTH - 1 - x);
      #[allow(clippy::bool_to_int_with_if)]
      tile.set(
        x,
        y,
        if self.vram[addr] & mask > 0 { 0x1 } else { 0x0 }
          | if self.vram[addr + 1] & mask > 0 {
            0x2
          } else {
            0x0
          },
      );
    }
  }

  fn update_object(&mut self, addr: u16, value: u8) {
    let addr = (addr & 0x01ff) as usize;
    let obj_index = addr >> 2;
    if obj_index >= OBJ_COUNT {
      return;
    }
    let obj = self.obj_data[obj_index].borrow_mut();
    match addr & 0x03 {
      0x00 => obj.y = value as i16 - 16,
      0x01 => obj.x = value as i16 - 8,
      0x02 => obj.tile = value,
      0x03 => {
        obj.tile_bank = (value & 0x08 == 0x08) as u8;
        obj.palette = (value & 0x10 == 0x10) as u8;
        obj.xflip = value & 0x20 == 0x20;
        obj.yflip = value & 0x40 == 0x40;
        obj.bg_over = value & 0x80 == 0x80;
        obj.index = obj_index as u8;
      },
      _ => (),
    }
  }

  fn render_line(&mut self) {
    if self.bg_enable() {
      self.render_map(
        self.bg_map(),
        self.regs.scx,
        self.regs.scy,
        0,
        0,
        self.regs.ly,
      );
    }
    if self.bg_enable() && self.window_enable() {
      self.render_map(
        self.window_map(),
        0,
        0,
        self.regs.wx,
        self.regs.wy,
        self.window_counter,
      );
    }
    if self.obj_enable() {
      self.render_objects();
    }
  }

  fn render_map(&mut self, map: bool, scx: u8, scy: u8, wx: u8, wy: u8, ld: u8) {
    if self.regs.ly < wy {
      return;
    }

    let map_offset: usize = if map { 0x1c00 } else { 0x1800 };

    // calculates the map row index for the tile by using the current line
    // index and the DY (scroll Y) divided by 8 (as the tiles are 8x8 pixels),
    // on top of that ensures that the result is modulus 32 meaning that the
    // drawing wraps around the Y axis
    let row_index = (((ld as usize + scy as usize) & 0xff) >> 3) % 32;

    // calculates the map offset by the row offset multiplied by the number
    // of tiles in each row (32)
    let row_offset = row_index * 32;

    // calculates the sprite line offset by using the SCX register
    // shifted by 3 meaning that the tiles are 8x8
    let mut line_offset = (scx >> 3) as usize;

    // calculates the index of the initial tile in drawing,
    // if the tile data set in use is #1, the indexes are
    // signed, then calculates a real tile offset
    let mut tile_index = self.vram[map_offset + row_offset + line_offset] as usize;
    if !self.bg_tile() && tile_index < 128 {
      tile_index += 256;
    }

    // obtains the reference to the tile that is going to be drawn
    let mut tile = &self.tiles[tile_index];

    // calculates the offset that is going to be used in the update of the color buffer
    // which stores Game Boy colors from 0 to 3
    let mut color_offset = self.regs.ly as usize * DISPLAY_WIDTH;

    // obtains the current integer value (raw) for the background palette
    // this is going to be used for shade index value computation (DMG only)
    let palette_v = self.regs.bgp;

    // calculates both the current Y and X positions within the tiles
    // using the bitwise and operation as an effective modulus 8
    let y = (ld as usize + scy as usize) & 0x07;
    let mut x = (scx & 0x07) as usize;

    // calculates the initial tile X position in drawing, doing this
    // allows us to position the background map properly in the display
    let initial_index = max(wx as i16 - 7, 0) as usize;
    color_offset += initial_index;

    // iterates over all the pixels in the current line of the display
    // to draw the background map, note that the initial index is used
    // to skip the drawing of the tiles that are not visible (WX)
    for _ in initial_index..DISPLAY_WIDTH {
      // obtains the current pixel data from the tile
      let pixel = tile.get(x, y);

      // updates the pixel in the color buffer, which stores
      // the raw pixel color information (unmapped) and then
      // updates the shade buffer with the shade index
      self.shade_buffer[color_offset] = (palette_v >> (pixel * 2)) & 3;

      // increments the current tile X position in drawing
      x += 1;

      // in case the end of tile width has been reached then
      // a new tile must be retrieved for rendering
      if x == TILE_WIDTH {
        // resets the tile X position to the base value
        // as a new tile is going to be drawn
        x = 0;

        // calculates the new line tile offset making sure that
        // the maximum of 32 is not overflown
        line_offset = (line_offset + 1) % 32;

        // calculates the tile index and makes sure the value
        // takes into consideration the bg tile value
        tile_index = self.vram[map_offset + row_offset + line_offset] as usize;
        if !self.bg_tile() && tile_index < 128 {
          tile_index += 256;
        }

        // obtains the reference to the new tile in drawing
        tile = &self.tiles[tile_index];
      }

      // increments the color offset by one, representing
      // the drawing of one pixel
      color_offset += 1;
    }
  }

  fn render_objects(&mut self) {
    let mut draw_count = 0u8;

    // allocates the buffer that is going to be used to determine
    // drawing priority for overlapping pixels between different
    // objects, in MBR mode the object that has the smallest X
    // coordinate takes priority in drawing the pixel
    let mut index_buffer = [-256i16; DISPLAY_WIDTH];

    // iterates over the complete set of available object to check
    // the ones that require drawing and draws them
    for index in 0..OBJ_COUNT {
      if draw_count == 10 {
        break;
      }

      let obj = &self.obj_data[index];

      let obj_height = if self.obj_size() {
        TILE_DOUBLE_HEIGHT
      } else {
        TILE_HEIGHT
      };

      // verifies if the sprite is currently located at the
      // current line that is going to be drawn and skips it
      // in case it's not
      let is_contained =
        (obj.y <= self.regs.ly as i16) && ((obj.y + obj_height as i16) > self.regs.ly as i16);
      if !is_contained {
        continue;
      }

      let (palette, palette_index) = if obj.palette == 0 {
        (&self.palette_obj_0, 1_u8)
      } else if obj.palette == 1 {
        (&self.palette_obj_1, 2_u8)
      } else {
        panic!("Invalid object palette: {:02x}", obj.palette);
      };

      // obtains the current integer value (raw) for the palette in use
      // this is going to be used for shade index value computation (DMG only)
      // let palette_v = self.palettes[palette_index as usize];
      //
      let palette_v = if palette_index == 0 {
        self.regs.bgp
      } else if palette_index == 1 {
        self.regs.obp0
      } else {
        self.regs.obp1
      };

      // calculates the offset in the color buffer (raw color information
      // from 0 to 3) for the sprit that is going to be drawn, this value
      // is kept as a signed integer to allow proper negative number math
      let mut color_offset = self.regs.ly as i32 * DISPLAY_WIDTH as i32 + obj.x as i32;

      // calculates the offset in the frame buffer for the sprite
      // that is going to be drawn, this is going to be the starting
      // point for the draw operation to be performed
      let mut frame_offset =
        (self.regs.ly as i32 * DISPLAY_WIDTH as i32 + obj.x as i32) * RGB_SIZE as i32;

      // the relative title offset should range from 0 to 7 in 8x8
      // objects and from 0 to 15 in 8x16 objects
      let mut tile_offset = self.regs.ly as i16 - obj.y;

      // in case we're flipping the object we must recompute the
      // tile offset as an inverted value using the object's height
      if obj.yflip {
        tile_offset = obj_height as i16 - tile_offset - 1;
      }

      let tile: &Tile;

      // "calculates" the index offset that is going to be applied
      // to the tile index to retrieve the proper tile taking into
      // consideration the VRAM in which the tile is stored
      let tile_bank_offset = { obj.tile_bank as usize * TILE_COUNT };

      if self.obj_size() {
        // 8x16 object
        if tile_offset < 8 {
          let tile_index = (obj.tile as usize & 0xfe) + tile_bank_offset;
          tile = &self.tiles[tile_index];
        } else {
          let tile_index = (obj.tile as usize | 0x01) + tile_bank_offset;
          tile = &self.tiles[tile_index];
          tile_offset -= 8;
        }
      } else {
        // 8x8 object
        let tile_index = obj.tile as usize + tile_bank_offset;
        tile = &self.tiles[tile_index];
      }

      let tile_row = tile.get_row(tile_offset as usize);

      // determines if the object should always be placed over the
      // previously placed background or window pixels
      let obj_over = !obj.bg_over;

      for tile_x in 0..TILE_WIDTH {
        let x = obj.x + tile_x as i16;
        let is_contained = (x >= 0) && (x < DISPLAY_WIDTH as i16);
        if is_contained {
          // the object is only considered visible if no background or
          // window should be drawn over or if the underlying pixel
          // is transparent (zero value) meaning there's no background
          // or window for the provided pixel
          let mut is_visible = obj_over;

          // additionally (in CCG mode) the object is only considered to
          // be visible if the priority buffer is not set for the current
          // pixel, this means that the background is capturing priority
          // by having the BG-to-OAM priority bit set in the bg map attributes
          is_visible &= true;

          let pixel = tile_row[if obj.xflip {
            // TODO: 为什么-1
            TILE_WIDTH - 1 - tile_x
          } else {
            tile_x
          }];
          if is_visible && pixel != 0 {
            // marks the current pixel in iteration as "owned"
            // by the object with the defined X base position,
            // to be used in priority calculus
            index_buffer[x as usize] = obj.x;

            // updates the pixel in the color buffer, which stores
            // the raw pixel color information (unmapped) and then
            // updates the shade buffer with the shade index
            self.shade_buffer[color_offset as usize] = (palette_v >> (pixel * 2)) & 3;

            // re-maps the pixel according to the object palette
            // and then sets the color pixel in the frame buffer
            let color = &palette[pixel as usize];
            self.frame_buffer[frame_offset as usize] = color[0];
            self.frame_buffer[frame_offset as usize + 1] = color[1];
            self.frame_buffer[frame_offset as usize + 2] = color[2];
          }
        }

        // increment the color offset by one as this represents
        // the advance of one color pixel
        color_offset += 1;

        // increments the offset of the frame buffer by the
        // size of an RGB pixel (which is 3 bytes)
        frame_offset += RGB_SIZE as i32;
      }

      // increments the counter so that we're able to keep
      // track on the number of object drawn
      draw_count += 1;
    }
  }

  fn update_stat(&mut self) {
    self.int_stat = self.stat_level();
  }

  fn stat_level(&self) -> bool {
    self.stat_lyc() && self.regs.lyc == self.regs.ly
      || self.stat_oam() && self.mode == PpuMode::OamRead
      || self.stat_vblank() && self.mode == PpuMode::VBlank
      || self.stat_hblank() && self.mode == PpuMode::HBlank
  }
}

impl Default for Ppu {
  fn default() -> Self {
    Self::new()
  }
}

impl Address for Ppu {
  fn read(&self, addr: u16) -> u8 {
    match addr {
      0x8000..=0x9fff => self.vram[(addr & 0x1fff) as usize],
      0xfe00..=0xfe9f => self.oam[(addr & 0x009f) as usize],
      // Not Usable
      0xfea0..=0xfeff => 0xff,
      0xff40 => self.regs.lcdc,
      0xff41 => self.regs.stat,
      0xff42 => self.regs.scy,
      0xff43 => self.regs.scx,
      0xff44 => self.regs.ly,
      0xff45 => self.regs.lyc,
      0xff47 => self.regs.bgp,
      0xff48 => self.regs.obp0,
      0xff49 => self.regs.obp1,
      0xff4a => self.regs.wy,
      0xff4b => self.regs.wx,
      _ => {
        warn!("Reading from unknown PPU location 0x{:04x}", addr);
        0xff
      },
    }
  }

  fn write(&mut self, addr: u16, value: u8) {
    match addr {
      0x8000..=0x9fff => {
        self.vram[(addr & 0x1fff) as usize] = value;
        if addr < 0x9800 {
          self.update_tile(addr, value);
        }
      },
      0xfe00..=0xfe9f => {
        self.oam[(addr & 0x009f) as usize] = value;
        self.update_object(addr, value);
      },
      // Not Usable
      0xfea0..=0xfeff => (),
      0xff40 => {
        self.regs.lcdc = value;
        if !self.lcd_enable() {
          self.mode = PpuMode::HBlank;
          self.dot = 0;
          self.regs.ly = 0;
          self.int_vblank = false;
          self.int_stat = false;
          self.window_counter = 0;
        }
      },
      0xff41 => self.regs.stat = value,
      0xff42 => self.regs.scy = value, // scrolling
      0xff43 => self.regs.scx = value, // scrolling
      0xff45 => self.regs.lyc = value, // LCD status
      0xff47 => self.regs.bgp = value,
      0xff48 => self.regs.obp0 = value,
      0xff49 => self.regs.obp1 = value,
      0xff4a => self.regs.wy = value, // scrolling
      0xff4b => self.regs.wx = value, // scrolling
      0xff7f => (),
      _ => warn!("Writing in unknown PPU location 0x{:04x}", addr),
    }
  }
}

impl Device for Ppu {}
