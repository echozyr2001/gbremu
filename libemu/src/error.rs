use std::{array::TryFromSliceError, str::Utf8Error};
use thiserror::Error;

use crate::generic::arch::value::Value;

/// A type specifying categories of ['Header'] error.
#[derive(Debug, Error)]
pub enum CartError {
  #[error("cartridge size mismatch")]
  // Mismatch(Box<[u8]>),
  Mismatch(Box<[u8]>),
  #[error("Missing header")]
  Missing,
  #[error("Invalid Logo")]
  Logo,
  #[error(transparent)]
  Slice(#[from] TryFromSliceError),
  #[error(transparent)]
  Title(#[from] Utf8Error),
  #[error("Invalid licensee code: {0}")]
  Licensee(#[from] LicenseeError),
  #[error("Invalid CGB flag: {0:#04x}")]
  Color(u8),
  #[error("Invalid cartridge type: {0:#04x}")]
  Kind(u8),
  #[error("Invalid ROM size")]
  Rom(u8),
  #[error("Invalid RAM size")]
  Ram(u8),
  #[error("Invalid destination code: {0:#04x}")]
  Region(u8),
  #[error("Invalid header checksum: found {found}, expected {expected}")]
  HeaderCheck { found: u8, expected: u8 },
  #[error("Invalid global checksum: found {found}, expected {expected}")]
  GlobalCheck { found: u16, expected: u16 },
}

#[derive(Debug, Error)]
pub enum LicenseeError {
  #[error("Old licensee code: {0:#04x}")]
  Old(u8),
  #[error("New licensee code: {0:#06x}")]
  New(u16),
}

#[derive(Debug, Error)]
pub enum AddressError {}

#[derive(Debug, Error)]
pub enum MemoryError<Idx: Value> {
  #[error("Index out of bounds: {0:?}")]
  Bounds(Idx),
  #[error("Cannot write to read-only memory")]
  Write,
}

#[derive(Debug, Error)]
pub enum BusError<Idx: Value> {
  #[error("Unmapped address: {0:?}")]
  Unmapped(Idx),
}
