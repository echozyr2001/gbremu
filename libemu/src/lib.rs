use generic::{device::Device, memory::ram::Ram, pcb::Board, share::Shared};
use hardware::{cartridge::Cartridge, noc::NoC, soc::SoC};

pub mod boot;
pub mod error;
pub mod generic;
pub mod hardware;

type Wram = Ram<u8, 0x2000>;
type Vram = Ram<u8, 0x2000>;

#[derive(Debug)]
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
  // doc: Option<Doctor>,
}

// type Boot = Rom<u8, 0x100>;
impl GameBoy {
  pub fn new() -> Self {
    Self::default().setup()
  }

  pub fn setup(self) -> Self {
    // TODO: 1. Load boot rom
    self.connect();
    self
  }

  pub fn load_cart(&mut self, cart: Cartridge) {
    let bus = &mut self.noc.bus.borrow_mut();
    cart.connect(bus);
    self.cart = Some(cart);
  }

  pub fn connect(&self) {
    let vram = self.vram.clone();
    let wram = self.wram.clone();
    let echo = self.wram.clone();

    let bus = &mut self.noc.bus.borrow_mut();

    bus.map(0x8000..=0x9FFF, vram.clone().to_dynamic());
    bus.map(0xC000..=0xDFFF, wram.clone().to_dynamic());
    bus.map(0xE000..=0xFDFF, echo.clone().to_dynamic());
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
  pub fn cycle(&mut self) {
    self.soc.cpu.cycle();
  }
}
