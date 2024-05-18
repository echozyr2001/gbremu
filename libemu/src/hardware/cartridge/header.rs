//! Game cartridge header.
//!
//! In the ROM at the address range `0x0100..0x0150` is the header,
//! which encodes both physical attributes describing the hardware of the
//! cartridge, flags describing console support, and characteristics of the
//! software.
use log::warn;

use crate::error::CartError;

use super::{licensee::Licensee, mbc::Kind, LOGO};

#[derive(Debug)]
pub struct Header {
  /// Equality with boot ROM's Nintendo logo.
  pub logo: bool,
  /// Title of this ROM.
  pub title: Option<String>,
  /// Licensee code.
  pub licensee: Licensee,
  /// DMG model support.
  pub dmg: bool,
  /// CGB model support.
  pub cgb: bool,
  /// SGB model support.
  pub sgb: bool,
  /// Cartridge hardware.
  pub cart: Kind,
  /// ROM size in bytes.
  pub romsz: usize,
  /// ROM size in bytes.
  pub ramsz: usize,
  /// Destination code (Japan/Worldwide)
  pub jpn: bool,
  /// Revision number of this ROM.
  pub version: u8,
  /// 8-bit header checksum.
  pub hchk: u8,
  /// 16-bit global checksum.
  pub gchk: u16,
}

/// Cartridge header.
///
/// Information about the ROM and the cartridge containing it. Stored in the
/// byte range `[0x100, 0x150)`.
impl Header {
  pub fn new(rom: &[u8]) -> Result<Self, CartError> {
    // slice the header
    let header: &[u8; 0x50] = rom
      .get(0x100..=0x14F)
      .ok_or(CartError::Missing)?
      .try_into()
      .map_err(CartError::Slice)?;

    // check logo
    let logo = header[0x04..=0x33] == LOGO;
    if !logo {
      return Err(CartError::Logo);
    }

    // parse title
    // TODO: check title parser
    let tlen = if header[0x43] & 0x80 == 0 { 16 } else { 15 };
    let title = match std::str::from_utf8(&header[0x34..0x34 + tlen])
      .map_err(CartError::Title)?
      .trim_matches('\0')
    {
      "" => None,
      ok => Some(ok),
    }
    .map(ToString::to_string);

    // parse licensee code
    let licensee: Licensee = if header[0x4B] == 0x33 {
      u16::from_le_bytes([header[0x44], header[0x45]]).try_into()?
    } else {
      header[0x4B].try_into()?
    };

    let dmg = header[0x43] & 0x80 == 0;

    let cgb = match header[0x43] & 0xbf {
      0x00 => Ok(false),
      0x80 => Ok(true),
      byte => Err(CartError::Color(byte)),
    }?;

    let sgb = match header[0x46] {
      0x00 => false,
      0x03 => true,
      byte => {
        warn!("Invalid SGB flag: {:#04x}", byte);
        false
      },
    };

    // parse cartridge type
    let cart: Kind = header[0x47].try_into()?;

    // parse ROM size
    let romsz = match header[0x48] {
      byte @ 0x00..=0x08 => Ok(0x8000 << byte),
      byte => Err(CartError::Rom(byte)),
    }?;

    // parse RAM size
    let ramsz = match header[0x49] {
      0x00 => Ok(0),
      0x01 => Ok(0x800),
      0x02 => Ok(0x2000),
      0x03 => Ok(0x8000),
      0x04 => Ok(0x20000),
      0x05 => Ok(0x10000),
      byte => Err(CartError::Ram(byte)),
    }?;

    // parse region
    let jpn = match header[0x4A] {
      0x00 => Ok(true),
      0x01 => Ok(false),
      byte => Err(CartError::Region(byte)),
    }?;

    // parse version
    let version = header[0x4C];

    // parse header checksum
    let hchk = header[0x4D];
    let chk = rom[0x134..=0x14c]
      .iter()
      .copied()
      .fold(0u8, |accum, itme| accum.wrapping_sub(itme).wrapping_sub(1));
    if chk != hchk {
      return Err(CartError::HeaderCheck {
        found: chk,
        expected: hchk,
      });
    }

    // parse global checksum
    let gchk = u16::from_le_bytes([header[0x4E], header[0x4F]]);
    let chk = rom
      .iter()
      .copied()
      .fold(0u16, |accum, item| accum.wrapping_add(item as u16))
      .wrapping_sub(rom[0x14E] as u16)
      .wrapping_sub(rom[0x14F] as u16);
    if chk != gchk {
      // Gameboy doesn't verify this checksum
      warn!(
        "Global checksum mismatch: found {:#04x}, expected {:#04x}",
        chk, gchk
      )
    }

    Ok(Self {
      logo,
      title,
      licensee,
      dmg,
      cgb,
      sgb,
      cart,
      romsz,
      ramsz,
      jpn,
      version,
      hchk,
      gchk,
    })
  }
}

impl std::fmt::Display for Header {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    writeln!(f, "┌──────────────────┐")?;
    writeln!(f, "│ {:^16} │", self.title.as_deref().unwrap_or("Unknown"))?;
    writeln!(f, "├──────────────────┤")?;
    writeln!(f, "│Licensee: {:>10}│", self.licensee)?;
    writeln!(f, "├──────────────────┤")?;
    writeln!(
      f,
      "│ Model: {:>9} │",
      match (self.dmg, self.cgb) {
        (false, false) => "None",
        (false, true) => "CGB",
        (true, false) => "DMG",
        (true, true) => "DMG + CGB",
      }
    )?;
    writeln!(f, "│ SGB: {:>11} │", self.sgb)?;
    writeln!(f, "├──────────────────┤")?;
    writeln!(f, "│ MBC: {:>11} │", self.cart)?;
    writeln!(f, "├──────────────────┤")?;
    writeln!(f, "│ ROM: {:>9} B │", self.romsz)?;
    writeln!(f, "│ RAM: {:>9} B │", self.ramsz)?;
    writeln!(f, "├──────────────────┤")?;
    writeln!(
      f,
      "│ Region: {:>8} │",
      if self.jpn { "Japan" } else { "World" }
    )?;
    writeln!(
      f,
      "│ Version: {:>7} │",
      format!(
        "v{}.{}",
        ((self.version & 0xf0) >> 4) + 1,
        self.version & 0x0f
      )
    )?;
    writeln!(f, "├──────────────────┤")?;
    writeln!(f, "│ Header:       {:0>2x} │", self.hchk)?;
    writeln!(f, "│ Global:     {:0>4x} │", self.gchk)?;
    write!(f, "└──────────────────┘")
  }
}

#[cfg(test)]
mod test {
  #[test]
  fn test_header() {
    use dotenv::dotenv;
    use log::debug;

    use super::Header;
    use std::{env, fs::File, io::Read};

    dotenv().ok();
    let filepath = env::var("ROM_PATH").expect("HEADER_TEST_PATH must be set");
    let mut rom = vec![];

    let file = File::open(filepath);
    match file.and_then(|mut f| f.read_to_end(&mut rom)) {
      Ok(_) => {},
      Err(e) => panic!("Error reading file: {}", e),
    };
    env_logger::init();

    match Header::new(&rom) {
      Ok(header) => {
        debug!("{}", header);
      },
      Err(e) => {
        debug!("{:?}", e);
        panic!("Error parsing header: {:?}", e);
      },
    }
  }
}
