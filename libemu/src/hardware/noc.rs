// use crate::generic::share::Shared;

// use super::Bus;

use crate::generic::{device::Device, share::Shared};

use super::Bus;

/// Memory bus architecture.
#[derive(Debug, Default)]
pub struct NoC {
  // /// Internal bus.
  // pub internal: Shared<Bus>,
  // /// External bus.
  // pub external: Shared<Bus>,
  // /// Video bus.
  // pub video: Shared<Bus>,
  pub bus: Shared<Bus>,
}

impl NoC {
  pub fn cpu(&self) -> Bus {
    let bus = self.bus.clone();
    let mut mmap = Bus::new();
    mmap.map(0x0000..=0xFFFF, bus.to_dynamic());

    mmap
  }
}
