use self::register::Register;

pub mod register;

pub struct CPU {
  regs: Register,
  halted: bool,
  /// Interrupt Master Enable
  ime: bool,
}

impl CPU {
  pub fn step() {
    todo!()
  }
}
