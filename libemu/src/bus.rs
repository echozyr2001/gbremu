/// 0x4000 - 0x7FFF: 16KB ROM bank 01..NN { From cartridge, switchable bank via MBC (if any) }
/// 0x8000 - 0x9FFF: 8KB Video RAM (VRAM) { In CGB mode, switchable bank 0/1 }
/// 0xA000 - 0xBFFF: 8KB External RAM { From cartridge, switchable bank if any }
/// 0xC000 - 0xCFFF: 4KB Work RAM bank 0
/// 0xD000 - 0xDFFF: 4KB Work RAM bank 1 - 7 { In CGB mode, switchable bank 1-7 }
/// 0xE000 - 0xFDFF: Mirror of C000~DDFF (ECHO RAM) { Typically not used }
/// 0xFE00 - 0xFE9F: Sprite attribute table (OAM)
/// 0xFEA0 - 0xFEFF: Not usable { Do not use }
/// 0xFF00 - 0xFF7F: I/O Registers
/// 0xFF80 - 0xFFFE: High RAM (HRAM)
/// 0xFFFF: Interrupt Enable Register

/// 0xFF00 - Joypad input
/// 0xFF01 - 0xFF02 - Serial transfer
/// 0xFF04 - 0xFF07 - Timer and divider
/// 0xFF10 - 0xFF26 - Audio
/// 0xFF30 - 0xFF3F - Wave Pattern
/// 0xFF40 - 0xFF4B - LCD Control, Status, Position, Scrolling, and Palette
/// 0xFF4F - VRAM Bank Select
/// 0xFF50 - Set to non-zero to disable boot ROM
/// 0xFF51 - 0xFF55 - VRAM DMA
/// 0xFF68 - 0xFF6B - BG / OBJ Palettes
/// 0xFF70 - WRAM Bank Select

pub struct Bus {
  /// 0x0000 - 0x7FFF: ROM
  rom: [u8; 0x8000],
  /// 0x8000 - 0x9FFF: Video RAM
  _vram: [u8; 0x2000],
  /// 0xA000 - 0xBFFF: External RAM
  _eram: [u8; 0x2000],
  /// 0xC000 - 0xDFFF: Work RAM
  _wram: [u8; 0x2000],
  /// 0xE000 - 0xFDFF: Echo RAM
  /// 0xFE00 - 0xFE9F: Sprite attribute table (OAM)
  _oam: [u8; 0x00A0],
  /// 0xFEA0 - 0xFEFF: Not Usable
  /// 0xFF00 - 0xFF7F: I/O Registers
  _io: [u8; 0x0080],
  ///   0xFF00: Joypad input
  ///   0xFF01 - 0xFF02: Serial transfer
  ///   0xFF04 - 0xFF07: Timer and divider
  ///   0xFF0F: Interrupt Flag
  ///   0xFF10 - 0xFF3F: Sound Controller
  ///   0xFF40 - 0xFF45 | 0xFF47 - 0xFF4B: LCD Control, Status, Position, Scrolling, and Palette
  ///   0xFF4D: KEY1 - CGB Mode Only - Prepare Speed Switch
  ///   0xFF4F: LCD VRAM Bank (CGB only)
  ///   0xFF51 - 0xFF55: LCD VRAM DMA Transfer (CGB only)
  ///   0xFF68 - 0xFF6B: LCD Color Palettes (CGB only)
  ///   0xFF70: SVBK - CGB Mode Only - WRAM Bank
  /// 0xFF80 - 0xFFFE: High RAM (HRAM)
  _hram: [u8; 0x007F],
  /// 0xFFFF: Interrupt Enable Register
  _ier: u8,
}

impl Bus {
  pub fn new() -> Self {
    Self {
      rom: [0; 0x8000],
      _vram: [0; 0x2000],
      _eram: [0; 0x2000],
      _wram: [0; 0x2000],
      _oam: [0; 0x00A0],
      _io: [0; 0x0080],
      _hram: [0; 0x007F],
      _ier: 0,
    }
  }

  pub fn read(&self, addr: u16) -> u8 {
    self.rom[addr as usize]
  }

  pub fn write(&self, _addr: u16, _val: u8) {
    todo!()
  }

  pub fn load_boot(&mut self, addr: u16, boot: &[u8]) {
    self.rom[addr as usize..addr as usize + boot.len()].copy_from_slice(boot);
  }
}
