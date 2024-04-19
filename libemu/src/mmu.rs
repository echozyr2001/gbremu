pub struct MMU {
  /// 0x0000 - 0x7FFF: Cartridge ROM
  _rom: Vec<u8>,
  /// 0x8000 - 0x9FFF: Video RAM
  _vram: Vec<u8>,
  /// 0xA000 - 0xBFFF: External RAM
  _eram: Vec<u8>,
  /// 0xC000 - 0xDFFF: Work RAM
  _wram: Vec<u8>,
  /// 0xFE00 - 0xFE9F: Sprite attribute table (OAM)
  _oam: Vec<u8>,
  /// 0xFF00 - 0xFF7F: I/O Registers
  _io: Vec<u8>,
  /// 0xFF80 - 0xFFFE: High RAM (HRAM)
  _hram: Vec<u8>,
  /// 0xFFFF: Interrupt Enable Register
  _ier: u8,
}

impl MMU {
  pub fn new() -> Self {
    Self {
      _rom: Vec::new(),
      _vram: vec![0; 0x2000],
      _eram: vec![0; 0x2000],
      _wram: vec![0; 0x2000],
      _oam: vec![0; 0xA0],
      _io: vec![0; 0x80],
      _hram: vec![0; 0x7F],
      _ier: 0,
    }
  }

  pub fn read(&self, _addr: u16) -> u8 {
    todo!()
  }

  pub fn write(&self, _addr: u16, _val: u8) {
    todo!()
  }
}
