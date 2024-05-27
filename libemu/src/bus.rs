use std::cell::{Ref, RefMut};

use log::debug;

use crate::{
  cartridge::Cartridge,
  gb::{HRAM_SIZE, WRAM_SIZE},
  generic::{address::Address, device::Device, memory::Ram, shared::Shared},
  pad::Pad,
  soc::{boot::Boot, dma::Dma, ppu::Ppu},
};

type HRam = Ram;
type WRam = Ram;

#[derive(Clone, Default)]
pub struct Bus {
  hram: Shared<HRam>,
  ppu: Shared<Ppu>,
  dma: Shared<Dma>,
  pad: Shared<Pad>,
  cart: Shared<Cartridge>,

  boot: Shared<Boot>,
  wram: Shared<WRam>,

  ie: u8,
}

impl Bus {
  pub fn new() -> Self {
    Self {
      hram: HRam::default().to_shared(),
      ppu: Ppu::default().to_shared(),
      dma: Dma::default().to_shared(),
      pad: Pad::default().to_shared(),
      cart: Cartridge::default().to_shared(),
      boot: Boot::new().to_shared(),
      wram: WRam::default().to_shared(),

      ie: 0x00,
    }
  }

  pub fn reset(&mut self) {
    self.hram.borrow_mut().reset();
    self.ppu.borrow_mut().reset();
    self.dma.borrow_mut().reset();
    // self.pad.borrow_mut().reset();
    self.cart.borrow_mut().reset();
    self.boot.borrow_mut().reset();
    self.wram.borrow_mut().reset();
    self.ie = 0x00;
  }

  pub fn allocate_dmg(&mut self) {
    self
      .hram
      .borrow_mut()
      .set_data(vec![0u8; HRAM_SIZE].as_ref());
    self
      .wram
      .borrow_mut()
      .set_data(vec![0u8; WRAM_SIZE].as_ref());
  }
}

impl Bus {
  pub fn ppu(&self) -> Ref<Ppu> {
    self.ppu.borrow()
  }

  pub fn ppu_mut(&self) -> RefMut<Ppu> {
    self.ppu.borrow_mut()
  }

  pub fn pad(&self) -> Ref<Pad> {
    self.pad.borrow()
  }

  pub fn pad_mut(&self) -> RefMut<Pad> {
    self.pad.borrow_mut()
  }

  pub fn set_boot(&mut self, boot: Shared<Boot>) {
    self.boot = boot
  }

  pub fn set_cart(&mut self, cart: Shared<Cartridge>) {
    self.cart = cart
  }

  pub fn set_ppu(&mut self, ppu: Shared<Ppu>) {
    self.ppu = ppu
  }

  pub fn set_wram(&mut self, wram: Shared<WRam>) {
    self.wram = wram
  }

  pub fn set_hram(&mut self, hram: Shared<HRam>) {
    self.hram = hram
  }

  pub fn set_pad(&mut self, pad: Shared<Pad>) {
    self.pad = pad
  }

  pub fn set_dma(&mut self, dma: Shared<Dma>) {
    self.dma = dma
  }
}

impl Bus {
  pub fn write_boot(&mut self, data: &[u8]) {
    self.boot.borrow_mut().set_data(data);
  }

  pub fn read_many(&self, addr: u16, count: u16) -> Vec<u8> {
    let mut data: Vec<u8> = vec![];
    for index in 0..count {
      let byte = self.read(addr + index);
      data.push(byte);
    }
    data
  }

  pub fn write_many(&mut self, addr: u16, data: &[u8]) {
    for (index, byte) in data.iter().enumerate() {
      self.write(addr + index as u16, *byte);
    }
  }
}

impl Address for Bus {
  fn read(&self, addr: u16) -> u8 {
    match addr & 0xF000 {
      // BOOT (256 B) + ROM0 (4 KB/16 KB)
      0x0000 => {
        if self.boot.borrow().boot_active() && addr <= 0x00FE {
          return self.boot.borrow().read(addr);
        }
        self.cart.read(addr)
      },
      // ROM 0 (12 KB/16 KB)
      0x1000 | 0x2000 | 0x3000 => self.cart.read(addr),
      // ROM 1 (Banked) (16 KB)
      0x4000 | 0x5000 | 0x6000 | 0x7000 => self.cart.read(addr),
      // Graphics: VRAM (8 KB)
      0x8000 | 0x9000 => self.ppu.read(addr),
      // External RAM (8 KB)
      0xa000 | 0xb000 => self.cart.read(addr),
      // Working RAM 0 (4 KB)
      0xc000 => self.wram.read(addr & 0x0fff),
      // Working RAM 1 (Banked) (4KB)
      0xd000 => self.wram.read(0x1000 + (addr & 0x0fff)),
      // Working RAM Shadow
      0xe000 => self.wram.read(addr & 0x1fff),
      // Working RAM Shadow, I/O, Zero-page RAM
      0xf000 => match addr & 0x0f00 {
        0x000 | 0x100 | 0x200 | 0x300 | 0x400 | 0x500 | 0x600 | 0x700 | 0x800 | 0x900 | 0xa00
        | 0xb00 | 0xc00 | 0xd00 => self.wram.read(addr & 0x1fff),
        0xe00 => self.ppu.read(addr),
        0xf00 => match addr & 0x00ff {
          // 0xFF01-0xFF02 - Serial data transfer
          // 0x01..=0x02 => self.serial.read(addr),
          // 0xFF0F — IF: Interrupt flag
          0x0f => {
            (if self.ppu().int_vblank() { 0x01 } else { 0x00 }
              | if self.ppu().int_stat() { 0x02 } else { 0x00 }
              // | if self.timer.int_tima() { 0x04 } else { 0x00 }
              // | if self.serial.int_serial() { 0x08 } else { 0x00 }
              | if self.pad().int_pad() { 0x10 } else { 0x00 })
          },
          // 0xFF50 - Boot active flag
          0x50 => u8::from(!self.boot.borrow().boot_active()),
          // 0xFF80-0xFFFE - High RAM (HRAM)
          0x80..=0xfe => self.hram.read(addr & 0x007f),
          // 0xFFFF — IE: Interrupt enable
          0xff => self.ie,
          // Other registers
          _ => match addr & 0x00f0 {
            0x00 => match addr & 0x00ff {
              0x00 => self.pad.read(addr),
              0x04..=0x07 => {
                debug!(
                  "Reading from unimplemented IO control Timer at 0x{:04x}",
                  addr
                );
                0xFF
              },
              _ => {
                debug!("Reading from unknown IO control 0x{:04x}", addr);
                0xFF
              },
            },
            0x40 | 0x60 | 0x70 => match addr & 0x00ff {
              // 0xFF46 — DMA: OAM DMA source address & start
              0x0046 => self.dma.read(addr),
              // VRAM related read
              _ => self.ppu.read(addr),
            },
            0x50 => match addr & 0x00ff {
              0x51..=0x55 => self.dma.read(addr),
              _ => {
                debug!("Reading from unknown IO control 0x{:04x}", addr);
                0xFF
              },
            },
            _ => {
              debug!("Reading from unknown IO control 0x{:04x}", addr);
              0xFF
            },
          },
        },
        addr => panic!("Reading from unknown location 0x{:04x}", addr),
      },

      addr => panic!("Reading from unknown location 0x{:04x}", addr),
    }
  }

  fn write(&mut self, addr: u16, value: u8) {
    match addr & 0xf000 {
      // BOOT (256 B) + ROM0 (4 KB/16 KB)
      0x0000 => self.cart.write(addr, value),
      // ROM 0 (12 KB/16 KB)
      0x1000 | 0x2000 | 0x3000 => self.cart.write(addr, value),
      // ROM 1 (Banked) (16 KB)
      0x4000 | 0x5000 | 0x6000 | 0x7000 => self.cart.write(addr, value),
      // Graphics: VRAM (8 KB)
      0x8000 | 0x9000 => self.ppu.write(addr, value),
      // External RAM (8 KB)
      0xa000 | 0xb000 => self.cart.write(addr, value),
      // Working RAM 0 (4 KB)
      0xc000 => self.wram.write(addr & 0x0FFF, value),
      // Working RAM 1 (Banked) (4KB)
      0xd000 => self.wram.write(0x1000 + (addr & 0x0fff), value),
      // Working RAM Shadow
      0xe000 => self.wram.write(addr & 0x1fff, value),
      // Working RAM Shadow, I/O, Zero-page RAM
      0xf000 => match addr & 0x0f00 {
        0x000 | 0x100 | 0x200 | 0x300 | 0x400 | 0x500 | 0x600 | 0x700 | 0x800 | 0x900 | 0xa00
        | 0xb00 | 0xc00 | 0xd00 => {
          self.wram.write(addr & 0x1fff, value);
        },
        0xe00 => self.ppu.write(addr, value),
        0xf00 => match addr & 0x00ff {
          // 0xFF01-0xFF02 - Serial data transfer
          // 0x01..=0x02 => self.serial.write(addr, value),
          // 0xFF0F — IF: Interrupt flag
          0x0f => {
            self.ppu_mut().set_int_vblank(value & 0x01 == 0x01);
            self.ppu_mut().set_int_stat(value & 0x02 == 0x02);
            // self.timer.set_int_tima(value & 0x04 == 0x04);
            // self.serial.set_int_serial(value & 0x08 == 0x08);
            self.pad_mut().set_int_pad(value & 0x10 == 0x10);
          },
          // 0xFF50 - Boot active flag
          0x50 => self.boot.borrow_mut().set_active(value == 0x00),
          // 0xFF80-0xFFFE - High RAM (HRAM)
          0x80..=0xfe => self.hram.write(addr & 0x007f, value),
          // 0xFFFF — IE: Interrupt enable
          0xff => self.ie = value,
          // Other registers
          _ => match addr & 0x00f0 {
            0x00 => {
              if addr & 0x00ff == 0x00 {
                self.pad.write(addr, value)
              } else {
                debug!("Writing to unknown IO control 0x{:04x}", addr)
              }
            },
            0x40 | 0x60 | 0x70 => match addr & 0x00ff {
              // 0xFF46 — DMA: OAM DMA source address & start
              0x0046 => self.dma.write(addr, value),
              // VRAM related write
              _ => self.ppu.write(addr, value),
            },
            #[allow(clippy::single_match)]
            0x50 => match addr & 0x00ff {
              0x51..=0x55 => self.dma.write(addr, value),
              _ => debug!("Writing to unknown IO control 0x{:04x}", addr),
            },
            _ => debug!("Writing to unknown IO control 0x{:04x}", addr),
          },
        },
        addr => panic!("Writing to unknown location 0x{:04x}", addr),
      },
      addr => panic!("Writing to unknown location 0x{:04x}", addr),
    }
  }
}

impl Device for Bus {}
