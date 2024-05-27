#![allow(dead_code)]
mod inst;

use std::cell::{Ref, RefMut};

use crate::{
  bus::Bus,
  generic::{address::Address, shared::Shared},
};

use self::inst::{EXTENDED, INSTRUCTIONS};

pub const PREFIX: u8 = 0xcb;

#[derive(Default)]
pub struct CpuRegisters {
  pub pc: u16,
  pub sp: u16,
  pub a: u8,
  pub b: u8,
  pub c: u8,
  pub d: u8,
  pub e: u8,
  pub h: u8,
  pub l: u8,
}
pub struct Cpu {
  pub regs: CpuRegisters,

  ime: bool,
  zero: bool,
  sub: bool,
  half_carry: bool,
  carry: bool,
  halted: bool,

  pub bus: Shared<Bus>,
  pub cycles: u8,
}

impl Cpu {
  pub fn bus(&self) -> Ref<Bus> {
    self.bus.borrow()
  }

  pub fn bus_mut(&self) -> RefMut<Bus> {
    self.bus.borrow_mut()
  }
}

impl Cpu {
  pub fn new(bus: Shared<Bus>) -> Self {
    Self {
      regs: CpuRegisters::default(),
      ime: false,
      zero: false,
      sub: false,
      half_carry: false,
      carry: false,
      halted: false,
      bus,
      cycles: 0,
    }
  }
}

impl Cpu {
  pub fn clock(&mut self) -> u8 {
    let pc = self.regs.pc;

    if self.halted
      && !self.ime
      && self.bus().read(0xFFFF) != 0x00
      && (((self.bus().read(0xFFFF) & 0x01 == 0x01) && self.bus().ppu().int_vblank())
        || ((self.bus().read(0xFFFF) & 0x02 == 0x02) && self.bus().ppu().int_stat())
        // || ((self.bus.read(0xFFFF) & 0x04 == 0x04) && self.bus.timer().int_tima())
        // || ((self.bus.read(0xFFFF) & 0x08 == 0x08) && self.bus.serial().int_serial())
        || ((self.bus().read(0xFFFF) & 0x10 == 0x10) && self.bus().pad().int_pad()))
    {
      self.halted = false;
    }

    // checks the IME (interrupt master enable) is enabled and then checks
    // if there's any interrupt to be handled, in case there's one, tries
    // to check which one should be handled and then handles it
    // this code assumes that the're no more that one interrupt triggered
    // per clock cycle, this is a limitation of the current implementation
    if self.ime && self.bus().read(0xFFFF) != 0x00 {
      if (self.bus().read(0xFFFF) & 0x01 == 0x01) && self.bus().ppu().int_vblank() {
        // debugln!("Going to run V-Blank interrupt handler (0x40)");

        self.disable_int();
        self.push_word(pc);
        self.regs.pc = 0x40;

        // acknowledges that the V-Blank interrupt has been
        // properly handled
        self.bus_mut().ppu_mut().ack_vblank();

        // in case the CPU is currently halted waiting
        // for an interrupt, releases it
        if self.halted {
          self.halted = false;
        }

        return 20;
      } else if (self.bus.read(0xFFFF) & 0x02 == 0x02) && self.bus().ppu().int_stat() {
        // debugln!("Going to run LCD STAT interrupt handler (0x48)");

        self.disable_int();
        self.push_word(pc);
        self.regs.pc = 0x48;

        // acknowledges that the STAT interrupt has been
        // properly handled
        self.bus_mut().ppu_mut().ack_stat();

        // in case the CPU is currently halted waiting
        // for an interrupt, releases it
        if self.halted {
          self.halted = false;
        }

        return 20;
      } else if (self.bus.read(0xFFFF) & 0x10 == 0x10) && self.bus().pad().int_pad() {
        // debugln!("Going to run JoyPad interrupt handler (0x60)");

        self.disable_int();
        self.push_word(pc);
        self.regs.pc = 0x60;

        // acknowledges that the pad interrupt has been
        // properly handled
        self.bus_mut().pad_mut().ack_pad();

        // in case the CPU is currently halted waiting
        // for an interrupt, releases it
        if self.halted {
          self.halted = false;
        }

        return 20;
      }
    }

    // in case the CPU is currently in the halted state
    // returns the control flow immediately with the associated
    // number of cycles estimated for the halted execution
    if self.halted {
      return 4;
    }

    let mut opcode = self.bus.read(self.regs.pc);
    self.regs.pc = self.regs.pc.wrapping_add(1);

    let is_prefix = opcode == PREFIX;
    let inst: &(fn(&mut Cpu), u8, &str);

    if is_prefix {
      opcode = self.bus.read(self.regs.pc);
      self.regs.pc = self.regs.pc.wrapping_add(1);
      inst = &EXTENDED[opcode as usize];
    } else {
      inst = &INSTRUCTIONS[opcode as usize];
    }

    #[allow(unused_variables)]
    let (inst_fn, inst_time, inst_str) = inst;

    // calls the current instruction and increments the number of
    // cycles executed by the instruction time of the instruction
    // that has just been executed
    self.cycles = 0;
    inst_fn(self);
    self.cycles = self.cycles.wrapping_add(*inst_time);

    // returns the number of cycles that the operation
    // that has been executed has taken
    self.cycles
  }

  pub fn reset(&mut self) {
    self.regs.pc = 0x0100;
    self.regs.sp = 0xfffe;
    self.regs.a = 0x01;
    self.regs.b = 0x00;
    self.regs.c = 0x13;
    self.regs.d = 0x00;
    self.regs.e = 0xd8;
    self.regs.h = 0x01;
    self.regs.l = 0x4d;

    self.ime = false;
    self.zero = false;
    self.sub = false;
    self.half_carry = false;
    self.carry = false;
    self.halted = false;
  }
}

impl Cpu {
  #[inline(always)]
  pub fn pc(&self) -> u16 {
    self.regs.pc
  }

  #[inline(always)]
  pub fn set_pc(&mut self, value: u16) {
    self.regs.pc = value;
  }

  #[inline(always)]
  pub fn sp(&self) -> u16 {
    self.regs.sp
  }

  #[inline(always)]
  pub fn set_sp(&mut self, value: u16) {
    self.regs.sp = value;
  }

  #[inline(always)]
  pub fn af(&self) -> u16 {
    (self.regs.a as u16) << 8 | self.f() as u16
  }

  #[inline(always)]
  pub fn bc(&self) -> u16 {
    (self.regs.b as u16) << 8 | self.regs.c as u16
  }

  #[inline(always)]
  pub fn f(&self) -> u8 {
    let mut f = 0x0u8;
    if self.zero {
      f |= 0x80;
    }
    if self.sub {
      f |= 0x40;
    }
    if self.half_carry {
      f |= 0x20;
    }
    if self.carry {
      f |= 0x10;
    }
    f
  }

  #[inline(always)]
  pub fn set_f(&mut self, value: u8) {
    self.zero = value & 0x80 == 0x80;
    self.sub = value & 0x40 == 0x40;
    self.half_carry = value & 0x20 == 0x20;
    self.carry = value & 0x10 == 0x10;
  }

  #[inline(always)]
  pub fn set_af(&mut self, value: u16) {
    self.regs.a = (value >> 8) as u8;
    self.set_f(value as u8);
  }

  #[inline(always)]
  pub fn set_bc(&mut self, value: u16) {
    self.regs.b = (value >> 8) as u8;
    self.regs.c = value as u8;
  }

  #[inline(always)]
  pub fn de(&self) -> u16 {
    (self.regs.d as u16) << 8 | self.regs.e as u16
  }

  #[inline(always)]
  pub fn set_de(&mut self, value: u16) {
    self.regs.d = (value >> 8) as u8;
    self.regs.e = value as u8;
  }

  #[inline(always)]
  pub fn hl(&self) -> u16 {
    (self.regs.h as u16) << 8 | self.regs.l as u16
  }

  #[inline(always)]
  pub fn set_hl(&mut self, value: u16) {
    self.regs.h = (value >> 8) as u8;
    self.regs.l = value as u8;
  }

  #[inline(always)]
  pub fn ime(&self) -> bool {
    self.ime
  }

  #[inline(always)]
  pub fn set_ime(&mut self, value: bool) {
    self.ime = value;
  }

  #[inline(always)]
  pub fn read_u8(&mut self) -> u8 {
    let byte = self.bus().read(self.regs.pc);
    self.regs.pc = self.regs.pc.wrapping_add(1);
    byte
  }

  #[inline(always)]
  pub fn read_u16(&mut self) -> u16 {
    let byte1 = self.read_u8();
    let byte2 = self.read_u8();

    byte1 as u16 | ((byte2 as u16) << 8)
  }

  #[inline(always)]
  pub fn push_byte(&mut self, byte: u8) {
    self.regs.sp = self.regs.sp.wrapping_sub(1);
    self.bus_mut().write(self.regs.sp, byte);
  }

  #[inline(always)]
  pub fn push_word(&mut self, word: u16) {
    self.push_byte((word >> 8) as u8);
    self.push_byte(word as u8);
  }

  #[inline(always)]
  pub fn pop_byte(&mut self) -> u8 {
    let byte = self.bus().read(self.regs.sp);
    self.regs.sp = self.regs.sp.wrapping_add(1);
    byte
  }

  #[inline(always)]
  pub fn pop_word(&mut self) -> u16 {
    self.pop_byte() as u16 | ((self.pop_byte() as u16) << 8)
  }

  #[inline(always)]
  pub fn zero(&self) -> bool {
    self.zero
  }

  #[inline(always)]
  pub fn set_zero(&mut self, value: bool) {
    self.zero = value
  }

  #[inline(always)]
  pub fn sub(&self) -> bool {
    self.sub
  }

  #[inline(always)]
  pub fn set_sub(&mut self, value: bool) {
    self.sub = value;
  }

  #[inline(always)]
  pub fn half_carry(&self) -> bool {
    self.half_carry
  }

  #[inline(always)]
  pub fn set_half_carry(&mut self, value: bool) {
    self.half_carry = value
  }

  #[inline(always)]
  pub fn carry(&self) -> bool {
    self.carry
  }

  #[inline(always)]
  pub fn set_carry(&mut self, value: bool) {
    self.carry = value;
  }

  #[inline(always)]
  pub fn halt(&mut self) {
    self.halted = true;
  }

  #[inline(always)]
  pub fn stop(&mut self) {}

  #[inline(always)]
  pub fn enable_int(&mut self) {
    self.ime = true;
  }

  #[inline(always)]
  pub fn disable_int(&mut self) {
    self.ime = false;
  }
}
