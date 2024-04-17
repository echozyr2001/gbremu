use super::register::{Register16, Register8};

pub enum InstType {
  ALI8(Arithmetic8),
  LI(Logic),
  ALI16(Arithmetic16),
  BOI(BitOperations),
  BSI(BitShift),
  LDI(Load),
  JSI(JumpsAndSubroutines),
  SOI(StackOperations),
}

pub enum Arithmetic8 {
  /// Add the value in r8 plus the carry flag to A.
  /// Bytes: 1
  /// Cycles: 1
  /// Flags: Z, 0, H, C
  AdcAR8(Register8),
  /// Add the byte pointed to by HL plus the carry flag to A.
  /// Bytes: 1
  /// Cycles: 2
  /// Flags: Z, 0, H, C
  AdcAHl,
  /// Add the value u8 plus the carry flag to A.
  /// Bytes: 2
  /// Cycles: 2
  /// Flags: Z, 0, H, C
  AdcAU8(u8),
  /// Add the value in r8 to A.
  /// Bytes: 1
  /// Cycles: 1
  /// Flags: Z, 0, H, C
  AddAR8(Register8),
  /// Add the byte pointed to by HL to A.
  /// Bytes: 1
  /// Cycles: 2
  /// Flags: Z, 0, H, C
  AddAHl,
  /// Add the value u8 to A.
  /// Bytes: 2
  /// Cycles: 2
  /// Flags: Z, 0, H, C
  AddAU8(u8),
  /// Subtract the value in r8 from A and set flags accordingly, but don't store the result.
  /// This is useful for ComParing values.
  /// Bytes: 1
  /// Cycles: 1
  /// Flags: Z, 1, H, C
  CpAR8(Register8),
  /// Subtract the byte pointed to by HL from A and set flags accordingly, but don't store the result.
  /// Bytes: 1
  /// Cycles: 2
  /// Flags: Z, 1, H, C
  CpAHl,
  /// Subtract the value u8 from A and set flags accordingly, but don't store the result.
  /// Bytes: 2
  /// Cycles: 2
  /// Flags: Z, 1, H, C
  CpAU8(u8),
  /// Decrement value in register r8 by 1.
  /// Bytes: 1
  /// Cycles: 1
  /// Flags: Z, 1, H, -
  DecR8(Register8),
  /// Decrement the byte pointed to by HL by 1.
  /// Bytes: 1
  /// Cycles: 3
  /// Flags: Z, 1, H, -
  DecHl,
  /// Increment value in register r8 by 1.
  /// Bytes: 1
  /// Cycles: 1
  /// Flags: Z, 0, H, -
  IncR8(Register8),
  /// Increment the byte pointed to by HL by 1.
  /// Bytes: 1
  /// Cycles: 3
  /// Flags: Z, 0, H, -
  IncHl,
  SbcAR8(Register8),
  SbcAHl,
  SbcAU8(u8),
  SubAR8(Register8),
  SubAHl,
  SubAU8(u8),
}

pub enum Logic {
  /// Bitwise AND between the value in r8 and A.
  /// Bytes: 1
  /// Cycles: 1
  /// Flags: Z, 0, 1, 0
  AndAR8(Register8),
  /// Bitwise AND between the byte pointed to by HL and A.
  /// Bytes: 1
  /// Cycles: 2
  /// Flags: Z, 0, 1, 0
  AndAHl,
  /// Bitwise AND between the value u8 and A.
  /// Bytes: 2
  /// Cycles: 2
  /// Flags: Z, 0, 1, 0
  AndAU8(u8),
  OrAR8(Register8),
  OrAHl,
  OrAU8(u8),
  XorAR8(Register8),
  XorAHl,
  XorAU8(u8),
}

pub enum Arithmetic16 {
  /// Add the value in r16 to HL.
  /// Bytes: 1
  /// Cycles: 2
  /// Flags: -, 0, H, C
  AddHlR16(Register16),
  /// Decrement value in register r16 by 1.
  /// Bytes: 1
  /// Cycles: 2
  /// Flags: -, -, -, -
  DecR16(Register16),
  /// Increment value in register r16 by 1.
  /// Bytes: 1
  /// Cycles: 2
  /// Flags: -, -, -, -
  IncR16(Register16),
}

pub enum BitOperations {
  /// Test bit u3 in register r8, set the zero flag if bit not set.
  /// Bytes: 2
  /// Cycles: 2
  /// Flags: Z, 0, 1, -
  BitU3R8(u8, Register8),
  /// Test bit u3 in the byte pointed to by HL, set the zero flag if bit not set.
  /// Bytes: 2
  /// Cycles: 3
  /// Flags: Z, 0, 1, -
  BitU3Hl,
  ResU3R8(u8, Register8),
  ResU3Hl,
  SetU3R8(u8, Register8),
  SetU3Hl,
  SwapR8(Register8),
  SwapHl,
}

pub enum BitShift {
  RlR8(Register8),
  RlHl,
  RlA,
  RlcR8(Register8),
  RlcHl,
  RlcA,
  RrR8(Register8),
  RrHl,
  RrA,
  RrcR8(Register8),
  RrcHl,
  RrcA,
  SlaR8(Register8),
  SlaHl,
  SraR8(Register8),
  SraHl,
  SrlR8(Register8),
  SrlHl,
}

pub enum Load {
  /// Load (copy) value in register on the right into register on the left.
  /// Bytes: 1
  /// Cycles: 1
  /// Flags: -, -, -, -
  LdR8R8(Register8, Register8),
  /// Load value u8 into register r8.
  /// Bytes: 2
  /// Cycles: 2
  /// Flags: -, -, -, -
  LdR8U8(Register8, u8),
  /// Load value u16 into register r16.
  /// Bytes: 3
  /// Cycles: 3
  /// Flags: -, -, -, -
  LdR16U16(Register16, u16),
  /// Store value in register r8 into the byte pointed to by register HL.
  /// Bytes: 1
  /// Cycles: 2
  /// Flags: -, -, -, -
  LdHlR8(Register8),
  /// Store value u8 into the byte pointed to by register HL.
  /// Bytes: 2
  /// Cycles: 3
  /// Flags: -, -, -, -
  LdHlU8(u8),
  /// Load value into register r8 from the byte pointed to by register HL.
  /// Bytes: 1
  /// Cycles: 2
  /// Flags: -, -, -, -
  LdR8Hl(Register8),
  /// Store value in register A into the byte pointed to by register r16.
  /// Bytes: 1
  /// Cycles: 2
  /// Flags: -, -, -, -
  LdR16A(Register16),
  /// Store value in register A into the byte at address n16.
  /// Bytes: 3
  /// Cycles: 4
  /// Flags: -, -, -, -
  LdU16A(u16),
  /// Store value in register A into the byte at address n16, provided the address is between $FF00 and $FFFF.
  /// Bytes: 2
  /// Cycles: 3
  /// Flags: -, -, -, -
  LdhU16A(u16),
  /// Store value in register A into the byte at address $FF00+C.
  /// Bytes: 1
  /// Cycles: 2
  /// Flags: -, -, -, -
  LdhCA,
  /// Load value in register A from the byte pointed to by register r16.
  /// Bytes: 1
  /// Cycles: 2
  /// Flags: -, -, -, -
  LdAR16(Register16),
  /// Load value in register A from the byte at address n16.
  /// Bytes: 3
  /// Cycles: 4
  /// Flags: -, -, -, -
  LdAU16(Register16),
  /// Load value in register A from the byte at address n16, provided the address is between $FF00 and $FFFF.
  /// Bytes: 2
  /// Cycles: 3
  /// Flags: -, -, -, -
  LdhAU16(Register16),
  /// Load value in register A from the byte at address $FF00+c.
  /// Bytes: 1
  /// Cycles: 2
  /// Flags: -, -, -, -
  LdhAC,
  /// Store value in register A into the byte pointed by HL and increment HL afterwards.
  /// Bytes: 1
  /// Cycles: 2
  /// Flags: -, -, -, -
  LdHliA,
  /// Store value in register A into the byte pointed by HL and decrement HL afterwards.
  /// Bytes: 1
  /// Cycles: 2
  /// Flags: -, -, -, -
  LdHldA,
  /// Load value into register A from the byte pointed by HL and increment HL afterwards.
  /// Bytes: 1
  /// Cycles: 2
  /// Flags: -, -, -, -
  LdAHli,
  /// Load value into register A from the byte pointed by HL and decrement HL afterwards.
  /// Bytes: 1
  /// Cycles: 2
  /// Flags: -, -, -, -
  LdAHld,
}

pub enum JumpsAndSubroutines {
  /// Call address n16. This pushes the address of the instruction after the CALL on the stack,
  /// such that RET can pop it later; then, it executes an implicit JP n16.
  /// Bytes: 3
  /// Cycles: 6
  /// Flags: -, -, -, -
  CallU16(u16),
  // TODO: set CC
  /// Call address n16 if condition cc is met.
  /// Bytes: 3
  /// Cycles: 6 taken / 3 untaken
  CallCcU16(u16),
  /// Jump to address in HL; effectively, load PC with value in register HL.
  /// Bytes: 1
  /// Cycles: 1
  /// Flags: -, -, -, -
  JpHl,
  /// Jump to address n16; effectively, store n16 into PC.
  /// Bytes: 3
  /// Cycles: 4
  /// Flags: -, -, -, -
  JpU16(u16),
  // TODO: set CC
  /// Jump to address n16 if condition cc is met.
  /// Bytes: 3
  /// Cycles: 4 taken / 3 untaken
  /// Flags: -, -, -, -
  JpCcU16(u16),
  // TODO: describe this
  /// Relative Jump to address n16.
  /// Bytes: 2
  /// Cycles: 3
  /// Flags: -, -, -, -
  JrU16(u16),
  // TODO: set CC
  /// Relative Jump to address n16 if condition cc is met.
  /// Bytes: 2
  /// Cycles: 3 taken / 2 untaken
  /// Flags: -, -, -, -
  JrCcU16(u16),
  // TODO: set CC
  RetCc,
  Ret,
  Reti,
  // TODO: set rstvec
  Rst(u16),
}

pub enum StackOperations {
  /// Add the value in Sp to Hl.
  /// Bytes: 1
  /// Cycles: 2
  /// Flags: -, 0, H, C
  AddHlSp,
  /// Add the signed value e8 to SP.
  /// Bytes: 2
  /// Cycles: 4
  /// Flags: 0, 0, H, C
  ADDSpI8(i8),
  /// Decrement value in register SP by 1.
  /// Bytes: 1
  /// Cycles: 2
  /// Flags: -, -, -, -
  DecSp,
  /// Increment value in register SP by 1.
  /// Bytes: 1
  /// Cycles: 2
  /// Flags: -, -, -, -
  IncSp,
  /// Load value n16 into register SP.
  /// Bytes: 3
  /// Cycles: 3
  /// Flags: -, -, -, -
  LdSpU16(u16),
  /// Store SP & $FF at address n16 and SP >> 8 at address n16 + 1.
  /// Bytes: 3
  /// Cycles: 5
  /// Flags: -, -, -, -
  LdU16Sp(u16),
  /// Add the signed value e8 to SP and store the result in HL.
  /// Bytes: 2
  /// Cycles: 3
  /// Flags: 0, 0, H, C
  LdHlSpI8(i8),
  /// Load register HL into register SP.
  /// Bytes: 1
  /// Cycles: 2
  /// Flags: -, -, -, -
  LdSpHl,
  PopAf,
  PopR16(Register16),
  PushAf,
  PushR16(Register16),
}
