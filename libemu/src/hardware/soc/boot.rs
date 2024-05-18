use crate::generic::{arch::address::Address, device::Device, memory::rom::Rom, pcb::Board};

#[derive(Debug)]
pub struct Boot {
  data: Rom<u8, 0x100>,
  ctrl: BootCtrl,
}

#[derive(Debug, Default, Clone)]
struct BootCtrl(u8);
impl Device<u16, u8> for BootCtrl {}
impl Address<u16, u8> for BootCtrl {
  fn read(&self, _idx: u16) -> u8 {
    self.0
  }

  fn write(&mut self, _idx: u16, val: u8) {
    self.0 = val;
  }
}

impl Boot {
  pub fn with(data: Rom<u8, 0x100>) -> Self {
    Self {
      data,
      ctrl: BootCtrl::default(),
    }
  }
}

impl Device<u16, u8> for Boot {}

impl Address<u16, u8> for Boot {
  fn read(&self, addr: u16) -> u8 {
    self.data.read(addr)
  }

  fn write(&mut self, addr: u16, data: u8) {
    self.data.write(addr, data)
  }
}

impl Board<u16, u8> for Boot {
  fn connect(&self, bus: &mut crate::generic::bus::Bus<u16, u8>) {
    bus.map(0x00..=0xFF, self.data.clone().to_dynamic());
    bus.map(0xFF50..=0xFF50, self.ctrl.clone().to_dynamic());
  }
}
