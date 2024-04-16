use nom::{
  bytes::complete::take,
  combinator::{map, map_opt, map_res},
  error::{context, VerboseError},
  sequence::tuple,
  IResult,
};

#[derive(Debug)]
pub struct CartridgeHeader<'a> {
  /// 0x0100 - 0x0103
  entry_point: &'a [u8; 0x4],
  /// 0x104 - 0x0133
  nintendo_logo: &'a [u8; 0x30],
  /// 0x0134 - 0x0143
  /// up to 16 characters
  /// TODO: support for 16 bytes
  /// 0x0134 - 0x013E
  title: &'a str,
  /// 0x013F - 0x0142
  manufacturer_code: &'a str,
  /// 0x0143
  /// 0x80 for Compatible, 0xC0 for Exclusive, 0x00 for ?
  gbc_flag: GameBoyColorFlag,
  /// 0x0144 - 0x0145
  /// must old licensee code is 0x33, the new licensee code has its meaning
  new_licensee_code: &'a [u8; 0x2],
  /// 0x0146
  /// 0x03(true) for Super GameBoy, 0x00(false) for non-Super GameBoy
  sgb_flag: bool,
  /// 0x0147
  cartridge_type: CartridgeType,
  /// 0x0148
  /// for KB
  rom_size: u16,
  /// 0x0149
  /// for KB
  ram_size: (u8, u8),
  /// 0x014A
  /// 0x00(true) for Japan(and possibly overseas), 0x01(false) for overseas only
  destination_code: bool,
  /// 0x014B
  /// if the old licensee code is 0x33, the new licensee code is used
  old_licensee_code: u8,
  /// 0x014C
  /// usually 0x00
  mask_rom_version: u8,
  /// 0x014D
  header_checksum: u8,
  /// 0x014E - 0x014F
  global_checksum: u16,
}

/// Declare use of the GameBoy Color features
#[derive(Debug)]
enum GameBoyColorFlag {
  /// The cartridge indicates that it does not make use of any Gameboy Color enhancements
  Monochrome,
  /// The cartridge supports but does not require Gameboy Color
  ColorOptional,
  /// The cartridge requires Gameboy Color enhancements
  ColorRequired,
}

#[derive(Debug)]
enum CartridgeType {
  ROMONLY,
  MBC1,
  MBC1RAM,
  MBC1RAMBATTERY,
  MBC2,
  MBC2BATTERY,
  ROMRAM,
  ROMRAMBATTERY,
  MMM01,
  MMM01RAM,
  MMM01RAMBATTERY,
  MBC3TIMERBATTERY,
  MBC3TIMERRAMBATTERY,
  MBC3,
  MBC3RAM,
  MBC3RAMBATTERY,
  MBC5,
  MBC5RAM,
  MBC5RAMBATTERY,
  MBC5RUMBLE,
  MBC5RUMBLERAM,
  MBC5RUMBLERAMBATTERY,
  MBC6,
  MBC7SENSORRUMBLERAMBATTERY,
  POCKETCAMERA,
  BANDAITAMA5,
  HuC3,
  HuC1RAMBATTERY,
}

impl From<u8> for CartridgeType {
  fn from(byte: u8) -> Self {
    match byte {
      0x00 => CartridgeType::ROMONLY,
      0x01 => CartridgeType::MBC1,
      0x02 => CartridgeType::MBC1RAM,
      0x03 => CartridgeType::MBC1RAMBATTERY,
      0x05 => CartridgeType::MBC2,
      0x06 => CartridgeType::MBC2BATTERY,
      0x08 => CartridgeType::ROMRAM,
      0x09 => CartridgeType::ROMRAMBATTERY,
      0x0B => CartridgeType::MMM01,
      0x0C => CartridgeType::MMM01RAM,
      0x0D => CartridgeType::MMM01RAMBATTERY,
      0x0F => CartridgeType::MBC3TIMERBATTERY,
      0x10 => CartridgeType::MBC3TIMERRAMBATTERY,
      0x11 => CartridgeType::MBC3,
      0x12 => CartridgeType::MBC3RAM,
      0x13 => CartridgeType::MBC3RAMBATTERY,
      0x19 => CartridgeType::MBC5,
      0x1A => CartridgeType::MBC5RAM,
      0x1B => CartridgeType::MBC5RAMBATTERY,
      0x1C => CartridgeType::MBC5RUMBLE,
      0x1D => CartridgeType::MBC5RUMBLERAM,
      0x1E => CartridgeType::MBC5RUMBLERAMBATTERY,
      0x20 => CartridgeType::MBC6,
      0x22 => CartridgeType::MBC7SENSORRUMBLERAMBATTERY,
      0xFC => CartridgeType::POCKETCAMERA,
      0xFD => CartridgeType::BANDAITAMA5,
      0xFE => CartridgeType::HuC3,
      0xFF => CartridgeType::HuC1RAMBATTERY,
      _ => unreachable!(),
    }
  }
}

fn parse_start<'a>(input: &'a [u8]) -> IResult<&'a [u8], &'a [u8], VerboseError<&'a [u8]>> {
  context("Rom start", take(0x100usize))(input)
}

fn parse_entry_point<'a>(
  input: &'a [u8],
) -> IResult<&'a [u8], &'a [u8; 4], VerboseError<&'a [u8]>> {
  context(
    "entry point",
    map_res(take(0x4usize), |bytes: &'a [u8]| bytes.try_into()),
  )(input)
}

fn parse_nintendo_logo<'a>(
  input: &'a [u8],
) -> IResult<&'a [u8], &'a [u8; 0x30], VerboseError<&'a [u8]>> {
  context(
    "nintendo logo",
    map_res(take(0x30usize), |bytes: &'a [u8]| bytes.try_into()),
  )(input)
}

fn parse_title<'a>(input: &'a [u8]) -> IResult<&'a [u8], &'a str, VerboseError<&'a [u8]>> {
  context(
    "title",
    map_res(take(0xBusize), |bytes: &[u8]| {
      std::str::from_utf8(bytes).map(|s| s.trim_end_matches('\0'))
    }),
  )(input)
}

fn parse_manufacturer_code<'a>(
  input: &'a [u8],
) -> IResult<&'a [u8], &'a str, VerboseError<&'a [u8]>> {
  context(
    "manufacturer code",
    map_res(take(0x4usize), std::str::from_utf8),
  )(input)
}

fn parse_gbc_flag<'a>(
  input: &'a [u8],
) -> IResult<&'a [u8], GameBoyColorFlag, VerboseError<&'a [u8]>> {
  context(
    "gbc flag",
    map_res(take(0x0001usize), |byte: &'a [u8]| {
      Ok::<GameBoyColorFlag, VerboseError<&'a [u8]>>(match byte[0] {
        0x80 => GameBoyColorFlag::ColorRequired,
        0xC0 => GameBoyColorFlag::ColorOptional,
        _ => GameBoyColorFlag::Monochrome,
      })
    }),
  )(input)
}

fn parse_cartridge_type<'a>(
  input: &'a [u8],
) -> IResult<&'a [u8], CartridgeType, VerboseError<&'a [u8]>> {
  context(
    "cartridge type",
    map_res(take(0x0001usize), |c_type: &'a [u8]| {
      Ok::<CartridgeType, VerboseError<&'a [u8]>>(CartridgeType::from(c_type[0]))
    }),
  )(input)
}

fn parse_new_licensee_code<'a>(
  input: &'a [u8],
) -> IResult<&'a [u8], &'a [u8; 2], VerboseError<&'a [u8]>> {
  context(
    "new licensee code",
    map_res(take(2usize), |bytes: &'a [u8]| bytes.try_into()),
  )(input)
}

fn parse_sgb_flag<'a>(input: &'a [u8]) -> IResult<&'a [u8], bool, VerboseError<&'a [u8]>> {
  context(
    "sgb flag",
    map_res(take(0x0001usize), |byte: &'a [u8]| {
      Ok::<bool, VerboseError<&'a [u8]>>(byte[0] == 0x03)
    }),
  )(input)
}

fn parse_rom_size<'a>(input: &'a [u8]) -> IResult<&'a [u8], u16, VerboseError<&'a [u8]>> {
  context(
    "rom size",
    map_res(take(0x0001usize), |byte: &'a [u8]| {
      Ok::<u16, VerboseError<&'a [u8]>>(32 << byte[0])
    }),
  )(input)
}

fn parse_ram_size<'a>(input: &'a [u8]) -> IResult<&'a [u8], (u8, u8), VerboseError<&'a [u8]>> {
  context(
    "ram size",
    map_opt(take(0x0001usize), |byte: &'a [u8]| match byte[0] {
      0x00 => None, // No RAM
      0x01 => unreachable!("Unused"),
      0x02 => Some((1, 8)),  // 1 bank of 8KB
      0x03 => Some((4, 8)),  // 4 banks of 8KB
      0x04 => Some((16, 8)), // 16 banks of 8KB
      0x05 => Some((8, 8)),  // 8 banks of 8KB
      _ => unreachable!(),
    }),
  )(input)
}

fn parse_destination_code<'a>(input: &'a [u8]) -> IResult<&'a [u8], bool, VerboseError<&'a [u8]>> {
  context(
    "destination code",
    map_res(take(0x0001usize), |byte: &'a [u8]| {
      Ok::<bool, VerboseError<&'a [u8]>>(byte[0] == 0x00)
    }),
  )(input)
}

fn parse_old_licensee_code<'a>(input: &'a [u8]) -> IResult<&'a [u8], u8, VerboseError<&'a [u8]>> {
  context(
    "old licensee code",
    map(take(0x0001usize), |byte: &'a [u8]| byte[0]),
  )(input)
}

fn parse_mask_rom_version<'a>(input: &'a [u8]) -> IResult<&'a [u8], u8, VerboseError<&'a [u8]>> {
  context(
    "mask rom version",
    map(take(0x0001usize), |byte: &'a [u8]| byte[0]),
  )(input)
}

fn parse_header_checksum<'a>(input: &'a [u8]) -> IResult<&'a [u8], u8, VerboseError<&'a [u8]>> {
  context(
    "header checksum",
    map(take(0x0001usize), |byte: &'a [u8]| byte[0]),
  )(input)
}

fn parse_global_checksum<'a>(input: &'a [u8]) -> IResult<&'a [u8], u16, VerboseError<&'a [u8]>> {
  context(
    "global checksum",
    map(take(0x0002usize), |bytes: &'a [u8]| {
      u16::from_be_bytes([bytes[0], bytes[1]])
    }),
  )(input)
}

pub fn parse_cart_header<'a>(
  input: &'a [u8],
) -> IResult<&'a [u8], CartridgeHeader<'a>, VerboseError<&'a [u8]>> {
  map(
    // Combining parsers
    tuple((
      parse_start,
      parse_entry_point,
      parse_nintendo_logo,
      parse_title,
      parse_manufacturer_code,
      parse_gbc_flag,
      parse_new_licensee_code,
      parse_sgb_flag,
      parse_cartridge_type,
      parse_rom_size,
      parse_ram_size,
      parse_destination_code,
      parse_old_licensee_code,
      parse_mask_rom_version,
      parse_header_checksum,
      parse_global_checksum,
    )),
    |(
      _,
      entry_point,
      nintendo_logo,
      title,
      manufacturer_code,
      gbc_flag,
      new_licensee_code,
      sgb_flag,
      cartridge_type,
      rom_size,
      ram_size,
      destination_code,
      old_licensee_code,
      mask_rom_version,
      header_checksum,
      global_checksum,
    )| CartridgeHeader {
      entry_point,
      nintendo_logo,
      title,
      manufacturer_code,
      gbc_flag,
      new_licensee_code,
      sgb_flag,
      cartridge_type,
      rom_size,
      ram_size,
      destination_code,
      old_licensee_code,
      mask_rom_version,
      header_checksum,
      global_checksum,
    },
  )(input)
}

#[cfg(test)]
mod test {
  use dotenv::dotenv;
  use log::debug;

  use super::parse_cart_header;
  use std::{env, fs::File, io::Read};

  #[test]
  fn test_parse() {
    dotenv().ok();
    let filepath = env::var("ROM_PATH").expect("ROM_PATH must be set");
    let mut rom = vec![];

    let file = File::open(filepath);
    match file.and_then(|mut f| f.read_to_end(&mut rom)) {
      Ok(_) => {},
      Err(e) => assert!(false, "Error reading file: {}", e),
    };
    env_logger::init();

    match parse_cart_header(&rom) {
      Ok((_, header)) => {
        debug!("{:#?}", header);
      },
      Err(e) => {
        debug!("{:?}", e);
        assert!(false, "Error parsing header: {:?}", e);
      },
    }
  }
}
