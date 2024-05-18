#![allow(dead_code)]

use std::cmp::Ordering;
use std::iter;

use log::{error, info, trace, warn};

use crate::error::CartError;
use crate::generic::device::{Device, Dynamic};
use crate::generic::memory::ram::Ram;
use crate::generic::memory::rom::Rom;
use crate::generic::pcb::Board;
use crate::hardware::cartridge::mbc::{Kind, MBC0, MBC1};
// use crate::hardware::Unmapped;

use self::header::Header;
use self::mbc::Mbc;

mod header;
mod licensee;
mod mbc;

/// Nintendo logo.
///
/// ```text
/// ██▄  ██ ██        ▄▄                   ██
/// ██▀▄ ██ ▄▄ ▄▄ ▄▄ ▀██▀ ▄▄▄▄  ▄▄ ▄▄   ▄▄▄██  ▄▄▄▄
/// ██ ▀▄██ ██ ██▀ ██ ██ ██▄▄██ ██▀ ██ ██  ██ ██  ██
/// ██  ▀██ ██ ██  ██ ██ ▀█▄▄▄▄ ██  ██ ▀█▄▄██ ▀█▄▄█▀
/// ```
///
/// Compressed copy of Nintendo's logo rendered by the boot ROM. The console
/// will refuse to pass control to cartridges that do not contain an exact copy
/// of this data.
const LOGO: [u8; 0x30] = [
  0xce, 0xed, 0x66, 0x66, 0xcc, 0x0d, 0x00, 0x0b, 0x03, 0x73, 0x00, 0x83, 0x00, 0x0c, 0x00, 0x0d,
  0x00, 0x08, 0x11, 0x1f, 0x88, 0x89, 0x00, 0x0e, 0xdc, 0xcc, 0x6e, 0xe6, 0xdd, 0xdd, 0xd9, 0x99,
  0xbb, 0xbb, 0x67, 0x63, 0x6e, 0x0e, 0xec, 0xcc, 0xdd, 0xdc, 0x99, 0x9f, 0xbb, 0xb9, 0x33, 0x3e,
];

type RomPart = (Vec<u8>, Vec<Dynamic<u16, u8>>);
fn partition_rom(data: &[u8]) -> Result<RomPart, CartError> {
  let mut parts = Vec::new();
  let bank_num = data.len() / 0x4000;
  let rom0 = data[..0x4000].to_vec();
  for i in 1..bank_num {
    let start = i * 0x4000;
    let end = start + 0x4000;

    let mut array = [0u8; 0x4000];
    array.copy_from_slice(&data[start..end]);

    let rom = Rom::<u8, 0x4000>::from(&*Box::new(array)).to_dynamic();
    parts.push(rom);
  }
  Ok((rom0, parts))
}

fn partition_ram(data: &[u8]) -> Result<Vec<Dynamic<u16, u8>>, CartError> {
  let mut parts = Vec::new();
  let bank_num = data.len() / 0x2000;
  for i in 0..bank_num {
    let start = i * 0x2000;
    let end = start + 0x2000;

    let mut array = [0u8; 0x2000];
    array.copy_from_slice(&data[start..end]);

    let ram = Ram::<u8, 0x2000>::from(&*Box::new(array)).to_dynamic();
    parts.push(ram);
  }
  Ok(parts)
}

/// Removeable cartridge for GameBoy.
///
/// Parse the [`Header`] and [`MBC`] of a cartridge.
#[derive(Debug)]
#[allow(dead_code)]
pub struct Cartridge {
  // Metadate
  header: Header,
  // Memory
  body: Box<dyn Mbc>,
}

impl Cartridge {
  /// Constructs a new `Cartridge` return an error when pares filed.
  pub fn new(rom: Vec<u8>) -> Result<Self, CartError> {
    let header = Header::new(&rom)?;
    let body = Self::body(&header, &rom)?;

    Ok(Self { header, body })
  }

  fn body(head: &Header, rom: &[u8]) -> Result<Box<dyn Mbc>, CartError> {
    // prepare ROM
    let rom = {
      let read = rom.len();
      match read.cmp(&head.romsz) {
        Ordering::Less => {
          warn!(
            "loaded {} bytes; remaining {} bytes uninitialized",
            read,
            head.romsz - read
          );
        },
        Ordering::Equal => {
          info!("loaded {} bytes", read);
        },
        Ordering::Greater => {
          warn!(
            "loaded {} bytes; truncating {} bytes",
            read,
            read - head.romsz
          );
        },
      }

      rom
        .iter()
        .copied()
        .chain(iter::repeat(0xffu8))
        .take(head.romsz)
        .collect::<Vec<u8>>()
        .into_boxed_slice()
    };

    trace!("ROM:\n{:?}", rom);

    // // Construct ROM
    // let rom: Dynamic<u16, u8> = match head.romsz {
    //   // 无 bank
    //   0x0000_8000 => Rom::<u8, 0x0000_8000>::from(
    //     &*Box::<[_; 0x0000_8000]>::try_from(rom).map_err(CartError::Mismatch)?,
    //   )
    //   .to_dynamic(),
    //   // 4 banks x 0x4000
    //   0x0001_0000 => Rom::<u8, 0x0001_0000>::from(
    //     &*Box::<[_; 0x0001_0000]>::try_from(rom).map_err(CartError::Mismatch)?,
    //   )
    //   .to_dynamic(),
    //   // 8 banks x 0x4000
    //   0x0002_0000 => Rom::<u8, 0x0002_0000>::from(
    //     &*Box::<[_; 0x0002_0000]>::try_from(rom).map_err(CartError::Mismatch)?,
    //   )
    //   .to_dynamic(),
    //   // 16 banks x 0x4000
    //   0x0004_0000 => Rom::<u8, 0x0004_0000>::from(
    //     &*Box::<[_; 0x0004_0000]>::try_from(rom).map_err(CartError::Mismatch)?,
    //   )
    //   .to_dynamic(),
    //   // 32 banks x 0x4000
    //   0x0008_0000 => Rom::<u8, 0x0008_0000>::from(
    //     &*Box::<[_; 0x0008_0000]>::try_from(rom).map_err(CartError::Mismatch)?,
    //   )
    //   .to_dynamic(),
    //   // 64 banks x 0x4000
    //   0x0010_0000 => Rom::<u8, 0x0010_0000>::from(
    //     &*Box::<[_; 0x0010_0000]>::try_from(rom).map_err(CartError::Mismatch)?,
    //   )
    //   .to_dynamic(),
    //   // 128 banks x 0x4000
    //   0x0020_0000 => Rom::<u8, 0x0020_0000>::from(
    //     &*Box::<[_; 0x0020_0000]>::try_from(rom).map_err(CartError::Mismatch)?,
    //   )
    //   .to_dynamic(),
    //   // 256 banks x 0x4000
    //   0x0040_0000 => Rom::<u8, 0x0040_0000>::from(
    //     &*Box::<[_; 0x0040_0000]>::try_from(rom).map_err(CartError::Mismatch)?,
    //   )
    //   .to_dynamic(),
    //   // 512 banks x 0x4000
    //   0x0080_0000 => Rom::<u8, 0x0080_0000>::from(
    //     &*Box::<[_; 0x0080_0000]>::try_from(rom).map_err(CartError::Mismatch)?,
    //   )
    //   .to_dynamic(),
    //   _ => unreachable!(),
    // };

    // Construct RAM
    // let ram: Dynamic<u16, u8> = match head.ramsz {
    //   // 0x00000 => Unmapped::<0x2000>::new().to_dynamic(),
    //   0x00000 => Ram::<u8, 0>::new().to_dynamic(),
    //   0x00800 => Ram::<u8, 0x00800>::new().to_dynamic(),
    //   0x02000 => Ram::<u8, 0x02000>::new().to_dynamic(),
    //   0x08000 => Ram::<u8, 0x08000>::new().to_dynamic(),
    //   0x20000 => Ram::<u8, 0x20000>::new().to_dynamic(),
    //   0x10000 => Ram::<u8, 0x10000>::new().to_dynamic(),
    //   _ => unreachable!(),
    // };
    let ram = match head.ramsz {
      // TODO: fix no ram
      0x00000 => vec![],
      0x00800 => vec![0u8; 0x00800],
      0x02000 => vec![0u8; 0x02000],
      0x08000 => vec![0u8; 0x08000],
      0x20000 => vec![0u8; 0x20000],
      0x10000 => vec![0u8; 0x10000],
      _ => unreachable!(),
    };

    let mbc: Box<dyn Mbc> = match head.cart {
      Kind::RomOnly => Box::new(MBC0::with(
        Rom::<u8, 0x8000>::from(&*Box::<[_; 0x8000]>::try_from(rom).map_err(CartError::Mismatch)?)
          .to_dynamic(),
      )),
      Kind::MBC1 | Kind::MBC1RAM | Kind::MBC1RAMBATT => {
        let rom_part = partition_rom(&rom)?;
        let ram_part = partition_ram(&ram)?;
        Box::new(MBC1::with(rom_part, ram_part))
      },
      _ => {
        error!("Unsupported cartridge type: {:?}", head.cart);
        // TODO: resolve panic
        panic!()
      },
    };

    Ok(mbc)
  }

  pub fn header(&self) -> &Header {
    &self.header
  }

  fn rom(&self) -> Dynamic<u16, u8> {
    self.body.rom()
  }

  fn ram(&self) -> Dynamic<u16, u8> {
    self.body.ram()
  }
}

impl Board<u16, u8> for Cartridge {
  fn connect(&self, bus: &mut crate::generic::bus::Bus<u16, u8>) {
    let rom = self.rom();
    let ram = self.ram();

    bus.map(0x0000..=0x7FFF, rom);
    bus.map(0xA000..=0xBFFF, ram);
  }
}
