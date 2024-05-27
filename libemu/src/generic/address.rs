pub trait Address {
  fn read(&self, addr: u16) -> u8;
  fn write(&mut self, addr: u16, value: u8);
}

pub trait TryAddress {
  type Error;

  fn try_read(&self, addr: u16) -> Result<u8, Self::Error>;
  fn try_write(&mut self, addr: u16, value: u8) -> Result<(), Self::Error>;
}
