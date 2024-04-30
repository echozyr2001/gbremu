use std::fmt::Display;

use crate::error::{CartError, LicenseeError};

pub enum Licensee {
  New(NewLicensee),
  Old(OldLicensee),
}

impl Display for Licensee {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::New(lic) => write!(f, "{}", lic),
      Self::Old(lic) => write!(f, "{}", lic),
    }
  }
}

impl TryFrom<u16> for Licensee {
  type Error = CartError;

  fn try_from(value: u16) -> Result<Self, Self::Error> {
    Ok(Self::New(NewLicensee::try_from(value)?))
  }
}

impl TryFrom<u8> for Licensee {
  type Error = CartError;

  fn try_from(value: u8) -> Result<Self, Self::Error> {
    Ok(Self::Old(OldLicensee::try_from(value)?))
  }
}

pub enum NewLicensee {
  None,                // 00
  NintendoRD1,         // 01
  Capcom,              // 08
  ElectronicArts,      // 13 69
  HudsonSoft,          // 18
  Bai,                 // 19
  Kss,                 // 20
  Pow,                 // 22
  PCMComplete,         // 24
  Sanx,                // 25
  KemcoJapan,          // 28
  Seta,                // 29
  Viacom,              // 30
  Nintendo,            // 31
  Bandai,              // 32
  OceanAcclaim,        // 33 93
  Konami,              // 34 54
  Hector,              // 35
  Taito,               // 37
  Hudson,              // 38
  Banpresto,           // 39
  UbiSoft,             // 41
  Atlus,               // 42
  Malibu,              // 44
  Angel,               // 46
  BulletProof,         // 47
  Irem,                // 49
  Absolute,            // 50
  Acclaim,             // 51
  Activision,          // 52
  Americansammy,       // 53
  HiTechEntertainment, // 55
  Ljn,                 // 56
  Matchbox,            // 57
  Mattel,              // 58
  MiltonBradley,       // 59
  Titus,               // 60
  Virgin,              // 61
  Lucasarts,           // 64
  Ocean,               // 67
  Infogrames,          // 70
  Interplay,           // 71
  Broderbund,          // 72
  Sculptured,          // 73
  Sci,                 // 75
  Thq,                 // 78
  Accolade,            // 79
  Misawa,              // 80
  Lozc,                // 83
  TokumaShoteni,       // 86
  TsukudaOri,          // 87
  Chunsoft,            // 91
  Videosystem,         // 92
  Varie,               // 95
  Yonezawaspal,        // 96
  Kaneko,              // 97
  PackInSoft,          // 99
  KonamiYuGiOh,        // A4
}

impl TryFrom<u16> for NewLicensee {
  type Error = CartError;

  fn try_from(value: u16) -> Result<Self, Self::Error> {
    match value {
      0x3030 => Ok(Self::None),
      0x3130 => Ok(Self::NintendoRD1),
      0x3830 => Ok(Self::Capcom),
      0x3331 | 0x3936 => Ok(Self::ElectronicArts),
      0x3831 => Ok(Self::HudsonSoft),
      0x3931 => Ok(Self::Bai),
      0x3032 => Ok(Self::Kss),
      0x3232 => Ok(Self::Pow),
      0x3432 => Ok(Self::PCMComplete),
      0x3532 => Ok(Self::Sanx),
      0x3832 => Ok(Self::KemcoJapan),
      0x3932 => Ok(Self::Seta),
      0x3033 => Ok(Self::Viacom),
      0x3133 => Ok(Self::Nintendo),
      0x3233 => Ok(Self::Bandai),
      0x3333 | 0x3339 => Ok(Self::OceanAcclaim),
      0x3433 | 0x3435 => Ok(Self::Konami),
      0x3533 => Ok(Self::Hector),
      0x3733 => Ok(Self::Taito),
      0x3833 => Ok(Self::Hudson),
      0x3933 => Ok(Self::Banpresto),
      0x3134 => Ok(Self::UbiSoft),
      0x3234 => Ok(Self::Atlus),
      0x3434 => Ok(Self::Malibu),
      0x3634 => Ok(Self::Angel),
      0x3734 => Ok(Self::BulletProof),
      0x3934 => Ok(Self::Irem),
      0x3035 => Ok(Self::Absolute),
      0x3135 => Ok(Self::Acclaim),
      0x3235 => Ok(Self::Activision),
      0x3335 => Ok(Self::Americansammy),
      0x3535 => Ok(Self::HiTechEntertainment),
      0x3635 => Ok(Self::Ljn),
      0x3735 => Ok(Self::Matchbox),
      0x3835 => Ok(Self::Mattel),
      0x3935 => Ok(Self::MiltonBradley),
      0x3036 => Ok(Self::Titus),
      0x3136 => Ok(Self::Virgin),
      0x3436 => Ok(Self::Lucasarts),
      0x3736 => Ok(Self::Ocean),
      0x3037 => Ok(Self::Infogrames),
      0x3137 => Ok(Self::Interplay),
      0x3237 => Ok(Self::Broderbund),
      0x3337 => Ok(Self::Sculptured),
      0x3537 => Ok(Self::Sci),
      0x3837 => Ok(Self::Thq),
      0x3937 => Ok(Self::Accolade),
      0x3038 => Ok(Self::Misawa),
      0x3338 => Ok(Self::Lozc),
      0x3638 => Ok(Self::TokumaShoteni),
      0x3738 => Ok(Self::TsukudaOri),
      0x3139 => Ok(Self::Chunsoft),
      0x3239 => Ok(Self::Videosystem),
      0x3539 => Ok(Self::Varie),
      0x3639 => Ok(Self::Yonezawaspal),
      0x3739 => Ok(Self::Kaneko),
      0x3939 => Ok(Self::PackInSoft),
      0x3441 => Ok(Self::KonamiYuGiOh),
      word => Err(CartError::Licensee(LicenseeError::New(word))),
    }
  }
}

impl Display for NewLicensee {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::None => write!(f, "None"),
      Self::NintendoRD1 => write!(f, "Nintendo RD1"),
      Self::Capcom => write!(f, "Capcom"),
      Self::ElectronicArts => write!(f, "Electronic Arts"),
      Self::HudsonSoft => write!(f, "Hudson Soft"),
      Self::Bai => write!(f, "Bai"),
      Self::Kss => write!(f, "Kss"),
      Self::Pow => write!(f, "Pow"),
      Self::PCMComplete => write!(f, "PCM Complete"),
      Self::Sanx => write!(f, "Sanx"),
      Self::KemcoJapan => write!(f, "Kemco Japan"),
      Self::Seta => write!(f, "Seta"),
      Self::Viacom => write!(f, "Viacom"),
      Self::Nintendo => write!(f, "Nintendo"),
      Self::Bandai => write!(f, "Bandai"),
      Self::OceanAcclaim => write!(f, "Ocean Acclaim"),
      Self::Konami => write!(f, "Konami"),
      Self::Hector => write!(f, "Hector"),
      Self::Taito => write!(f, "Taito"),
      Self::Hudson => write!(f, "Hudson"),
      Self::Banpresto => write!(f, "Banpresto"),
      Self::UbiSoft => write!(f, "Ubi Soft"),
      Self::Atlus => write!(f, "Atlus"),
      Self::Malibu => write!(f, "Malibu"),
      Self::Angel => write!(f, "Angel"),
      Self::BulletProof => write!(f, "Bullet Proof"),
      Self::Irem => write!(f, "Irem"),
      Self::Absolute => write!(f, "Absolute"),
      Self::Acclaim => write!(f, "Acclaim"),
      Self::Activision => write!(f, "Activision"),
      Self::Americansammy => write!(f, "American Sammy"),
      Self::HiTechEntertainment => write!(f, "Hi Tech Entertainment"),
      Self::Ljn => write!(f, "LJN"),
      Self::Matchbox => write!(f, "Matchbox"),
      Self::Mattel => write!(f, "Mattel"),
      Self::MiltonBradley => write!(f, "Milton Bradley"),
      Self::Titus => write!(f, "Titus"),
      Self::Virgin => write!(f, "Virgin"),
      Self::Lucasarts => write!(f, "Lucasarts"),
      Self::Ocean => write!(f, "Ocean"),
      Self::Infogrames => write!(f, "Infogrames"),
      Self::Interplay => write!(f, "Interplay"),
      Self::Broderbund => write!(f, "Broderbund"),
      Self::Sculptured => write!(f, "Sculptured"),
      Self::Sci => write!(f, "Sci"),
      Self::Thq => write!(f, "THQ"),
      Self::Accolade => write!(f, "Accolade"),
      Self::Misawa => write!(f, "Misawa"),
      Self::Lozc => write!(f, "Lozc"),
      Self::TokumaShoteni => write!(f, "Tokuma Shoteni"),
      Self::TsukudaOri => write!(f, "Tsukuda Ori"),
      Self::Chunsoft => write!(f, "Chunsoft"),
      Self::Videosystem => write!(f, "Videosystem"),
      Self::Varie => write!(f, "Varie"),
      Self::Yonezawaspal => write!(f, "Yonezawa Spal"),
      Self::Kaneko => write!(f, "Kaneko"),
      Self::PackInSoft => write!(f, "Pack In Soft"),
      Self::KonamiYuGiOh => write!(f, "Konami Yu-Gi-Oh"),
    }
  }
}
pub enum OldLicensee {
  None,             // 0x00
  Nintendo,         // 0x01 0x31
  Capcom,           // 0x08 0x38
  HotB,             // 0x09
  Jaleco,           // 0x0A 0xE0
  Coconuts,         // 0x0B
  EliteSystems,     // 0x0C 0x6E
  ElectronicArts,   // 0x13 0x69
  Hudsonsoft,       // 0x18
  ITCEntertainment, // 0x19
  Yanoman,          // 0x1A
  Clary,            // 0x1D
  Virgin,           // 0x1F 0x4A 0x61
  Kss,              // 0x20
  PCMComplete,      // 0x24
  SanX,             // 0x25
  KotobukiSystems,  // 0x28
  Seta,             // 0x29
  Infogrames,       // 0x30 0x70
  Bandai,           // 0x32 0xA2 0xB2
  // 0x33 Indicates that the New licensee code should be used instead.
  Konami,                 // 0x34 0xA4
  Hector,                 // 0x35
  Banpresto,              // 0x39 0x9D 0xD9
  Entertainmenti,         // 0x3C
  Gremlin,                // 0x3E
  Ubisoft,                // 0x41
  Atlus,                  // 0x42 0xEB
  Malibu,                 // 0x44 0x4D
  Angel,                  // 0x46 0xCF
  SpectrumHoloby,         // 0x47
  Irem,                   // 0x49
  USGold,                 // 0x4F
  Absolute,               // 0x50
  Acclaim,                // 0x51 0xB0
  Activision,             // 0x52
  AmericanSammy,          // 0x53
  GameTek,                // 0x54
  ParkPlace,              // 0x55
  Ljn,                    // 0x56 0xDB
  Matchbox,               // 0x57
  MiltonBradley,          // 0x59
  Mindscape,              // 0x5A
  Romstar,                // 0x5B
  NaxatSoft,              // 0x5C 0xD6
  Tradewest,              // 0x5D
  Titus,                  // 0x60
  Ocean,                  // 0x67
  ElectroBrain,           // 0x6F
  Interplay,              // 0x71
  Broderbund,             // 0x72 0xAA
  SculpteredSoft,         // 0x73
  TheSalesCurve,          // 0x75
  Thq,                    // 0x78
  Accolade,               // 0x79
  TriffixEntertainment,   // 0x7A
  Microprose,             // 0x7C
  Kemco,                  // 0x7F 0xC2
  MisawaEntertainment,    // 0x80
  Lozc,                   // 0x83
  TokumaShotenIntermedia, // 0x86 0xC4
  BulletProofSoftware,    // 0x8B
  VicTokai,               // 0x8C
  Ape,                    // 0x8E
  IMax,                   // 0x8F
  Chunsoft,               // 0x91
  VideoSystem,            // 0x92
  Tsubaraya,              // 0x93
  Varie,                  // 0x95 0xE3
  YonezawaSPal,           // 0x96
  Kaneko,                 // 0x97
  Arc,                    // 0x99
  NihonBussan,            // 0x9A
  Tecmo,                  // 0x9B
  Imagineer,              // 0x9C
  Nova,                   // 0x9F
  HoriElectric,           // 0xA1
  Kawada,                 // 0xA6
  Takara,                 // 0xA7
  TechnosJapan,           // 0xA9
  ToeiAnimation,          // 0xAC
  Toho,                   // 0xAD
  Namco,                  // 0xAF
  ASCIIorNexsoft,         // 0xB1
  Enix,                   // 0xB4
  Hal,                    // 0xB6
  Snk,                    // 0xB7
  PonyCanyon,             // 0xB9
  CultureBrainO,          // 0xBA
  Sunsoft,                // 0xBB
  SonyImagesoft,          // 0xBD
  Sammy,                  // 0xBF
  Taito,                  // 0xC0 0xD0
  Squaresoft,             // 0xC3
  DataEast,               // 0xC5
  Tonkinhouse,            // 0xC6
  Koei,                   // 0xC8
  Ufl,                    // 0xC9
  Ultra,                  // 0xCA
  Vap,                    // 0xCB
  Use,                    // 0xCC
  Meldac,                 // 0xCD
  PonyCanyonor,           // 0xCE
  Sofel,                  // 0xD1
  Quest,                  // 0xD2
  SigmaEnterprises,       // 0xD3
  ASKKodansha,            // 0xD4
  CopyaSystem,            // 0xD7
  Tomy,                   // 0xDA
  Ncs,                    // 0xDD
  Human,                  // 0xDE
  Altron,                 // 0xDF
  TowaChiki,              // 0xE1
  Yutaka,                 // 0xE2
  Epcoh,                  // 0xE5
  Athena,                 // 0xE7
  Asmik,                  // 0xE8
  Natsume,                // 0xE9
  KingRecords,            // 0xEA
  EpicSonyRecords,        // 0xEC
  Igs,                    // 0xEE
  AWave,                  // 0xF0
  ExtremeEntertainment,   // 0xF3
}

impl TryFrom<u8> for OldLicensee {
  type Error = CartError;

  fn try_from(value: u8) -> Result<Self, Self::Error> {
    match value {
      0x00 => Ok(Self::None),
      0x01 | 0x31 => Ok(Self::Nintendo),
      0x08 | 0x38 => Ok(Self::Capcom),
      0x09 => Ok(Self::HotB),
      0x0A | 0xE0 => Ok(Self::Jaleco),
      0x0B => Ok(Self::Coconuts),
      0x0C | 0x6E => Ok(Self::EliteSystems),
      0x13 | 0x69 => Ok(Self::ElectronicArts),
      0x18 => Ok(Self::Hudsonsoft),
      0x19 => Ok(Self::ITCEntertainment),
      0x1A => Ok(Self::Yanoman),
      0x1D => Ok(Self::Clary),
      0x1F | 0x4A | 0x61 => Ok(Self::Virgin),
      0x20 => Ok(Self::Kss),
      0x24 => Ok(Self::PCMComplete),
      0x25 => Ok(Self::SanX),
      0x28 => Ok(Self::KotobukiSystems),
      0x29 => Ok(Self::Seta),
      0x30 | 0x70 => Ok(Self::Infogrames),
      0x32 | 0xA2 | 0xB2 => Ok(Self::Bandai),
      0x34 | 0xA4 => Ok(Self::Konami),
      0x35 => Ok(Self::Hector),
      0x39 | 0x9D | 0xD9 => Ok(Self::Banpresto),
      0x3C => Ok(Self::Entertainmenti),
      0x3E => Ok(Self::Gremlin),
      0x41 => Ok(Self::Ubisoft),
      0x42 | 0xEB => Ok(Self::Atlus),
      0x44 | 0x4D => Ok(Self::Malibu),
      0x46 | 0xCF => Ok(Self::Angel),
      0x47 => Ok(Self::SpectrumHoloby),
      0x49 => Ok(Self::Irem),
      0x4F => Ok(Self::USGold),
      0x50 => Ok(Self::Absolute),
      0x51 | 0xB0 => Ok(Self::Acclaim),
      0x52 => Ok(Self::Activision),
      0x53 => Ok(Self::AmericanSammy),
      0x54 => Ok(Self::GameTek),
      0x55 => Ok(Self::ParkPlace),
      0x56 | 0xDB => Ok(Self::Ljn),
      0x57 => Ok(Self::Matchbox),
      0x59 => Ok(Self::MiltonBradley),
      0x5A => Ok(Self::Mindscape),
      0x5B => Ok(Self::Romstar),
      0x5C | 0xD6 => Ok(Self::NaxatSoft),
      0x5D => Ok(Self::Tradewest),
      0x60 => Ok(Self::Titus),
      0x67 => Ok(Self::Ocean),
      0x6F => Ok(Self::ElectroBrain),
      0x71 => Ok(Self::Interplay),
      0x72 | 0xAA => Ok(Self::Broderbund),
      0x73 => Ok(Self::SculpteredSoft),
      0x75 => Ok(Self::TheSalesCurve),
      0x78 => Ok(Self::Thq),
      0x79 => Ok(Self::Accolade),
      0x7A => Ok(Self::TriffixEntertainment),
      0x7C => Ok(Self::Microprose),
      0x7F | 0xC2 => Ok(Self::Kemco),
      0x80 => Ok(Self::MisawaEntertainment),
      0x83 => Ok(Self::Lozc),
      0x86 | 0xC4 => Ok(Self::TokumaShotenIntermedia),
      0x8B => Ok(Self::BulletProofSoftware),
      0x8C => Ok(Self::VicTokai),
      0x8E => Ok(Self::Ape),
      0x8F => Ok(Self::IMax),
      0x91 => Ok(Self::Chunsoft),
      0x92 => Ok(Self::VideoSystem),
      0x93 => Ok(Self::Tsubaraya),
      0x95 | 0xE3 => Ok(Self::Varie),
      0x96 => Ok(Self::YonezawaSPal),
      0x97 => Ok(Self::Kaneko),
      0x99 => Ok(Self::Arc),
      0x9A => Ok(Self::NihonBussan),
      0x9B => Ok(Self::Tecmo),
      0x9C => Ok(Self::Imagineer),
      0x9F => Ok(Self::Nova),
      0xA1 => Ok(Self::HoriElectric),
      0xA6 => Ok(Self::Kawada),
      0xA7 => Ok(Self::Takara),
      0xA9 => Ok(Self::TechnosJapan),
      0xAC => Ok(Self::ToeiAnimation),
      0xAD => Ok(Self::Toho),
      0xAF => Ok(Self::Namco),
      0xB1 => Ok(Self::ASCIIorNexsoft),
      0xB4 => Ok(Self::Enix),
      0xB6 => Ok(Self::Hal),
      0xB7 => Ok(Self::Snk),
      0xB9 => Ok(Self::PonyCanyon),
      0xBA => Ok(Self::CultureBrainO),
      0xBB => Ok(Self::Sunsoft),
      0xBD => Ok(Self::SonyImagesoft),
      0xBF => Ok(Self::Sammy),
      0xC0 | 0xD0 => Ok(Self::Taito),
      0xC3 => Ok(Self::Squaresoft),
      0xC5 => Ok(Self::DataEast),
      0xC6 => Ok(Self::Tonkinhouse),
      0xC8 => Ok(Self::Koei),
      0xC9 => Ok(Self::Ufl),
      0xCA => Ok(Self::Ultra),
      0xCB => Ok(Self::Vap),
      0xCC => Ok(Self::Use),
      0xCD => Ok(Self::Meldac),
      0xCE => Ok(Self::PonyCanyonor),
      0xD1 => Ok(Self::Sofel),
      0xD2 => Ok(Self::Quest),
      0xD3 => Ok(Self::SigmaEnterprises),
      0xD4 => Ok(Self::ASKKodansha),
      0xD7 => Ok(Self::CopyaSystem),
      0xDA => Ok(Self::Tomy),
      0xDD => Ok(Self::Ncs),
      0xDE => Ok(Self::Human),
      0xDF => Ok(Self::Altron),
      0xE1 => Ok(Self::TowaChiki),
      0xE2 => Ok(Self::Yutaka),
      0xE5 => Ok(Self::Epcoh),
      0xE7 => Ok(Self::Athena),
      0xE8 => Ok(Self::Asmik),
      0xE9 => Ok(Self::Natsume),
      0xEA => Ok(Self::KingRecords),
      0xEC => Ok(Self::EpicSonyRecords),
      0xEE => Ok(Self::Igs),
      0xF0 => Ok(Self::AWave),
      0xF3 => Ok(Self::ExtremeEntertainment),
      byte => Err(CartError::Licensee(LicenseeError::Old(byte))),
    }
  }
}

impl Display for OldLicensee {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::None => write!(f, "None"),
      Self::Nintendo => write!(f, "Nintendo"),
      Self::Capcom => write!(f, "Capcom"),
      Self::HotB => write!(f, "Hot B"),
      Self::Jaleco => write!(f, "Jaleco"),
      Self::Coconuts => write!(f, "Coconuts"),
      Self::EliteSystems => write!(f, "Elite Systems"),
      Self::ElectronicArts => write!(f, "Electronic Arts"),
      Self::Hudsonsoft => write!(f, "Hudsonsoft"),
      Self::ITCEntertainment => write!(f, "ITC Entertainment"),
      Self::Yanoman => write!(f, "Yanoman"),
      Self::Clary => write!(f, "Clary"),
      Self::Virgin => write!(f, "Virgin"),
      Self::Kss => write!(f, "KSS"),
      Self::PCMComplete => write!(f, "PCM Complete"),
      Self::SanX => write!(f, "San X"),
      Self::KotobukiSystems => write!(f, "Kotobuki Systems"),
      Self::Seta => write!(f, "Seta"),
      Self::Infogrames => write!(f, "Infogrames"),
      Self::Bandai => write!(f, "Bandai"),
      Self::Konami => write!(f, "Konami"),
      Self::Hector => write!(f, "Hector"),
      Self::Banpresto => write!(f, "Banpresto"),
      Self::Entertainmenti => write!(f, "Entertainmenti"),
      Self::Gremlin => write!(f, "Gremlin"),
      Self::Ubisoft => write!(f, "Ubisoft"),
      Self::Atlus => write!(f, "Atlus"),
      Self::Malibu => write!(f, "Malibu"),
      Self::Angel => write!(f, "Angel"),
      Self::SpectrumHoloby => write!(f, "Spectrum Holoby"),
      Self::Irem => write!(f, "Irem"),
      Self::USGold => write!(f, "US Gold"),
      Self::Absolute => write!(f, "Absolute"),
      Self::Acclaim => write!(f, "Acclaim"),
      Self::Activision => write!(f, "Activision"),
      Self::AmericanSammy => write!(f, "American Sammy"),
      Self::GameTek => write!(f, "Game Tek"),
      Self::ParkPlace => write!(f, "Park Place"),
      Self::Ljn => write!(f, "LJN"),
      Self::Matchbox => write!(f, "Matchbox"),
      Self::MiltonBradley => write!(f, "Milton Bradley"),
      Self::Mindscape => write!(f, "Mindscape"),
      Self::Romstar => write!(f, "Romstar"),
      Self::NaxatSoft => write!(f, "Naxat Soft"),
      Self::Tradewest => write!(f, "Tradewest"),
      Self::Titus => write!(f, "Titus"),
      Self::Ocean => write!(f, "Ocean"),
      Self::ElectroBrain => write!(f, "Electro Brain"),
      Self::Interplay => write!(f, "Interplay"),
      Self::Broderbund => write!(f, "Broderbund"),
      Self::SculpteredSoft => write!(f, "Sculptered Soft"),
      Self::TheSalesCurve => write!(f, "The Sales Curve"),
      Self::Thq => write!(f, "THQ"),
      Self::Accolade => write!(f, "Accolade"),
      Self::TriffixEntertainment => write!(f, "Triffix Entertainment"),
      Self::Microprose => write!(f, "Microprose"),
      Self::Kemco => write!(f, "Kemco"),
      Self::MisawaEntertainment => write!(f, "Misawa Entertainment"),
      Self::Lozc => write!(f, "Lozc"),
      Self::TokumaShotenIntermedia => write!(f, "Tokuma Shoten Intermedia"),
      Self::BulletProofSoftware => write!(f, "Bullet Proof Software"),
      Self::VicTokai => write!(f, "Vic Tokai"),
      Self::Ape => write!(f, "Ape"),
      Self::IMax => write!(f, "IMax"),
      Self::Chunsoft => write!(f, "Chunsoft"),
      Self::VideoSystem => write!(f, "Video System"),
      Self::Tsubaraya => write!(f, "Tsubaraya"),
      Self::Varie => write!(f, "Varie"),
      Self::YonezawaSPal => write!(f, "Yonezawa SPal"),
      Self::Kaneko => write!(f, "Kaneko"),
      Self::Arc => write!(f, "Arc"),
      Self::NihonBussan => write!(f, "Nihon Bussan"),
      Self::Tecmo => write!(f, "Tecmo"),
      Self::Imagineer => write!(f, "Imagineer"),
      Self::Nova => write!(f, "Nova"),
      Self::HoriElectric => write!(f, "Hori Electric"),
      Self::Kawada => write!(f, "Kawada"),
      Self::Takara => write!(f, "Takara"),
      Self::TechnosJapan => write!(f, "Technos Japan"),
      Self::ToeiAnimation => write!(f, "Toei Animation"),
      Self::Toho => write!(f, "Toho"),
      Self::Namco => write!(f, "Namco"),
      Self::ASCIIorNexsoft => write!(f, "ASCII or Nexsoft"),
      Self::Enix => write!(f, "Enix"),
      Self::Hal => write!(f, "HAL"),
      Self::Snk => write!(f, "SNK"),
      Self::PonyCanyon => write!(f, "Pony Canyon"),
      Self::CultureBrainO => write!(f, "Culture Brain O"),
      Self::Sunsoft => write!(f, "Sunsoft"),
      Self::SonyImagesoft => write!(f, "Sony Imagesoft"),
      Self::Sammy => write!(f, "Sammy"),
      Self::Taito => write!(f, "Taito"),
      Self::Squaresoft => write!(f, "Squaresoft"),
      Self::DataEast => write!(f, "Data East"),
      Self::Tonkinhouse => write!(f, "Tonkinhouse"),
      Self::Koei => write!(f, "Koei"),
      Self::Ufl => write!(f, "UFL"),
      Self::Ultra => write!(f, "Ultra"),
      Self::Vap => write!(f, "Vap"),
      Self::Use => write!(f, "Use"),
      Self::Meldac => write!(f, "Meldac"),
      Self::PonyCanyonor => write!(f, "Pony Canyon or"),
      Self::Sofel => write!(f, "Sofel"),
      Self::Quest => write!(f, "Quest"),
      Self::SigmaEnterprises => write!(f, "Sigma Enterprises"),
      Self::ASKKodansha => write!(f, "ASK Kodansha"),
      Self::CopyaSystem => write!(f, "Copya System"),
      Self::Tomy => write!(f, "Tomy"),
      Self::Ncs => write!(f, "NCS"),
      Self::Human => write!(f, "Human"),
      Self::Altron => write!(f, "Altron"),
      Self::TowaChiki => write!(f, "Towa Chiki"),
      Self::Yutaka => write!(f, "Yutaka"),
      Self::Epcoh => write!(f, "Epcoh"),
      Self::Athena => write!(f, "Athena"),
      Self::Asmik => write!(f, "Asmik"),
      Self::Natsume => write!(f, "Natsume"),
      Self::KingRecords => write!(f, "King Records"),
      Self::EpicSonyRecords => write!(f, "Epic Sony Records"),
      Self::Igs => write!(f, "IGS"),
      Self::AWave => write!(f, "A Wave"),
      Self::ExtremeEntertainment => write!(f, "Extreme Entertainment"),
    }
  }
}
