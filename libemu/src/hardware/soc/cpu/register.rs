use std::fmt::Display;

// -------------
// | A   Flags |  ---> Program Status Word
// | B       C |  ---> B
// | D       E |  ---> D
// | H       L |  ---> H
// |    SP     |  ---> Stack Pointer
// |    PC     |  ---> Program Counter
/// -------------
#[derive(Default, Debug)]
pub struct Register {
  a: u8,
  f: u8,
  b: u8,
  c: u8,
  d: u8,
  e: u8,
  h: u8,
  l: u8,
  sp: u16,
  pc: u16,
}

impl Display for Register {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    writeln!(f, "┌───┬────┬───┬────┐")?;
    writeln!(f, "│ A │ {:02x} │ F │ {:02x} │", self.get_a(), self.get_f())?;
    writeln!(f, "├───┼────┼───┼────┤")?;
    writeln!(f, "│ B │ {:02x} │ C │ {:02x} │", self.get_b(), self.get_c())?;
    writeln!(f, "├───┼────┼───┼────┤")?;
    writeln!(f, "│ D │ {:02x} │ E │ {:02x} │", self.get_d(), self.get_e())?;
    writeln!(f, "├───┼────┼───┼────┤")?;
    writeln!(f, "│ H │ {:02x} │ L │ {:02x} │", self.get_h(), self.get_l())?;
    writeln!(f, "├───┴────┼───┴────┤")?;
    writeln!(f, "│   SP   │  {:04x}  │", self.get_sp())?;
    writeln!(f, "├────────┼────────┤")?;
    writeln!(f, "│   PC   │  {:04x}  │", self.get_pc())?;
    write!(f, "└────────┴────────┘")
  }
}

impl Register {
  pub fn new() -> Self {
    Self {
      a: 0x01,
      f: 0xB0,
      b: 0x00,
      c: 0x13,
      d: 0x00,
      e: 0xD8,
      h: 0x01,
      l: 0x4D,
      sp: 0xFFFE,
      pc: 0x0100,
    }
  }

  pub fn get_af(&self) -> u16 {
    (self.a as u16) << 8 | self.f as u16
  }

  pub fn set_af(&mut self, val: u16) {
    self.a = (val >> 8) as u8;
    self.f = val as u8;
  }

  pub fn get_bc(&self) -> u16 {
    (self.b as u16) << 8 | self.c as u16
  }

  pub fn set_bc(&mut self, val: u16) {
    self.b = (val >> 8) as u8;
    self.c = val as u8;
  }

  pub fn get_de(&self) -> u16 {
    (self.d as u16) << 8 | self.e as u16
  }

  pub fn set_de(&mut self, val: u16) {
    self.d = (val >> 8) as u8;
    self.e = val as u8;
  }

  pub fn get_hl(&self) -> u16 {
    (self.h as u16) << 8 | self.l as u16
  }

  pub fn set_hl(&mut self, val: u16) {
    self.h = (val >> 8) as u8;
    self.l = val as u8;
  }

  pub fn get_sp(&self) -> u16 {
    self.sp
  }

  pub fn set_sp(&mut self, val: u16) {
    self.sp = val;
  }

  pub fn get_pc(&self) -> u16 {
    self.pc
  }

  pub fn set_pc(&mut self, val: u16) {
    self.pc = val;
  }

  pub fn get_a(&self) -> u8 {
    self.a
  }

  pub fn set_a(&mut self, val: u8) {
    self.a = val;
  }

  pub fn get_f(&self) -> u8 {
    self.f
  }

  pub fn set_f(&mut self, val: u8) {
    self.f = val;
  }

  pub fn get_b(&self) -> u8 {
    self.b
  }

  pub fn set_b(&mut self, val: u8) {
    self.b = val;
  }

  pub fn get_c(&self) -> u8 {
    self.c
  }

  pub fn set_c(&mut self, val: u8) {
    self.c = val;
  }

  pub fn get_d(&self) -> u8 {
    self.d
  }

  pub fn set_d(&mut self, val: u8) {
    self.d = val;
  }

  pub fn get_e(&self) -> u8 {
    self.e
  }

  pub fn set_e(&mut self, val: u8) {
    self.e = val;
  }

  pub fn get_h(&self) -> u8 {
    self.h
  }

  pub fn set_h(&mut self, val: u8) {
    self.h = val;
  }

  pub fn get_l(&self) -> u8 {
    self.l
  }

  pub fn set_l(&mut self, val: u8) {
    self.l = val;
  }

  pub fn set_flags_znhc(&mut self, z: bool, n: bool, h: bool, c: bool) {
    self.f = 0;
    if z {
      self.f |= 1 << 7;
    }
    if n {
      self.f |= 1 << 6;
    }
    if h {
      self.f |= 1 << 5;
    }
    if c {
      self.f |= 1 << 4;
    }
  }

  pub fn get_flags_c(&self) -> bool {
    self.f & 0b0001_0000 != 0
  }

  pub fn set_flags_c(&mut self, val: bool) {
    if val {
      self.f |= 0b0001_0000;
    } else {
      self.f &= 0b1110_1111;
    }
  }

  pub fn get_flags_z(&self) -> bool {
    self.f & 0b1000_0000 != 0
  }

  pub fn set_flags_z(&mut self, val: bool) {
    if val {
      self.f |= 0b1000_0000;
    } else {
      self.f &= 0b0111_1111;
    }
  }

  pub fn get_flags_n(&self) -> bool {
    self.f & 0b0100_0000 != 0
  }

  pub fn set_flags_n(&mut self, val: bool) {
    if val {
      self.f |= 0b0100_0000;
    } else {
      self.f &= 0b1011_1111;
    }
  }

  pub fn get_flags_h(&self) -> bool {
    self.f & 0b0010_0000 != 0
  }

  pub fn set_flags_h(&mut self, val: bool) {
    if val {
      self.f |= 0b0010_0000;
    } else {
      self.f &= 0b1101_1111;
    }
  }

  pub fn inc_pc(&mut self, val: u16) {
    self.pc = self.pc.wrapping_add(val);
  }
}

pub enum Flags {
  Z, // bit 7
  N, // bit 6
  H, // bit 5
  C, // bit 4
}

pub enum Register8 {
  A,
  B,
  C,
  D,
  E,
  H,
  L,
}

pub enum Register16 {
  AF,
  BC,
  DE,
  HL,
  SP,
  PC,
}
