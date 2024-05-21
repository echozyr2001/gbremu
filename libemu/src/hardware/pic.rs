use log::trace;

#[derive(Debug)]
pub struct Pic {
  IE: u8,
  IF: u8,
}

impl Pic {
  pub fn int(&self) -> Option<Interrupt> {
    let int = (self.IE & self.IF).try_into().ok();
    if let Some(int) = int {
      trace!("int: {:?}", int);
    }

    int
  }

  pub fn req(&mut self, int: Interrupt) {
    self.IF |= int as u8;
    trace!("request int: {:?}", int);
  }

  pub fn ack(&mut self, int: Interrupt) {
    self.IF &= !(int as u8);
    trace!("acknowledge int: {:?}", int);
  }
}

#[derive(Debug, Clone, Copy)]
pub enum Interrupt {
  VBlank,
  LcdStat,
  Timer,
  Serial,
  Joypad,
}

impl TryFrom<u8> for Interrupt {
  type Error = ();

  fn try_from(value: u8) -> Result<Self, Self::Error> {
    match value.trailing_zeros() {
      0 => Ok(Interrupt::VBlank),
      1 => Ok(Interrupt::LcdStat),
      2 => Ok(Interrupt::Timer),
      3 => Ok(Interrupt::Serial),
      4 => Ok(Interrupt::Joypad),
      _ => Err(()),
    }
  }
}
