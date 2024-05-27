use std::cell::{Ref, RefMut};

use crate::{
  boot_data::DMG_BOOT,
  bus::Bus,
  cartridge::{Cartridge, RamSize},
  error::Error,
  generic::{memory::Ram, shared::Shared},
  pad::{Pad, PadKey},
  soc::{
    ppu::{Ppu, DISPLAY_HEIGHT, DISPLAY_WIDTH},
    Soc,
  },
  util::read_file,
};

// TODO: impl const
pub const WRAM_SIZE: usize = 8192;
pub const HRAM_SIZE: usize = 128;

pub struct GameBoy {
  soc: Soc,
  pad: Shared<Pad>,
  cart: Shared<Cartridge>,
  wram: Shared<Ram>,
  bus: Shared<Bus>,
}

impl GameBoy {
  pub const CPU_FREQ: u32 = 4194304;
  pub const VISUAL_FREQ: f32 = 59.7275;

  // TODO: impl new
  pub fn new() -> Self {
    let bus = Shared::new(Bus::new());
    let soc = Soc::new(bus.clone());

    let wram = Shared::new(Ram::new());
    let pad = Shared::new(Pad::default());
    bus.borrow_mut().set_wram(wram.clone());
    bus.borrow_mut().set_pad(pad.clone());

    Self {
      soc,
      pad,
      cart: Shared::new(Cartridge::default()),
      wram,
      bus,
    }
  }

  pub fn reset(&mut self) {
    self.soc.reset();
    self.cart.borrow_mut().reset();
  }
}

impl GameBoy {
  pub fn ppu(&self) -> Ref<Ppu> {
    self.soc.ppu()
  }

  pub fn ppu_mut(&mut self) -> RefMut<Ppu> {
    self.soc.ppu_mut()
  }

  pub fn wram(&self) -> Ref<Ram> {
    self.wram.borrow()
  }

  pub fn wram_mut(&mut self) -> RefMut<Ram> {
    self.wram.borrow_mut()
  }

  pub fn cart(&self) -> Ref<Cartridge> {
    self.cart.borrow()
  }

  pub fn cart_mut(&mut self) -> RefMut<Cartridge> {
    self.cart.borrow_mut()
  }
}

impl GameBoy {
  pub fn ppu_frame(&self) -> u16 {
    self.ppu().frame_index()
  }

  pub fn display_width(&self) -> usize {
    DISPLAY_WIDTH
  }

  pub fn display_height(&self) -> usize {
    DISPLAY_HEIGHT
  }

  pub fn ram_size(&self) -> RamSize {
    RamSize::Size8K
  }

  pub fn vram_size(&self) -> RamSize {
    RamSize::Size8K
  }
}

impl GameBoy {
  pub fn clock(&mut self) -> u16 {
    let cycles = self.clock_cpu() as u16;
    self.clock_ppu(cycles);
    self.clock_dma(cycles);
    cycles
  }

  fn clock_cpu(&mut self) -> u8 {
    self.soc.clock_cpu()
  }

  fn clock_ppu(&mut self, cycles: u16) {
    self.soc.clock_ppu(cycles)
  }

  fn clock_dma(&mut self, cycles: u16) {
    self.soc.clock_dma(cycles)
  }

  pub fn key_press(&mut self, key: PadKey) {
    self.pad.borrow_mut().key_press(key);
  }

  pub fn key_lift(&mut self, key: PadKey) {
    self.pad.borrow_mut().key_lift(key);
  }

  pub fn load_dmg(&mut self) {
    self.load_boot(&DMG_BOOT);
    self.bus.borrow_mut().allocate_dmg();
  }

  fn load_boot(&mut self, data: &[u8]) {
    self.bus.borrow_mut().write_boot(data);
  }

  pub fn load_cart_file(
    &mut self,
    path: &str,
    ram_path: Option<&str>,
  ) -> Result<Ref<Cartridge>, Error> {
    let data = read_file(path)?;
    match ram_path {
      Some(ram_path) => {
        let ram_data = read_file(ram_path)?;
        self.load_cart(&data, Some(&ram_data))
      },
      None => self.load_cart(&data, None),
    }
  }

  fn load_cart(&mut self, data: &[u8], ram_data: Option<&[u8]>) -> Result<Ref<Cartridge>, Error> {
    let mut cart = Cartridge::from_data(data)?;
    if let Some(ram_data) = ram_data {
      cart.set_ram_data(ram_data)
    }
    self.cart = Shared::new(cart);
    self.bus.borrow_mut().set_cart(self.cart.clone());
    Ok(self.cart.borrow())
  }
}

impl Default for GameBoy {
  fn default() -> Self {
    Self::new()
  }
}
