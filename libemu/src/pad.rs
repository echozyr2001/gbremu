use log::warn;

use crate::generic::{address::Address, device::Device};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PadSelection {
  None,
  Action,
  Direction,
}

pub enum PadKey {
  Up,
  Down,
  Left,
  Right,
  Start,
  Select,
  A,
  B,
}

pub struct Pad {
  up: bool,
  down: bool,
  left: bool,
  right: bool,
  start: bool,
  select: bool,
  b: bool,
  a: bool,
  selection: PadSelection,
  int_pad: bool,
}

impl Pad {
  pub fn new() -> Self {
    Self {
      down: false,
      up: false,
      left: false,
      right: false,
      start: false,
      select: false,
      b: false,
      a: false,
      selection: PadSelection::None,
      int_pad: false,
    }
  }

  pub fn key_press(&mut self, key: PadKey) {
    match key {
      PadKey::Up => self.up = true,
      PadKey::Down => self.down = true,
      PadKey::Left => self.left = true,
      PadKey::Right => self.right = true,
      PadKey::Start => self.start = true,
      PadKey::Select => self.select = true,
      PadKey::A => self.a = true,
      PadKey::B => self.b = true,
    }

    self.int_pad = true;
  }

  pub fn key_lift(&mut self, key: PadKey) {
    match key {
      PadKey::Up => self.up = false,
      PadKey::Down => self.down = false,
      PadKey::Left => self.left = false,
      PadKey::Right => self.right = false,
      PadKey::Start => self.start = false,
      PadKey::Select => self.select = false,
      PadKey::A => self.a = false,
      PadKey::B => self.b = false,
    }
  }

  pub fn int_pad(&self) -> bool {
    self.int_pad
  }

  pub fn set_int_pad(&mut self, value: bool) {
    self.int_pad = value;
  }

  pub fn ack_pad(&mut self) {
    self.set_int_pad(false);
  }
}

impl Default for Pad {
  fn default() -> Self {
    Self::new()
  }
}

impl Address for Pad {
  fn read(&self, addr: u16) -> u8 {
    match addr {
      // 0xFF00 — P1/JOYP: Joypad
      0xff00 => {
        let mut value = match self.selection {
          PadSelection::Action => {
            (if self.a { 0x00 } else { 0x01 }
              | if self.b { 0x00 } else { 0x02 }
              | if self.select { 0x00 } else { 0x04 }
              | if self.start { 0x00 } else { 0x08 })
          },
          PadSelection::Direction => {
            (if self.right { 0x00 } else { 0x01 }
              | if self.left { 0x00 } else { 0x02 }
              | if self.up { 0x00 } else { 0x04 }
              | if self.down { 0x00 } else { 0x08 })
          },
          PadSelection::None => 0x0f,
        };
        value |= match self.selection {
          PadSelection::Action => 0x10,
          PadSelection::Direction => 0x20,
          PadSelection::None => 0x30,
        };
        value
      },
      _ => {
        warn!("Reading from unknown Pad location 0x{:04x}", addr);
        0xff
      },
    }
  }

  fn write(&mut self, addr: u16, value: u8) {
    match addr {
      // 0xFF00 — P1/JOYP: Joypad
      0xff00 => {
        self.selection = match value & 0x30 {
          0x10 => PadSelection::Action,
          0x20 => PadSelection::Direction,
          0x30 => PadSelection::None,
          _ => PadSelection::None,
        };
      },
      _ => warn!("Writing to unknown Pad location 0x{:04x}", addr),
    }
  }
}

impl Device for Pad {}
