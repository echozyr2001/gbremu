use crate::generic::{address::Address, device::Device, memory::Rom};

#[derive(Default)]
pub struct Boot {
  data: Rom,
  boot_active: bool,
}

impl Boot {
  pub fn new() -> Self {
    Self {
      data: Rom::default(),
      boot_active: true,
    }
  }

  pub fn set_data(&mut self, data: &[u8]) {
    self.data.set_data(data);
  }

  pub fn reset(&mut self) {
    self.data.reset();
    self.boot_active = true;
  }
}

impl Boot {
  pub fn boot_active(&self) -> bool {
    self.boot_active
  }

  pub fn set_active(&mut self, active: bool) {
    self.boot_active = active;
  }
}

impl Address for Boot {
  fn read(&self, addr: u16) -> u8 {
    self.data.read(addr)
  }

  fn write(&mut self, addr: u16, value: u8) {
    self.data.write(addr, value)
  }
}

impl Device for Boot {}
