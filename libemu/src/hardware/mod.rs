#![allow(dead_code)]

pub mod cartridge;
pub mod noc;
pub mod soc;

pub type Bus = crate::generic::bus::Bus<u16, u8>;

pub type HRam = crate::generic::memory::ram::Ram<u8, 0x7F>;
