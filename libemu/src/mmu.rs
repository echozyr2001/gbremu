/// 0x0000 - 0x3FFF: 16KB ROM bank 00 { From cartridge, usually a fixed bank }
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
  pub fn read(&self, _addr: u16) -> u8 {
    todo!()
  }
}
