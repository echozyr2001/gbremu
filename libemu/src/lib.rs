use std::{fs::File, io::Read};

use generic::{device::Device, memory::ram::Ram, pcb::Board, share::Shared};
use hardware::{
  cartridge::Cartridge,
  noc::NoC,
  soc::{
    ppu::{DISPLAY_HEIGHT, DISPLAY_WIDTH, FRAME_BUFFER_SIZE},
    SoC,
  },
};

pub mod boot;
pub mod error;
pub mod generic;
pub mod hardware;

type Wram = Ram<u8, 0x2000>;
type Vram = Ram<u8, 0x2000>;

pub const CPU_FREQ: u32 = 4194304;
pub const VISUAL_FREQ: f32 = 59.7275;

// #[derive(Debug)]
pub struct GameBoy {
  // clock: u128,
  pub soc: SoC,
  wram: Shared<Wram>,
  vram: Shared<Vram>,
  // joypad: JoyPad
  // serial: Serial
  // timer: Timer
  cart: Option<Cartridge>,
  pub noc: NoC,
  // pic: Shared<Pic>,
}

// type Boot = Rom<u8, 0x100>;
impl GameBoy {
  pub fn new() -> Self {
    Self::default().setup()
  }

  pub fn setup(self) -> Self {
    self.connect();
    self
  }

  fn load_cart(&mut self, cart: Cartridge) {
    let bus = &mut self.noc.bus.borrow_mut();
    cart.connect(bus);
    self.cart = Some(cart);
  }

  pub fn load_cart_file(&mut self, path: &str) {
    // let data = read_file(path);
    //
    let rom = {
      // Open ROM file
      let mut file = File::open(path).unwrap();
      // Read ROM into a buffer
      let mut buf = Vec::new();
      let nbytes = file.read_to_end(&mut buf);
      // Use ROM contents
      buf
    };

    let cart = Cartridge::new(rom).unwrap();

    self.load_cart(cart);
  }

  pub fn connect(&self) {
    let vram = self.vram.clone();
    let wram = self.wram.clone();
    let echo = self.wram.clone();

    let bus = &mut self.noc.bus.borrow_mut();

    bus.map(0x8000..=0x9FFF, vram.clone().to_dynamic());
    bus.map(0xC000..=0xDFFF, wram.clone().to_dynamic());
    bus.map(0xE000..=0xFDFF, echo.clone().to_dynamic());

    self.soc.connect(bus);
  }

  pub fn load_dmg(&mut self) {
    todo!()
    // self.load_boot(&DMG_BOOT);
  }

  pub fn load_boot(&mut self, boot: &[u8]) {
    // let bus = &mut self.noc.bus.borrow_mut();
    // bus.write(0x0000..=0x00FF, boot);
  }

  pub fn display_width(&self) -> usize {
    DISPLAY_WIDTH
  }

  pub fn display_height(&self) -> usize {
    DISPLAY_HEIGHT
  }

  pub fn ppu_frame(&mut self) -> u16 {
    self.soc.ppu.frame_index()
  }

  pub fn frame_buffer(&mut self) -> &[u8; FRAME_BUFFER_SIZE] {
    self.soc.ppu.frame_buffer()
  }
}

impl Default for GameBoy {
  fn default() -> Self {
    let noc = NoC::default();
    Self {
      soc: SoC::new(&noc),
      cart: Option::default(),
      wram: Shared::new(Wram::default()),
      vram: Shared::new(Vram::default()),
      noc,
    }
  }
}

impl GameBoy {
  pub fn cycle(&mut self) -> u16 {
    self.soc.cpu.cycle();
    // let cycles_n = cycles;
    self.cycle_devices(4);
    4
  }

  pub fn cycle_devices(&mut self, cycles: u16) {
    self.ppu_cycle(cycles);
  }

  pub fn ppu_cycle(&mut self, cycles: u16) {
    self.soc.ppu.cycle(cycles);
  }
}
