// -------------
// | A   Flags |  ---> Program Status Word
// | B       C |  ---> B
// | D       E |  ---> D
// | H       L |  ---> H
// |    SP     |  ---> Stack Pointer
// |    PC     |  ---> Program Counter
/// -------------
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

impl Register {
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
}

pub enum Flags {
  Z, // bit 7
  N, // bit 6
  H, // bit 5
  C, // bit 4
}
