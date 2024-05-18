#![allow(dead_code)]

use crate::hardware::soc::cpu::Cpu;

pub fn illegal(_cpu: &mut Cpu) {
  panic!("Illegal instruction");
}

pub fn nop(_cpu: &mut Cpu) {}

pub fn ld_bc_u16(cpu: &mut Cpu) {
  let byte1 = cpu.fetch_byte();
  let byte2 = cpu.fetch_byte();
  let value = u16::from_le_bytes([byte1, byte2]);
  cpu.regs.set_bc(value);
}

pub fn ld_mbc_a(cpu: &mut Cpu) {
  cpu.write(cpu.regs.get_bc(), cpu.regs.get_a());
}

pub fn inc_bc(cpu: &mut Cpu) {
  cpu.regs.set_bc(cpu.regs.get_bc().wrapping_add(1));
}

pub fn inc_b(cpu: &mut Cpu) {
  let b = cpu.regs.get_b().wrapping_add(1);
  cpu
    .regs
    .set_flags_znhc(b == 0, false, (b & 0x0F) == 0, false);
  cpu.regs.set_b(b);
}

pub fn dec_b(cpu: &mut Cpu) {
  let b = cpu.regs.get_b().wrapping_sub(1);
  cpu
    .regs
    .set_flags_znhc(b == 0, true, (b & 0x0F) == 0x0F, false);
  cpu.regs.set_b(b);
}

pub fn ld_b_u8(cpu: &mut Cpu) {
  let byte = cpu.fetch_byte();
  cpu.regs.set_b(byte);
}

pub fn rlca(cpu: &mut Cpu) {
  let a = cpu.regs.get_a();
  let carry = a & 0x80 != 0;
  let a = a.rotate_left(1);
  cpu.regs.set_a(a);
  cpu.regs.set_flags_znhc(false, false, false, carry);
}

pub fn ld_mu16_sp(cpu: &mut Cpu) {
  let byte1 = cpu.fetch_byte();
  let byte2 = cpu.fetch_byte();
  let value = u16::from_le_bytes([byte1, byte2]);
  cpu.write(value, cpu.regs.get_sp() as u8);
}

pub fn add_hl_bc(cpu: &mut Cpu) {
  let hl = cpu.regs.get_hl();
  let bc = cpu.regs.get_bc();
  let result = hl.wrapping_add(bc);
  let half_carry = (hl & 0x0FFF) + (bc & 0x0FFF) > 0x0FFF;
  let carry = hl > 0xFFFF - bc;
  cpu.regs.set_flags_znhc(false, false, half_carry, carry);
  cpu.regs.set_hl(result);
}

pub fn ld_a_mbc(cpu: &mut Cpu) {
  let bc = cpu.regs.get_bc();
  let value = cpu.read(bc);
  cpu.regs.set_a(value);
}

pub fn dec_bc(cpu: &mut Cpu) {
  cpu.regs.set_bc(cpu.regs.get_bc().wrapping_sub(1));
}

pub fn inc_c(cpu: &mut Cpu) {
  let c = cpu.regs.get_c().wrapping_add(1);
  cpu
    .regs
    .set_flags_znhc(c == 0, false, (c & 0x0F) == 0, false);
  cpu.regs.set_c(c);
}

pub fn dec_c(cpu: &mut Cpu) {
  let c = cpu.regs.get_c().wrapping_sub(1);
  cpu
    .regs
    .set_flags_znhc(c == 0, true, (c & 0x0F) == 0x0F, false);
  cpu.regs.set_c(c);
}

pub fn ld_c_u8(cpu: &mut Cpu) {
  let byte = cpu.fetch_byte();
  cpu.regs.set_c(byte);
}

pub fn rrca(cpu: &mut Cpu) {
  let a = cpu.regs.get_a();
  let carry = a & 0x01 != 0;
  let a = a.rotate_right(1);
  cpu.regs.set_a(a);
  cpu.regs.set_flags_znhc(false, false, false, carry);
}

// TODO: todo this
pub fn stop_u8(cpu: &mut Cpu) {
  todo!()
}

pub fn ld_de_u16(cpu: &mut Cpu) {
  let byte1 = cpu.fetch_byte();
  let byte2 = cpu.fetch_byte();
  let value = u16::from_le_bytes([byte1, byte2]);
  cpu.regs.set_de(value);
}

pub fn ld_mde_a(cpu: &mut Cpu) {
  cpu.write(cpu.regs.get_de(), cpu.regs.get_a());
}

pub fn inc_de(cpu: &mut Cpu) {
  cpu.regs.set_de(cpu.regs.get_de().wrapping_add(1));
}

pub fn inc_d(cpu: &mut Cpu) {
  let d = cpu.regs.get_d().wrapping_add(1);
  cpu
    .regs
    .set_flags_znhc(d == 0, false, (d & 0x0F) == 0, false);
  cpu.regs.set_d(d);
}

pub fn dec_d(cpu: &mut Cpu) {
  let d = cpu.regs.get_d().wrapping_sub(1);
  cpu
    .regs
    .set_flags_znhc(d == 0, true, (d & 0x0F) == 0x0F, false);
  cpu.regs.set_d(d);
}

pub fn ld_d_u8(cpu: &mut Cpu) {
  let byte = cpu.fetch_byte();
  cpu.regs.set_d(byte);
}

// TODO: check this
pub fn rla(cpu: &mut Cpu) {
  let a = cpu.regs.get_a();
  let carry = a & 0x80 != 0;
  let mut a = a.rotate_left(1);
  a |= if cpu.regs.get_flags_c() { 1 } else { 0 };
  cpu.regs.set_a(a);
  cpu.regs.set_flags_znhc(false, false, false, carry);
}

pub fn jr_i8(cpu: &mut Cpu) {
  let byte = cpu.fetch_byte() as i8;
  let pc = cpu.regs.get_pc() as i16;
  let new_pc = pc.wrapping_add(byte as i16);
  cpu.regs.set_pc(new_pc as u16);
}

pub fn add_hl_de(cpu: &mut Cpu) {
  let hl = cpu.regs.get_hl();
  let de = cpu.regs.get_de();
  let result = hl.wrapping_add(de);
  let half_carry = (hl & 0x0FFF) + (de & 0x0FFF) > 0x0FFF;
  let carry = hl > 0xFFFF - de;
  cpu.regs.set_flags_znhc(false, false, half_carry, carry);
  cpu.regs.set_hl(result);
}

pub fn ld_a_mde(cpu: &mut Cpu) {
  let de = cpu.regs.get_de();
  let value = cpu.read(de);
  cpu.regs.set_a(value);
}

pub fn dec_de(cpu: &mut Cpu) {
  cpu.regs.set_de(cpu.regs.get_de().wrapping_sub(1));
}

pub fn inc_e(cpu: &mut Cpu) {
  let e = cpu.regs.get_e().wrapping_add(1);
  cpu
    .regs
    .set_flags_znhc(e == 0, false, (e & 0x0F) == 0, false);
  cpu.regs.set_e(e);
}

pub fn dec_e(cpu: &mut Cpu) {
  let e = cpu.regs.get_e().wrapping_sub(1);
  cpu
    .regs
    .set_flags_znhc(e == 0, true, (e & 0x0F) == 0x0F, false);
  cpu.regs.set_e(e);
}

pub fn ld_e_u8(cpu: &mut Cpu) {
  let byte = cpu.fetch_byte();
  cpu.regs.set_e(byte);
}

pub fn rra(cpu: &mut Cpu) {
  let a = cpu.regs.get_a();
  let carry = a & 0x01 != 0;
  let mut a = a.rotate_right(1);
  a |= if cpu.regs.get_flags_c() { 0x80 } else { 0 };
  cpu.regs.set_a(a);
  cpu.regs.set_flags_znhc(false, false, false, carry);
}

pub fn jr_nz_i8(cpu: &mut Cpu) {
  let byte = cpu.fetch_byte() as i8;
  if !cpu.regs.get_flags_z() {
    let pc = cpu.regs.get_pc() as i16;
    let new_pc = pc.wrapping_add(byte as i16);
    cpu.regs.set_pc(new_pc as u16);
  }
}

pub fn ld_hl_u16(cpu: &mut Cpu) {
  let byte1 = cpu.fetch_byte();
  let byte2 = cpu.fetch_byte();
  let value = u16::from_le_bytes([byte1, byte2]);
  cpu.regs.set_hl(value);
}

pub fn ld_mhli_a(cpu: &mut Cpu) {
  let hl = cpu.regs.get_hl();
  cpu.write(hl, cpu.regs.get_a());
  cpu.regs.set_hl(hl.wrapping_add(1));
}

pub fn inc_hl(cpu: &mut Cpu) {
  cpu.regs.set_hl(cpu.regs.get_hl().wrapping_add(1));
}

pub fn inc_h(cpu: &mut Cpu) {
  let h = cpu.regs.get_h().wrapping_add(1);
  cpu
    .regs
    .set_flags_znhc(h == 0, false, (h & 0x0F) == 0, false);
  cpu.regs.set_h(h);
}

pub fn dec_h(cpu: &mut Cpu) {
  let h = cpu.regs.get_h().wrapping_sub(1);
  cpu
    .regs
    .set_flags_znhc(h == 0, true, (h & 0x0F) == 0x0F, false);
  cpu.regs.set_h(h);
}

pub fn ld_h_u8(cpu: &mut Cpu) {
  let byte = cpu.fetch_byte();
  cpu.regs.set_h(byte);
}

pub fn daa(cpu: &mut Cpu) {
  todo!()
}

pub fn jr_z_i8(cpu: &mut Cpu) {
  let byte = cpu.fetch_byte() as i8;
  if cpu.regs.get_flags_z() {
    let pc = cpu.regs.get_pc() as i16;
    let new_pc = pc.wrapping_add(byte as i16);
    cpu.regs.set_pc(new_pc as u16);
  }
}

pub fn add_hl_hl(cpu: &mut Cpu) {
  let hl = cpu.regs.get_hl();
  let result = hl.wrapping_add(hl);
  let half_carry = (hl & 0x0FFF) + (hl & 0x0FFF) > 0x0FFF;
  let carry = hl > 0xFFFF - hl;
  cpu.regs.set_flags_znhc(false, false, half_carry, carry);
  cpu.regs.set_hl(result);
}

pub fn ld_a_mhli(cpu: &mut Cpu) {
  let hl = cpu.regs.get_hl();
  let value = cpu.read(hl);
  cpu.regs.set_a(value);
  cpu.regs.set_hl(hl.wrapping_add(1));
}

pub fn dec_hl(cpu: &mut Cpu) {
  cpu.regs.set_hl(cpu.regs.get_hl().wrapping_sub(1));
}

pub fn inc_l(cpu: &mut Cpu) {
  let l = cpu.regs.get_l().wrapping_add(1);
  cpu
    .regs
    .set_flags_znhc(l == 0, false, (l & 0x0F) == 0, false);
  cpu.regs.set_l(l);
}

pub fn dec_l(cpu: &mut Cpu) {
  let l = cpu.regs.get_l().wrapping_sub(1);
  cpu
    .regs
    .set_flags_znhc(l == 0, true, (l & 0x0F) == 0x0F, false);
  cpu.regs.set_l(l);
}

pub fn ld_l_u8(cpu: &mut Cpu) {
  let byte = cpu.fetch_byte();
  cpu.regs.set_l(byte);
}

pub fn cpl(cpu: &mut Cpu) {
  let a = cpu.regs.get_a();
  let a = !a;
  cpu.regs.set_a(a);
  cpu
    .regs
    .set_flags_znhc(false, true, true, cpu.regs.get_flags_c());
}

pub fn jr_nc_i8(cpu: &mut Cpu) {
  let byte = cpu.fetch_byte() as i8;
  if !cpu.regs.get_flags_c() {
    let pc = cpu.regs.get_pc() as i16;
    let new_pc = pc.wrapping_add(byte as i16);
    cpu.regs.set_pc(new_pc as u16);
  }
}

pub fn ld_sp_u16(cpu: &mut Cpu) {
  let byte1 = cpu.fetch_byte();
  let byte2 = cpu.fetch_byte();
  let value = u16::from_le_bytes([byte1, byte2]);
  cpu.regs.set_sp(value);
}

pub fn ld_mhld_a(cpu: &mut Cpu) {
  let hl = cpu.regs.get_hl();
  cpu.write(hl, cpu.regs.get_a());
  cpu.regs.set_hl(hl.wrapping_sub(1));
}

pub fn inc_sp(cpu: &mut Cpu) {
  cpu.regs.set_sp(cpu.regs.get_sp().wrapping_add(1));
}

pub fn inc_mhl(cpu: &mut Cpu) {
  let hl = cpu.regs.get_hl();
  let value = cpu.read(hl).wrapping_add(1);
  cpu.write(hl, value);
  cpu
    .regs
    .set_flags_znhc(value == 0, false, (value & 0x0F) == 0, false);
}

pub fn dec_mhl(cpu: &mut Cpu) {
  let hl = cpu.regs.get_hl();
  let value = cpu.read(hl).wrapping_sub(1);
  cpu.write(hl, value);
  cpu
    .regs
    .set_flags_znhc(value == 0, true, (value & 0x0F) == 0x0F, false);
}

pub fn ld_mhl_u8(cpu: &mut Cpu) {
  let byte = cpu.fetch_byte();
  let hl = cpu.regs.get_hl();
  cpu.write(hl, byte);
}

pub fn scf(cpu: &mut Cpu) {
  cpu.regs.set_flags_znhc(false, false, false, true);
}

pub fn jr_c_i8(cpu: &mut Cpu) {
  let byte = cpu.fetch_byte() as i8;
  if cpu.regs.get_flags_c() {
    let pc = cpu.regs.get_pc() as i16;
    let new_pc = pc.wrapping_add(byte as i16);
    cpu.regs.set_pc(new_pc as u16);
  }
}

pub fn add_hl_sp(cpu: &mut Cpu) {
  let hl = cpu.regs.get_hl();
  let sp = cpu.regs.get_sp();
  let result = hl.wrapping_add(sp);
  let half_carry = (hl & 0x0FFF) + (sp & 0x0FFF) > 0x0FFF;
  let carry = hl > 0xFFFF - sp;
  cpu.regs.set_flags_znhc(false, false, half_carry, carry);
  cpu.regs.set_hl(result);
}

pub fn ld_a_mhld(cpu: &mut Cpu) {
  let hl = cpu.regs.get_hl();
  let value = cpu.read(hl);
  cpu.regs.set_a(value);
  cpu.regs.set_hl(hl.wrapping_sub(1));
}

pub fn dec_sp(cpu: &mut Cpu) {
  cpu.regs.set_sp(cpu.regs.get_sp().wrapping_sub(1));
}

pub fn inc_a(cpu: &mut Cpu) {
  let a = cpu.regs.get_a().wrapping_add(1);
  cpu
    .regs
    .set_flags_znhc(a == 0, false, (a & 0x0F) == 0, false);
  cpu.regs.set_a(a);
}

pub fn dec_a(cpu: &mut Cpu) {
  let a = cpu.regs.get_a().wrapping_sub(1);
  cpu
    .regs
    .set_flags_znhc(a == 0, true, (a & 0x0F) == 0x0F, false);
  cpu.regs.set_a(a);
}

pub fn ld_a_u8(cpu: &mut Cpu) {
  let byte = cpu.fetch_byte();
  cpu.regs.set_a(byte);
}

pub fn ccf(cpu: &mut Cpu) {
  let carry = !cpu.regs.get_flags_c();
  cpu.regs.set_flags_znhc(false, false, false, carry);
}

pub fn ld_b_b(cpu: &mut Cpu) {
  cpu.regs.set_b(cpu.regs.get_b());
}

pub fn ld_b_c(cpu: &mut Cpu) {
  cpu.regs.set_b(cpu.regs.get_c());
}

pub fn ld_b_d(cpu: &mut Cpu) {
  cpu.regs.set_b(cpu.regs.get_d());
}

pub fn ld_b_e(cpu: &mut Cpu) {
  cpu.regs.set_b(cpu.regs.get_e());
}

pub fn ld_b_h(cpu: &mut Cpu) {
  cpu.regs.set_b(cpu.regs.get_h());
}

pub fn ld_b_l(cpu: &mut Cpu) {
  cpu.regs.set_b(cpu.regs.get_l());
}

pub fn ld_b_mhl(cpu: &mut Cpu) {
  let hl = cpu.regs.get_hl();
  let value = cpu.read(hl);
  cpu.regs.set_b(value);
}

pub fn ld_b_a(cpu: &mut Cpu) {
  cpu.regs.set_b(cpu.regs.get_a());
}

pub fn ld_c_b(cpu: &mut Cpu) {
  cpu.regs.set_c(cpu.regs.get_b());
}

pub fn ld_c_c(cpu: &mut Cpu) {
  cpu.regs.set_c(cpu.regs.get_c());
}

pub fn ld_c_d(cpu: &mut Cpu) {
  cpu.regs.set_c(cpu.regs.get_d());
}

pub fn ld_c_e(cpu: &mut Cpu) {
  cpu.regs.set_c(cpu.regs.get_e());
}

pub fn ld_c_h(cpu: &mut Cpu) {
  cpu.regs.set_c(cpu.regs.get_h());
}

pub fn ld_c_l(cpu: &mut Cpu) {
  cpu.regs.set_c(cpu.regs.get_l());
}

pub fn ld_c_mhl(cpu: &mut Cpu) {
  let hl = cpu.regs.get_hl();
  let value = cpu.read(hl);
  cpu.regs.set_c(value);
}

pub fn ld_c_a(cpu: &mut Cpu) {
  cpu.regs.set_c(cpu.regs.get_a());
}

pub fn ld_d_b(cpu: &mut Cpu) {
  cpu.regs.set_d(cpu.regs.get_b());
}

pub fn ld_d_c(cpu: &mut Cpu) {
  cpu.regs.set_d(cpu.regs.get_c());
}

pub fn ld_d_d(cpu: &mut Cpu) {
  cpu.regs.set_d(cpu.regs.get_d());
}

pub fn ld_d_e(cpu: &mut Cpu) {
  cpu.regs.set_d(cpu.regs.get_e());
}

pub fn ld_d_h(cpu: &mut Cpu) {
  cpu.regs.set_d(cpu.regs.get_h());
}

pub fn ld_d_l(cpu: &mut Cpu) {
  cpu.regs.set_d(cpu.regs.get_l());
}

pub fn ld_d_mhl(cpu: &mut Cpu) {
  let hl = cpu.regs.get_hl();
  let value = cpu.read(hl);
  cpu.regs.set_d(value);
}

pub fn ld_d_a(cpu: &mut Cpu) {
  cpu.regs.set_d(cpu.regs.get_a());
}

pub fn ld_e_b(cpu: &mut Cpu) {
  cpu.regs.set_e(cpu.regs.get_b());
}

pub fn ld_e_c(cpu: &mut Cpu) {
  cpu.regs.set_e(cpu.regs.get_c());
}

pub fn ld_e_d(cpu: &mut Cpu) {
  cpu.regs.set_e(cpu.regs.get_d());
}

pub fn ld_e_e(cpu: &mut Cpu) {
  cpu.regs.set_e(cpu.regs.get_e());
}

pub fn ld_e_h(cpu: &mut Cpu) {
  cpu.regs.set_e(cpu.regs.get_h());
}

pub fn ld_e_l(cpu: &mut Cpu) {
  cpu.regs.set_e(cpu.regs.get_l());
}

pub fn ld_e_mhl(cpu: &mut Cpu) {
  let hl = cpu.regs.get_hl();
  let value = cpu.read(hl);
  cpu.regs.set_e(value);
}

pub fn ld_e_a(cpu: &mut Cpu) {
  cpu.regs.set_e(cpu.regs.get_a());
}

pub fn ld_h_b(cpu: &mut Cpu) {
  cpu.regs.set_h(cpu.regs.get_b());
}

pub fn ld_h_c(cpu: &mut Cpu) {
  cpu.regs.set_h(cpu.regs.get_c());
}

pub fn ld_h_d(cpu: &mut Cpu) {
  cpu.regs.set_h(cpu.regs.get_d());
}

pub fn ld_h_e(cpu: &mut Cpu) {
  cpu.regs.set_h(cpu.regs.get_e());
}

pub fn ld_h_h(cpu: &mut Cpu) {
  cpu.regs.set_h(cpu.regs.get_h());
}

pub fn ld_h_l(cpu: &mut Cpu) {
  cpu.regs.set_h(cpu.regs.get_l());
}

pub fn ld_h_mhl(cpu: &mut Cpu) {
  let hl = cpu.regs.get_hl();
  let value = cpu.read(hl);
  cpu.regs.set_h(value);
}

pub fn ld_h_a(cpu: &mut Cpu) {
  cpu.regs.set_h(cpu.regs.get_a());
}

pub fn ld_l_b(cpu: &mut Cpu) {
  cpu.regs.set_l(cpu.regs.get_b());
}

pub fn ld_l_c(cpu: &mut Cpu) {
  cpu.regs.set_l(cpu.regs.get_c());
}

pub fn ld_l_d(cpu: &mut Cpu) {
  cpu.regs.set_l(cpu.regs.get_d());
}

pub fn ld_l_e(cpu: &mut Cpu) {
  cpu.regs.set_l(cpu.regs.get_e());
}

pub fn ld_l_h(cpu: &mut Cpu) {
  cpu.regs.set_l(cpu.regs.get_h());
}

pub fn ld_l_l(cpu: &mut Cpu) {
  cpu.regs.set_l(cpu.regs.get_l());
}

pub fn ld_l_mhl(cpu: &mut Cpu) {
  let hl = cpu.regs.get_hl();
  let value = cpu.read(hl);
  cpu.regs.set_l(value);
}

pub fn ld_l_a(cpu: &mut Cpu) {
  cpu.regs.set_l(cpu.regs.get_a());
}

pub fn ld_mhl_b(cpu: &mut Cpu) {
  let hl = cpu.regs.get_hl();
  let b = cpu.regs.get_b();
  cpu.write(hl, b);
}

pub fn ld_mhl_c(cpu: &mut Cpu) {
  let hl = cpu.regs.get_hl();
  let c = cpu.regs.get_c();
  cpu.write(hl, c);
}

pub fn ld_mhl_d(cpu: &mut Cpu) {
  let hl = cpu.regs.get_hl();
  let d = cpu.regs.get_d();
  cpu.write(hl, d);
}

pub fn ld_mhl_e(cpu: &mut Cpu) {
  let hl = cpu.regs.get_hl();
  let e = cpu.regs.get_e();
  cpu.write(hl, e);
}

pub fn ld_mhl_h(cpu: &mut Cpu) {
  let hl = cpu.regs.get_hl();
  let h = cpu.regs.get_h();
  cpu.write(hl, h);
}

pub fn ld_mhl_l(cpu: &mut Cpu) {
  let hl = cpu.regs.get_hl();
  let l = cpu.regs.get_l();
  cpu.write(hl, l);
}

pub fn halt(cpu: &mut Cpu) {
  cpu.halted = true;
}

pub fn ld_mhl_a(cpu: &mut Cpu) {
  let hl = cpu.regs.get_hl();
  let a = cpu.regs.get_a();
  cpu.write(hl, a);
}

pub fn ld_a_b(cpu: &mut Cpu) {
  cpu.regs.set_a(cpu.regs.get_b());
}

pub fn ld_a_c(cpu: &mut Cpu) {
  cpu.regs.set_a(cpu.regs.get_c());
}

pub fn ld_a_d(cpu: &mut Cpu) {
  cpu.regs.set_a(cpu.regs.get_d());
}

pub fn ld_a_e(cpu: &mut Cpu) {
  cpu.regs.set_a(cpu.regs.get_e());
}

pub fn ld_a_h(cpu: &mut Cpu) {
  cpu.regs.set_a(cpu.regs.get_h());
}

pub fn ld_a_l(cpu: &mut Cpu) {
  cpu.regs.set_a(cpu.regs.get_l());
}

pub fn ld_a_mhl(cpu: &mut Cpu) {
  let hl = cpu.regs.get_hl();
  let value = cpu.read(hl);
  cpu.regs.set_a(value);
}

pub fn ld_a_a(cpu: &mut Cpu) {
  cpu.regs.set_a(cpu.regs.get_a());
}

pub fn add_a_b(cpu: &mut Cpu) {
  let a = cpu.regs.get_a();
  let b = cpu.regs.get_b();
  let result = a.wrapping_add(b);
  let half_carry = (a & 0x0F) + (b & 0x0F) > 0x0F;
  let carry = a > 0xFF - b;
  cpu
    .regs
    .set_flags_znhc(result == 0, false, half_carry, carry);
  cpu.regs.set_a(result);
}

pub fn add_a_c(cpu: &mut Cpu) {
  let a = cpu.regs.get_a();
  let c = cpu.regs.get_c();
  let result = a.wrapping_add(c);
  let half_carry = (a & 0x0F) + (c & 0x0F) > 0x0F;
  let carry = a > 0xFF - c;
  cpu
    .regs
    .set_flags_znhc(result == 0, false, half_carry, carry);
  cpu.regs.set_a(result);
}

pub fn add_a_d(cpu: &mut Cpu) {
  let a = cpu.regs.get_a();
  let d = cpu.regs.get_d();
  let result = a.wrapping_add(d);
  let half_carry = (a & 0x0F) + (d & 0x0F) > 0x0F;
  let carry = a > 0xFF - d;
  cpu
    .regs
    .set_flags_znhc(result == 0, false, half_carry, carry);
  cpu.regs.set_a(result);
}

pub fn add_a_e(cpu: &mut Cpu) {
  let a = cpu.regs.get_a();
  let e = cpu.regs.get_e();
  let result = a.wrapping_add(e);
  let half_carry = (a & 0x0F) + (e & 0x0F) > 0x0F;
  let carry = a > 0xFF - e;
  cpu
    .regs
    .set_flags_znhc(result == 0, false, half_carry, carry);
  cpu.regs.set_a(result);
}

pub fn add_a_h(cpu: &mut Cpu) {
  let a = cpu.regs.get_a();
  let h = cpu.regs.get_h();
  let result = a.wrapping_add(h);
  let half_carry = (a & 0x0F) + (h & 0x0F) > 0x0F;
  let carry = a > 0xFF - h;
  cpu
    .regs
    .set_flags_znhc(result == 0, false, half_carry, carry);
  cpu.regs.set_a(result);
}

pub fn add_a_l(cpu: &mut Cpu) {
  let a = cpu.regs.get_a();
  let l = cpu.regs.get_l();
  let result = a.wrapping_add(l);
  let half_carry = (a & 0x0F) + (l & 0x0F) > 0x0F;
  let carry = a > 0xFF - l;
  cpu
    .regs
    .set_flags_znhc(result == 0, false, half_carry, carry);
  cpu.regs.set_a(result);
}

pub fn add_a_mhl(cpu: &mut Cpu) {
  let a = cpu.regs.get_a();
  let hl = cpu.regs.get_hl();
  let value = cpu.read(hl);
  let result = a.wrapping_add(value);
  let half_carry = (a & 0x0F) + (value & 0x0F) > 0x0F;
  let carry = a > 0xFF - value;
  cpu
    .regs
    .set_flags_znhc(result == 0, false, half_carry, carry);
  cpu.regs.set_a(result);
}

pub fn add_a_a(cpu: &mut Cpu) {
  let a = cpu.regs.get_a();
  let result = a.wrapping_add(a);
  let half_carry = (a & 0x0F) + (a & 0x0F) > 0x0F;
  let carry = a > 0xFF - a;
  cpu
    .regs
    .set_flags_znhc(result == 0, false, half_carry, carry);
  cpu.regs.set_a(result);
}

pub fn adc_a_b(cpu: &mut Cpu) {
  let a = cpu.regs.get_a();
  let b = cpu.regs.get_b();
  let carry = if cpu.regs.get_flags_c() { 1 } else { 0 };
  let result = a.wrapping_add(b).wrapping_add(carry);
  let half_carry = (a & 0x0F) + (b & 0x0F) + carry > 0x0F;
  let carry = a > 0xFF - b - carry;
  cpu
    .regs
    .set_flags_znhc(result == 0, false, half_carry, carry);
  cpu.regs.set_a(result);
}

pub fn adc_a_c(cpu: &mut Cpu) {
  let a = cpu.regs.get_a();
  let c = cpu.regs.get_c();
  let carry = if cpu.regs.get_flags_c() { 1 } else { 0 };
  let result = a.wrapping_add(c).wrapping_add(carry);
  let half_carry = (a & 0x0F) + (c & 0x0F) + carry > 0x0F;
  let carry = a > 0xFF - c - carry;
  cpu
    .regs
    .set_flags_znhc(result == 0, false, half_carry, carry);
  cpu.regs.set_a(result);
}

pub fn adc_a_d(cpu: &mut Cpu) {
  let a = cpu.regs.get_a();
  let d = cpu.regs.get_d();
  let carry = if cpu.regs.get_flags_c() { 1 } else { 0 };
  let result = a.wrapping_add(d).wrapping_add(carry);
  let half_carry = (a & 0x0F) + (d & 0x0F) + carry > 0x0F;
  let carry = a > 0xFF - d - carry;
  cpu
    .regs
    .set_flags_znhc(result == 0, false, half_carry, carry);
  cpu.regs.set_a(result);
}

pub fn adc_a_e(cpu: &mut Cpu) {
  let a = cpu.regs.get_a();
  let e = cpu.regs.get_e();
  let carry = if cpu.regs.get_flags_c() { 1 } else { 0 };
  let result = a.wrapping_add(e).wrapping_add(carry);
  let half_carry = (a & 0x0F) + (e & 0x0F) + carry > 0x0F;
  let carry = a > 0xFF - e - carry;
  cpu
    .regs
    .set_flags_znhc(result == 0, false, half_carry, carry);
  cpu.regs.set_a(result);
}

pub fn adc_a_h(cpu: &mut Cpu) {
  let a = cpu.regs.get_a();
  let h = cpu.regs.get_h();
  let carry = if cpu.regs.get_flags_c() { 1 } else { 0 };
  let result = a.wrapping_add(h).wrapping_add(carry);
  let half_carry = (a & 0x0F) + (h & 0x0F) + carry > 0x0F;
  let carry = a > 0xFF - h - carry;
  cpu
    .regs
    .set_flags_znhc(result == 0, false, half_carry, carry);
  cpu.regs.set_a(result);
}

pub fn adc_a_l(cpu: &mut Cpu) {
  let a = cpu.regs.get_a();
  let l = cpu.regs.get_l();
  let carry = if cpu.regs.get_flags_c() { 1 } else { 0 };
  let result = a.wrapping_add(l).wrapping_add(carry);
  let half_carry = (a & 0x0F) + (l & 0x0F) + carry > 0x0F;
  let carry = a > 0xFF - l - carry;
  cpu
    .regs
    .set_flags_znhc(result == 0, false, half_carry, carry);
  cpu.regs.set_a(result);
}

pub fn adc_a_mhl(cpu: &mut Cpu) {
  let a = cpu.regs.get_a();
  let hl = cpu.regs.get_hl();
  let value = cpu.read(hl);
  let carry = if cpu.regs.get_flags_c() { 1 } else { 0 };
  let result = a.wrapping_add(value).wrapping_add(carry);
  let half_carry = (a & 0x0F) + (value & 0x0F) + carry > 0x0F;
  let carry = a > 0xFF - value - carry;
  cpu
    .regs
    .set_flags_znhc(result == 0, false, half_carry, carry);
  cpu.regs.set_a(result);
}

pub fn adc_a_a(cpu: &mut Cpu) {
  let a = cpu.regs.get_a();
  let carry = if cpu.regs.get_flags_c() { 1 } else { 0 };
  let result = a.wrapping_add(a).wrapping_add(carry);
  let half_carry = (a & 0x0F) + (a & 0x0F) + carry > 0x0F;
  let carry = a > 0xFF - a - carry;
  cpu
    .regs
    .set_flags_znhc(result == 0, false, half_carry, carry);
  cpu.regs.set_a(result);
}

pub fn sub_a_b(cpu: &mut Cpu) {
  let a = cpu.regs.get_a();
  let b = cpu.regs.get_b();
  let result = a.wrapping_sub(b);
  let half_carry = (a & 0x0F) < (b & 0x0F);
  let carry = a < b;
  cpu
    .regs
    .set_flags_znhc(result == 0, true, half_carry, carry);
  cpu.regs.set_a(result);
}

pub fn sub_a_c(cpu: &mut Cpu) {
  let a = cpu.regs.get_a();
  let c = cpu.regs.get_c();
  let result = a.wrapping_sub(c);
  let half_carry = (a & 0x0F) < (c & 0x0F);
  let carry = a < c;
  cpu
    .regs
    .set_flags_znhc(result == 0, true, half_carry, carry);
  cpu.regs.set_a(result);
}

pub fn sub_a_d(cpu: &mut Cpu) {
  let a = cpu.regs.get_a();
  let d = cpu.regs.get_d();
  let result = a.wrapping_sub(d);
  let half_carry = (a & 0x0F) < (d & 0x0F);
  let carry = a < d;
  cpu
    .regs
    .set_flags_znhc(result == 0, true, half_carry, carry);
  cpu.regs.set_a(result);
}

pub fn sub_a_e(cpu: &mut Cpu) {
  let a = cpu.regs.get_a();
  let e = cpu.regs.get_e();
  let result = a.wrapping_sub(e);
  let half_carry = (a & 0x0F) < (e & 0x0F);
  let carry = a < e;
  cpu
    .regs
    .set_flags_znhc(result == 0, true, half_carry, carry);
  cpu.regs.set_a(result);
}

pub fn sub_a_h(cpu: &mut Cpu) {
  let a = cpu.regs.get_a();
  let h = cpu.regs.get_h();
  let result = a.wrapping_sub(h);
  let half_carry = (a & 0x0F) < (h & 0x0F);
  let carry = a < h;
  cpu
    .regs
    .set_flags_znhc(result == 0, true, half_carry, carry);
  cpu.regs.set_a(result);
}

pub fn sub_a_l(cpu: &mut Cpu) {
  let a = cpu.regs.get_a();
  let l = cpu.regs.get_l();
  let result = a.wrapping_sub(l);
  let half_carry = (a & 0x0F) < (l & 0x0F);
  let carry = a < l;
  cpu
    .regs
    .set_flags_znhc(result == 0, true, half_carry, carry);
  cpu.regs.set_a(result);
}

pub fn sub_a_mhl(cpu: &mut Cpu) {
  let a = cpu.regs.get_a();
  let hl = cpu.regs.get_hl();
  let value = cpu.read(hl);
  let result = a.wrapping_sub(value);
  let half_carry = (a & 0x0F) < (value & 0x0F);
  let carry = a < value;
  cpu
    .regs
    .set_flags_znhc(result == 0, true, half_carry, carry);
  cpu.regs.set_a(result);
}

// TODO: check this
pub fn sub_a_a(_cpu: &mut Cpu) {
  todo!()
  // let a = cpu.regs.get_a();
  // let result = a.wrapping_sub(a);
  // let half_carry = (a & 0x0F) < (a & 0x0F);
  // let carry = a < a;
  // cpu
  //   .regs
  //   .set_flags_znhc(result == 0, true, half_carry, carry);
  // cpu.regs.set_a(result);
}

pub fn sbc_a_b(cpu: &mut Cpu) {
  let a = cpu.regs.get_a();
  let b = cpu.regs.get_b();
  let carry = if cpu.regs.get_flags_c() { 1 } else { 0 };
  let result = a.wrapping_sub(b).wrapping_sub(carry);
  let half_carry = (a & 0x0F) < (b & 0x0F) + carry;
  let carry = a < b + carry;
  cpu
    .regs
    .set_flags_znhc(result == 0, true, half_carry, carry);
  cpu.regs.set_a(result);
}

pub fn sbc_a_c(cpu: &mut Cpu) {
  let a = cpu.regs.get_a();
  let c = cpu.regs.get_c();
  let carry = if cpu.regs.get_flags_c() { 1 } else { 0 };
  let result = a.wrapping_sub(c).wrapping_sub(carry);
  let half_carry = (a & 0x0F) < (c & 0x0F) + carry;
  let carry = a < c + carry;
  cpu
    .regs
    .set_flags_znhc(result == 0, true, half_carry, carry);
  cpu.regs.set_a(result);
}

pub fn sbc_a_d(cpu: &mut Cpu) {
  let a = cpu.regs.get_a();
  let d = cpu.regs.get_d();
  let carry = if cpu.regs.get_flags_c() { 1 } else { 0 };
  let result = a.wrapping_sub(d).wrapping_sub(carry);
  let half_carry = (a & 0x0F) < (d & 0x0F) + carry;
  let carry = a < d + carry;
  cpu
    .regs
    .set_flags_znhc(result == 0, true, half_carry, carry);
  cpu.regs.set_a(result);
}

pub fn sbc_a_e(cpu: &mut Cpu) {
  let a = cpu.regs.get_a();
  let e = cpu.regs.get_e();
  let carry = if cpu.regs.get_flags_c() { 1 } else { 0 };
  let result = a.wrapping_sub(e).wrapping_sub(carry);
  let half_carry = (a & 0x0F) < (e & 0x0F) + carry;
  let carry = a < e + carry;
  cpu
    .regs
    .set_flags_znhc(result == 0, true, half_carry, carry);
  cpu.regs.set_a(result);
}

pub fn sbc_a_h(cpu: &mut Cpu) {
  let a = cpu.regs.get_a();
  let h = cpu.regs.get_h();
  let carry = if cpu.regs.get_flags_c() { 1 } else { 0 };
  let result = a.wrapping_sub(h).wrapping_sub(carry);
  let half_carry = (a & 0x0F) < (h & 0x0F) + carry;
  let carry = a < h + carry;
  cpu
    .regs
    .set_flags_znhc(result == 0, true, half_carry, carry);
  cpu.regs.set_a(result);
}

pub fn sbc_a_l(cpu: &mut Cpu) {
  let a = cpu.regs.get_a();
  let l = cpu.regs.get_l();
  let carry = if cpu.regs.get_flags_c() { 1 } else { 0 };
  let result = a.wrapping_sub(l).wrapping_sub(carry);
  let half_carry = (a & 0x0F) < (l & 0x0F) + carry;
  let carry = a < l + carry;
  cpu
    .regs
    .set_flags_znhc(result == 0, true, half_carry, carry);
  cpu.regs.set_a(result);
}

pub fn sbc_a_mhl(cpu: &mut Cpu) {
  let a = cpu.regs.get_a();
  let hl = cpu.regs.get_hl();
  let value = cpu.read(hl);
  let carry = if cpu.regs.get_flags_c() { 1 } else { 0 };
  let result = a.wrapping_sub(value).wrapping_sub(carry);
  let half_carry = (a & 0x0F) < (value & 0x0F) + carry;
  let carry = a < value + carry;
  cpu
    .regs
    .set_flags_znhc(result == 0, true, half_carry, carry);
  cpu.regs.set_a(result);
}

pub fn sbc_a_a(cpu: &mut Cpu) {
  let a = cpu.regs.get_a();
  let carry = if cpu.regs.get_flags_c() { 1 } else { 0 };
  let result = a.wrapping_sub(a).wrapping_sub(carry);
  let half_carry = (a & 0x0F) < (a & 0x0F) + carry;
  let carry = a < a + carry;
  cpu
    .regs
    .set_flags_znhc(result == 0, true, half_carry, carry);
  cpu.regs.set_a(result);
}

pub fn and_a_b(cpu: &mut Cpu) {
  let a = cpu.regs.get_a();
  let b = cpu.regs.get_b();
  let result = a & b;
  cpu.regs.set_flags_znhc(result == 0, false, true, false);
  cpu.regs.set_a(result);
}

pub fn and_a_c(cpu: &mut Cpu) {
  let a = cpu.regs.get_a();
  let c = cpu.regs.get_c();
  let result = a & c;
  cpu.regs.set_flags_znhc(result == 0, false, true, false);
  cpu.regs.set_a(result);
}

pub fn and_a_d(cpu: &mut Cpu) {
  let a = cpu.regs.get_a();
  let d = cpu.regs.get_d();
  let result = a & d;
  cpu.regs.set_flags_znhc(result == 0, false, true, false);
  cpu.regs.set_a(result);
}

pub fn and_a_e(cpu: &mut Cpu) {
  let a = cpu.regs.get_a();
  let e = cpu.regs.get_e();
  let result = a & e;
  cpu.regs.set_flags_znhc(result == 0, false, true, false);
  cpu.regs.set_a(result);
}

pub fn and_a_h(cpu: &mut Cpu) {
  let a = cpu.regs.get_a();
  let h = cpu.regs.get_h();
  let result = a & h;
  cpu.regs.set_flags_znhc(result == 0, false, true, false);
  cpu.regs.set_a(result);
}

pub fn and_a_l(cpu: &mut Cpu) {
  let a = cpu.regs.get_a();
  let l = cpu.regs.get_l();
  let result = a & l;
  cpu.regs.set_flags_znhc(result == 0, false, true, false);
  cpu.regs.set_a(result);
}

pub fn and_a_mhl(cpu: &mut Cpu) {
  let a = cpu.regs.get_a();
  let hl = cpu.regs.get_hl();
  let value = cpu.read(hl);
  let result = a & value;
  cpu.regs.set_flags_znhc(result == 0, false, true, false);
  cpu.regs.set_a(result);
}

// TODO: check this
pub fn and_a_a(_cpu: &mut Cpu) {
  todo!()
  //   let a = cpu.regs.get_a();
  //   let result = a & a;
  //   cpu.regs.set_flags_znhc(result == 0, false, true, false);
  //   cpu.regs.set_a(result);
}

pub fn xor_a_b(cpu: &mut Cpu) {
  let a = cpu.regs.get_a();
  let b = cpu.regs.get_b();
  let result = a ^ b;
  cpu.regs.set_flags_znhc(result == 0, false, false, false);
  cpu.regs.set_a(result);
}

pub fn xor_a_c(cpu: &mut Cpu) {
  let a = cpu.regs.get_a();
  let c = cpu.regs.get_c();
  let result = a ^ c;
  cpu.regs.set_flags_znhc(result == 0, false, false, false);
  cpu.regs.set_a(result);
}

pub fn xor_a_d(cpu: &mut Cpu) {
  let a = cpu.regs.get_a();
  let d = cpu.regs.get_d();
  let result = a ^ d;
  cpu.regs.set_flags_znhc(result == 0, false, false, false);
  cpu.regs.set_a(result);
}

pub fn xor_a_e(cpu: &mut Cpu) {
  let a = cpu.regs.get_a();
  let e = cpu.regs.get_e();
  let result = a ^ e;
  cpu.regs.set_flags_znhc(result == 0, false, false, false);
  cpu.regs.set_a(result);
}

pub fn xor_a_h(cpu: &mut Cpu) {
  let a = cpu.regs.get_a();
  let h = cpu.regs.get_h();
  let result = a ^ h;
  cpu.regs.set_flags_znhc(result == 0, false, false, false);
  cpu.regs.set_a(result);
}

pub fn xor_a_l(cpu: &mut Cpu) {
  let a = cpu.regs.get_a();
  let l = cpu.regs.get_l();
  let result = a ^ l;
  cpu.regs.set_flags_znhc(result == 0, false, false, false);
  cpu.regs.set_a(result);
}

pub fn xor_a_mhl(cpu: &mut Cpu) {
  let a = cpu.regs.get_a();
  let hl = cpu.regs.get_hl();
  let value = cpu.read(hl);
  let result = a ^ value;
  cpu.regs.set_flags_znhc(result == 0, false, false, false);
  cpu.regs.set_a(result);
}

// TODO: check this
pub fn xor_a_a(_cpu: &mut Cpu) {
  todo!()
  // let a = cpu.regs.get_a();
  // let result = a ^ a;
  // cpu.regs.set_flags_znhc(result == 0, false, false, false);
  // cpu.regs.set_a(result);
}

pub fn or_a_b(cpu: &mut Cpu) {
  let a = cpu.regs.get_a();
  let b = cpu.regs.get_b();
  let result = a | b;
  cpu.regs.set_flags_znhc(result == 0, false, false, false);
  cpu.regs.set_a(result);
}

pub fn or_a_c(cpu: &mut Cpu) {
  let a = cpu.regs.get_a();
  let c = cpu.regs.get_c();
  let result = a | c;
  cpu.regs.set_flags_znhc(result == 0, false, false, false);
  cpu.regs.set_a(result);
}

pub fn or_a_d(cpu: &mut Cpu) {
  let a = cpu.regs.get_a();
  let d = cpu.regs.get_d();
  let result = a | d;
  cpu.regs.set_flags_znhc(result == 0, false, false, false);
  cpu.regs.set_a(result);
}

pub fn or_a_e(cpu: &mut Cpu) {
  let a = cpu.regs.get_a();
  let e = cpu.regs.get_e();
  let result = a | e;
  cpu.regs.set_flags_znhc(result == 0, false, false, false);
  cpu.regs.set_a(result);
}

pub fn or_a_h(cpu: &mut Cpu) {
  let a = cpu.regs.get_a();
  let h = cpu.regs.get_h();
  let result = a | h;
  cpu.regs.set_flags_znhc(result == 0, false, false, false);
  cpu.regs.set_a(result);
}

pub fn or_a_l(cpu: &mut Cpu) {
  let a = cpu.regs.get_a();
  let l = cpu.regs.get_l();
  let result = a | l;
  cpu.regs.set_flags_znhc(result == 0, false, false, false);
  cpu.regs.set_a(result);
}

pub fn or_a_mhl(cpu: &mut Cpu) {
  let a = cpu.regs.get_a();
  let hl = cpu.regs.get_hl();
  let value = cpu.read(hl);
  let result = a | value;
  cpu.regs.set_flags_znhc(result == 0, false, false, false);
  cpu.regs.set_a(result);
}

// TODO: check this
pub fn or_a_a(_cpu: &mut Cpu) {
  todo!()
  // let a = cpu.regs.get_a();
  // let result = a | a;
  // cpu.regs.set_flags_znhc(result == 0, false, false, false);
  // cpu.regs.set_a(result);
}

pub fn cp_a_b(cpu: &mut Cpu) {
  let a = cpu.regs.get_a();
  let b = cpu.regs.get_b();
  let result = a.wrapping_sub(b);
  let half_carry = (a & 0x0F) < (b & 0x0F);
  let carry = a < b;
  cpu
    .regs
    .set_flags_znhc(result == 0, true, half_carry, carry);
}

pub fn cp_a_c(cpu: &mut Cpu) {
  let a = cpu.regs.get_a();
  let c = cpu.regs.get_c();
  let result = a.wrapping_sub(c);
  let half_carry = (a & 0x0F) < (c & 0x0F);
  let carry = a < c;
  cpu
    .regs
    .set_flags_znhc(result == 0, true, half_carry, carry);
}

pub fn cp_a_d(cpu: &mut Cpu) {
  let a = cpu.regs.get_a();
  let d = cpu.regs.get_d();
  let result = a.wrapping_sub(d);
  let half_carry = (a & 0x0F) < (d & 0x0F);
  let carry = a < d;
  cpu
    .regs
    .set_flags_znhc(result == 0, true, half_carry, carry);
}

pub fn cp_a_e(cpu: &mut Cpu) {
  let a = cpu.regs.get_a();
  let e = cpu.regs.get_e();
  let result = a.wrapping_sub(e);
  let half_carry = (a & 0x0F) < (e & 0x0F);
  let carry = a < e;
  cpu
    .regs
    .set_flags_znhc(result == 0, true, half_carry, carry);
}

pub fn cp_a_h(cpu: &mut Cpu) {
  let a = cpu.regs.get_a();
  let h = cpu.regs.get_h();
  let result = a.wrapping_sub(h);
  let half_carry = (a & 0x0F) < (h & 0x0F);
  let carry = a < h;
  cpu
    .regs
    .set_flags_znhc(result == 0, true, half_carry, carry);
}

pub fn cp_a_l(cpu: &mut Cpu) {
  let a = cpu.regs.get_a();
  let l = cpu.regs.get_l();
  let result = a.wrapping_sub(l);
  let half_carry = (a & 0x0F) < (l & 0x0F);
  let carry = a < l;
  cpu
    .regs
    .set_flags_znhc(result == 0, true, half_carry, carry);
}

pub fn cp_a_mhl(cpu: &mut Cpu) {
  let a = cpu.regs.get_a();
  let hl = cpu.regs.get_hl();
  let value = cpu.read(hl);
  let result = a.wrapping_sub(value);
  let half_carry = (a & 0x0F) < (value & 0x0F);
  let carry = a < value;
  cpu
    .regs
    .set_flags_znhc(result == 0, true, half_carry, carry);
}

// TODO: check this
pub fn cp_a_a(_cpu: &mut Cpu) {
  todo!()
  // let a = cpu.regs.get_a();
  // let result = a.wrapping_sub(a);
  // let half_carry = (a & 0x0F) < (a & 0x0F);
  // let carry = a < a;
  // cpu
  //   .regs
  //   .set_flags_znhc(result == 0, true, half_carry, carry);
}

pub fn ret_nz(cpu: &mut Cpu) {
  if !cpu.regs.get_flags_z() {
    let lo = cpu.pop_byte();
    let hi = cpu.pop_byte();
    cpu.regs.set_pc((hi as u16) << 8 | lo as u16);
  }
}

pub fn pop_bc(cpu: &mut Cpu) {
  let lo = cpu.pop_byte();
  let hi = cpu.pop_byte();
  cpu.regs.set_bc((hi as u16) << 8 | lo as u16);
}

pub fn jp_nz_u16(cpu: &mut Cpu) {
  if !cpu.regs.get_flags_z() {
    let lo = cpu.read(cpu.regs.get_pc());
    let hi = cpu.read(cpu.regs.get_pc() + 1);
    cpu.regs.set_pc((hi as u16) << 8 | lo as u16);
  } else {
    cpu.regs.inc_pc(2);
  }
}

pub fn jp_u16(cpu: &mut Cpu) {
  let lo = cpu.read(cpu.regs.get_pc());
  let hi = cpu.read(cpu.regs.get_pc() + 1);
  cpu.regs.set_pc((hi as u16) << 8 | lo as u16);
}

pub fn call_nz_u16(cpu: &mut Cpu) {
  if !cpu.regs.get_flags_z() {
    let pc = cpu.regs.get_pc();
    let lo = cpu.read(pc);
    let hi = cpu.read(pc + 1);
    cpu.push_byte((pc >> 8) as u8);
    cpu.push_byte(pc as u8);
    cpu.regs.set_pc((hi as u16) << 8 | lo as u16);
  } else {
    cpu.regs.inc_pc(2);
  }
}

pub fn push_bc(cpu: &mut Cpu) {
  let bc = cpu.regs.get_bc();
  cpu.push_byte((bc >> 8) as u8);
  cpu.push_byte(bc as u8);
}

pub fn add_a_u8(cpu: &mut Cpu) {
  let a = cpu.regs.get_a();
  let value = cpu.read(cpu.regs.get_pc());
  let result = a.wrapping_add(value);
  let half_carry = (a & 0x0F) + (value & 0x0F) > 0x0F;
  let carry = a > 0xFF - value;
  cpu
    .regs
    .set_flags_znhc(result == 0, false, half_carry, carry);
  cpu.regs.inc_pc(1);
  cpu.regs.set_a(result);
}

pub fn rst_00h(cpu: &mut Cpu) {
  let pc = cpu.regs.get_pc();
  cpu.push_byte((pc >> 8) as u8);
  cpu.push_byte(pc as u8);
  cpu.regs.set_pc(0x00);
}

pub fn ret_z(cpu: &mut Cpu) {
  if cpu.regs.get_flags_z() {
    let lo = cpu.pop_byte();
    let hi = cpu.pop_byte();
    cpu.regs.set_pc((hi as u16) << 8 | lo as u16);
  }
}

pub fn ret(cpu: &mut Cpu) {
  let lo = cpu.pop_byte();
  let hi = cpu.pop_byte();
  cpu.regs.set_pc((hi as u16) << 8 | lo as u16);
}

pub fn jp_z_u16(cpu: &mut Cpu) {
  if cpu.regs.get_flags_z() {
    let lo = cpu.read(cpu.regs.get_pc());
    let hi = cpu.read(cpu.regs.get_pc() + 1);
    cpu.regs.set_pc((hi as u16) << 8 | lo as u16);
  } else {
    cpu.regs.inc_pc(2);
  }
}

// TODO: check this
pub fn prefix_cb(_cpu: &mut Cpu) {
  panic!("CB prefix not implemented");
}

pub fn call_z_u16(cpu: &mut Cpu) {
  if cpu.regs.get_flags_z() {
    let pc = cpu.regs.get_pc();
    let lo = cpu.read(pc);
    let hi = cpu.read(pc + 1);
    cpu.push_byte((pc >> 8) as u8);
    cpu.push_byte(pc as u8);
    cpu.regs.set_pc((hi as u16) << 8 | lo as u16);
  } else {
    cpu.regs.inc_pc(2);
  }
}

pub fn call_u16(cpu: &mut Cpu) {
  let pc = cpu.regs.get_pc();
  let lo = cpu.read(pc);
  let hi = cpu.read(pc + 1);
  cpu.push_byte((pc >> 8) as u8);
  cpu.push_byte(pc as u8);
  cpu.regs.set_pc((hi as u16) << 8 | lo as u16);
}

pub fn adc_a_u8(cpu: &mut Cpu) {
  let a = cpu.regs.get_a();
  let value = cpu.read(cpu.regs.get_pc());
  let carry = if cpu.regs.get_flags_c() { 1 } else { 0 };
  let result = a.wrapping_add(value).wrapping_add(carry);
  let half_carry = (a & 0x0F) + (value & 0x0F) + carry > 0x0F;
  let carry = a > 0xFF - value - carry;
  cpu
    .regs
    .set_flags_znhc(result == 0, false, half_carry, carry);
  cpu.regs.inc_pc(1);
  cpu.regs.set_a(result);
}

pub fn rst_08h(cpu: &mut Cpu) {
  let pc = cpu.regs.get_pc();
  cpu.push_byte((pc >> 8) as u8);
  cpu.push_byte(pc as u8);
  cpu.regs.set_pc(0x08);
}

pub fn ret_nc(cpu: &mut Cpu) {
  if !cpu.regs.get_flags_c() {
    let lo = cpu.pop_byte();
    let hi = cpu.pop_byte();
    cpu.regs.set_pc((hi as u16) << 8 | lo as u16);
  }
}

pub fn pop_de(cpu: &mut Cpu) {
  let lo = cpu.pop_byte();
  let hi = cpu.pop_byte();
  cpu.regs.set_de((hi as u16) << 8 | lo as u16);
}

pub fn jp_nc_u16(cpu: &mut Cpu) {
  if !cpu.regs.get_flags_c() {
    let lo = cpu.read(cpu.regs.get_pc());
    let hi = cpu.read(cpu.regs.get_pc() + 1);
    cpu.regs.set_pc((hi as u16) << 8 | lo as u16);
  } else {
    cpu.regs.inc_pc(2);
  }
}

pub fn call_nc_u16(cpu: &mut Cpu) {
  if !cpu.regs.get_flags_c() {
    let pc = cpu.regs.get_pc();
    let lo = cpu.read(pc);
    let hi = cpu.read(pc + 1);
    cpu.push_byte((pc >> 8) as u8);
    cpu.push_byte(pc as u8);
    cpu.regs.set_pc((hi as u16) << 8 | lo as u16);
  } else {
    cpu.regs.inc_pc(2);
  }
}

pub fn push_de(cpu: &mut Cpu) {
  let de = cpu.regs.get_de();
  cpu.push_byte((de >> 8) as u8);
  cpu.push_byte(de as u8);
}

pub fn sub_a_u8(cpu: &mut Cpu) {
  let a = cpu.regs.get_a();
  let value = cpu.read(cpu.regs.get_pc());
  let result = a.wrapping_sub(value);
  let half_carry = (a & 0x0F) < (value & 0x0F);
  let carry = a < value;
  cpu
    .regs
    .set_flags_znhc(result == 0, true, half_carry, carry);
  cpu.regs.inc_pc(1);
  cpu.regs.set_a(result);
}

pub fn rst_10h(cpu: &mut Cpu) {
  let pc = cpu.regs.get_pc();
  cpu.push_byte((pc >> 8) as u8);
  cpu.push_byte(pc as u8);
  cpu.regs.set_pc(0x10);
}

pub fn ret_c(cpu: &mut Cpu) {
  if cpu.regs.get_flags_c() {
    let lo = cpu.pop_byte();
    let hi = cpu.pop_byte();
    cpu.regs.set_pc((hi as u16) << 8 | lo as u16);
  }
}

// TODO: fix this
pub fn reti(_cpu: &mut Cpu) {
  todo!()
  // let lo = cpu.pop_byte();
  // let hi = cpu.pop_byte();
  // cpu.regs.set_pc((hi as u16) << 8 | lo as u16);
  // cpu. = true;
}

pub fn jp_c_u16(cpu: &mut Cpu) {
  if cpu.regs.get_flags_c() {
    let lo = cpu.read(cpu.regs.get_pc());
    let hi = cpu.read(cpu.regs.get_pc() + 1);
    cpu.regs.set_pc((hi as u16) << 8 | lo as u16);
  } else {
    cpu.regs.inc_pc(2);
  }
}

pub fn call_c_u16(cpu: &mut Cpu) {
  if cpu.regs.get_flags_c() {
    let pc = cpu.regs.get_pc();
    let lo = cpu.read(pc);
    let hi = cpu.read(pc + 1);
    cpu.push_byte((pc >> 8) as u8);
    cpu.push_byte(pc as u8);
    cpu.regs.set_pc((hi as u16) << 8 | lo as u16);
  } else {
    cpu.regs.inc_pc(2);
  }
}

pub fn sbc_a_u8(cpu: &mut Cpu) {
  let a = cpu.regs.get_a();
  let value = cpu.read(cpu.regs.get_pc());
  let carry = if cpu.regs.get_flags_c() { 1 } else { 0 };
  let result = a.wrapping_sub(value).wrapping_sub(carry);
  let half_carry = (a & 0x0F) < (value & 0x0F) + carry;
  let carry = a < value + carry;
  cpu
    .regs
    .set_flags_znhc(result == 0, true, half_carry, carry);
  cpu.regs.inc_pc(1);
  cpu.regs.set_a(result);
}

pub fn rst_18h(cpu: &mut Cpu) {
  let pc = cpu.regs.get_pc();
  cpu.push_byte((pc >> 8) as u8);
  cpu.push_byte(pc as u8);
  cpu.regs.set_pc(0x18);
}

pub fn ld_mff00_u8_a(cpu: &mut Cpu) {
  let value = cpu.read(cpu.regs.get_pc());
  cpu.write(0xFF00 + value as u16, cpu.regs.get_a());
  cpu.regs.inc_pc(1);
}

pub fn pop_hl(cpu: &mut Cpu) {
  let lo = cpu.pop_byte();
  let hi = cpu.pop_byte();
  cpu.regs.set_hl((hi as u16) << 8 | lo as u16);
}

pub fn ld_mff00_c_a(cpu: &mut Cpu) {
  cpu.write(0xFF00 + cpu.regs.get_c() as u16, cpu.regs.get_a());
}

pub fn push_hl(cpu: &mut Cpu) {
  let hl = cpu.regs.get_hl();
  cpu.push_byte((hl >> 8) as u8);
  cpu.push_byte(hl as u8);
}

pub fn and_a_u8(cpu: &mut Cpu) {
  let a = cpu.regs.get_a();
  let value = cpu.read(cpu.regs.get_pc());
  let result = a & value;
  cpu.regs.set_flags_znhc(result == 0, false, true, false);
  cpu.regs.inc_pc(1);
  cpu.regs.set_a(result);
}

pub fn rst_20h(cpu: &mut Cpu) {
  let pc = cpu.regs.get_pc();
  cpu.push_byte((pc >> 8) as u8);
  cpu.push_byte(pc as u8);
  cpu.regs.set_pc(0x20);
}

// TODO: fix this
pub fn add_sp_i8(_cpu: &mut Cpu) {
  todo!()
  // let sp = cpu.regs.get_sp();
  // let value = cpu.read(cpu.regs.get_pc()) as i8 as i16;
  // let result = sp.wrapping_add(value as u16);
  // let half_carry = (sp & 0x0F) + (value & 0x0F) > 0x0F;
  // let carry = (sp & 0xFF) + (value & 0xFF) > 0xFF;
  // cpu.regs.set_flags_znhc(false, false, half_carry, carry);
  // cpu.regs.inc_pc(1);
  // cpu.regs.set_sp(result);
}

pub fn jp_hl(cpu: &mut Cpu) {
  cpu.regs.set_pc(cpu.regs.get_hl());
}

pub fn ld_mu16_a(cpu: &mut Cpu) {
  let lo = cpu.read(cpu.regs.get_pc());
  let hi = cpu.read(cpu.regs.get_pc() + 1);
  let address = (hi as u16) << 8 | lo as u16;
  cpu.write(address, cpu.regs.get_a());
  cpu.regs.inc_pc(2);
}

pub fn xor_a_u8(cpu: &mut Cpu) {
  let a = cpu.regs.get_a();
  let value = cpu.read(cpu.regs.get_pc());
  let result = a ^ value;
  cpu.regs.set_flags_znhc(result == 0, false, false, false);
  cpu.regs.inc_pc(1);
  cpu.regs.set_a(result);
}

pub fn rst_28h(cpu: &mut Cpu) {
  let pc = cpu.regs.get_pc();
  cpu.push_byte((pc >> 8) as u8);
  cpu.push_byte(pc as u8);
  cpu.regs.set_pc(0x28);
}

pub fn ld_a_mff00u8(cpu: &mut Cpu) {
  let value = cpu.read(cpu.regs.get_pc());
  cpu.regs.set_a(cpu.read(0xFF00 + value as u16));
  cpu.regs.inc_pc(1);
}

pub fn pop_af(cpu: &mut Cpu) {
  let lo = cpu.pop_byte();
  let hi = cpu.pop_byte();
  cpu.regs.set_af((hi as u16) << 8 | lo as u16);
}

pub fn ld_a_mff00_c(cpu: &mut Cpu) {
  cpu.regs.set_a(cpu.read(0xFF00 + cpu.regs.get_c() as u16));
}

// TODO: fix this
pub fn di(cpu: &mut Cpu) {
  cpu.disable_interrupts()
}

pub fn push_af(cpu: &mut Cpu) {
  let af = cpu.regs.get_af();
  cpu.push_byte((af >> 8) as u8);
  cpu.push_byte(af as u8);
}

pub fn or_a_u8(cpu: &mut Cpu) {
  let a = cpu.regs.get_a();
  let value = cpu.read(cpu.regs.get_pc());
  let result = a | value;
  cpu.regs.set_flags_znhc(result == 0, false, false, false);
  cpu.regs.inc_pc(1);
  cpu.regs.set_a(result);
}

pub fn rst_30h(cpu: &mut Cpu) {
  let pc = cpu.regs.get_pc();
  cpu.push_byte((pc >> 8) as u8);
  cpu.push_byte(pc as u8);
  cpu.regs.set_pc(0x30);
}

// TODO: fix this
pub fn ld_hl_sp_i8(_cpu: &mut Cpu) {
  todo!()
  // let sp = cpu.regs.get_sp();
  // let value = cpu.read(cpu.regs.get_pc()) as i8 as i16;
  // let result = sp.wrapping_add(value as u16);
  // let half_carry = (sp & 0x0F) + (value & 0x0F) > 0x0F;
  // let carry = (sp & 0xFF) + (value & 0xFF) > 0xFF;
  // cpu.regs.set_flags_znhc(false, false, half_carry, carry);
  // cpu.regs.inc_pc(1);
  // cpu.regs.set_hl(result);
}

pub fn ld_sp_hl(cpu: &mut Cpu) {
  cpu.regs.set_sp(cpu.regs.get_hl());
}

pub fn ld_a_mu16(cpu: &mut Cpu) {
  let lo = cpu.read(cpu.regs.get_pc());
  let hi = cpu.read(cpu.regs.get_pc() + 1);
  let address = (hi as u16) << 8 | lo as u16;
  cpu.regs.set_a(cpu.read(address));
  cpu.regs.inc_pc(2);
}

// TODO: fix this
pub fn ei(_cpu: &mut Cpu) {
  todo!()
}

pub fn cp_a_u8(cpu: &mut Cpu) {
  let a = cpu.regs.get_a();
  let value = cpu.read(cpu.regs.get_pc());
  let result = a.wrapping_sub(value);
  let half_carry = (a & 0x0F) < (value & 0x0F);
  let carry = a < value;
  cpu
    .regs
    .set_flags_znhc(result == 0, true, half_carry, carry);
  cpu.regs.inc_pc(1);
}

pub fn rst_38h(cpu: &mut Cpu) {
  let pc = cpu.regs.get_pc();
  cpu.push_byte((pc >> 8) as u8);
  cpu.push_byte(pc as u8);
  cpu.regs.set_pc(0x38);
}

pub fn rlc_b(cpu: &mut Cpu) {
  let b = cpu.regs.get_b();
  let carry = b & 0x80 != 0;
  let result = (b << 1) | (b >> 7);
  cpu.regs.set_flags_znhc(result == 0, false, false, carry);
  cpu.regs.set_b(result);
}

pub fn rlc_c(cpu: &mut Cpu) {
  let c = cpu.regs.get_c();
  let carry = c & 0x80 != 0;
  let result = (c << 1) | (c >> 7);
  cpu.regs.set_flags_znhc(result == 0, false, false, carry);
  cpu.regs.set_c(result);
}

pub fn rlc_d(cpu: &mut Cpu) {
  let d = cpu.regs.get_d();
  let carry = d & 0x80 != 0;
  let result = (d << 1) | (d >> 7);
  cpu.regs.set_flags_znhc(result == 0, false, false, carry);
  cpu.regs.set_d(result);
}

pub fn rlc_e(cpu: &mut Cpu) {
  let e = cpu.regs.get_e();
  let carry = e & 0x80 != 0;
  let result = (e << 1) | (e >> 7);
  cpu.regs.set_flags_znhc(result == 0, false, false, carry);
  cpu.regs.set_e(result);
}

pub fn rlc_h(cpu: &mut Cpu) {
  let h = cpu.regs.get_h();
  let carry = h & 0x80 != 0;
  let result = (h << 1) | (h >> 7);
  cpu.regs.set_flags_znhc(result == 0, false, false, carry);
  cpu.regs.set_h(result);
}

pub fn rlc_l(cpu: &mut Cpu) {
  let l = cpu.regs.get_l();
  let carry = l & 0x80 != 0;
  let result = (l << 1) | (l >> 7);
  cpu.regs.set_flags_znhc(result == 0, false, false, carry);
  cpu.regs.set_l(result);
}

pub fn rlc_mhl(cpu: &mut Cpu) {
  let hl = cpu.regs.get_hl();
  let value = cpu.read(hl);
  let carry = value & 0x80 != 0;
  let result = (value << 1) | (value >> 7);
  cpu.regs.set_flags_znhc(result == 0, false, false, carry);
  cpu.write(hl, result);
}

pub fn rlc_a(cpu: &mut Cpu) {
  let a = cpu.regs.get_a();
  let carry = a & 0x80 != 0;
  let result = (a << 1) | (a >> 7);
  cpu.regs.set_flags_znhc(result == 0, false, false, carry);
  cpu.regs.set_a(result);
}

pub fn rrc_b(cpu: &mut Cpu) {
  let b = cpu.regs.get_b();
  let carry = b & 0x01 != 0;
  let result = (b >> 1) | (b << 7);
  cpu.regs.set_flags_znhc(result == 0, false, false, carry);
  cpu.regs.set_b(result);
}

pub fn rrc_c(cpu: &mut Cpu) {
  let c = cpu.regs.get_c();
  let carry = c & 0x01 != 0;
  let result = (c >> 1) | (c << 7);
  cpu.regs.set_flags_znhc(result == 0, false, false, carry);
  cpu.regs.set_c(result);
}

pub fn rrc_d(cpu: &mut Cpu) {
  let d = cpu.regs.get_d();
  let carry = d & 0x01 != 0;
  let result = (d >> 1) | (d << 7);
  cpu.regs.set_flags_znhc(result == 0, false, false, carry);
  cpu.regs.set_d(result);
}

pub fn rrc_e(cpu: &mut Cpu) {
  let e = cpu.regs.get_e();
  let carry = e & 0x01 != 0;
  let result = (e >> 1) | (e << 7);
  cpu.regs.set_flags_znhc(result == 0, false, false, carry);
  cpu.regs.set_e(result);
}

pub fn rrc_h(cpu: &mut Cpu) {
  let h = cpu.regs.get_h();
  let carry = h & 0x01 != 0;
  let result = (h >> 1) | (h << 7);
  cpu.regs.set_flags_znhc(result == 0, false, false, carry);
  cpu.regs.set_h(result);
}

pub fn rrc_l(cpu: &mut Cpu) {
  let l = cpu.regs.get_l();
  let carry = l & 0x01 != 0;
  let result = (l >> 1) | (l << 7);
  cpu.regs.set_flags_znhc(result == 0, false, false, carry);
  cpu.regs.set_l(result);
}

pub fn rrc_mhl(cpu: &mut Cpu) {
  let hl = cpu.regs.get_hl();
  let value = cpu.read(hl);
  let carry = value & 0x01 != 0;
  let result = (value >> 1) | (value << 7);
  cpu.regs.set_flags_znhc(result == 0, false, false, carry);
  cpu.write(hl, result);
}

pub fn rrc_a(cpu: &mut Cpu) {
  let a = cpu.regs.get_a();
  let carry = a & 0x01 != 0;
  let result = (a >> 1) | (a << 7);
  cpu.regs.set_flags_znhc(result == 0, false, false, carry);
  cpu.regs.set_a(result);
}

pub fn rl_b(cpu: &mut Cpu) {
  let b = cpu.regs.get_b();
  let carry = b & 0x80 != 0;
  let result = (b << 1) | if cpu.regs.get_flags_c() { 1 } else { 0 };
  cpu.regs.set_flags_znhc(result == 0, false, false, carry);
  cpu.regs.set_b(result);
}

pub fn rl_c(cpu: &mut Cpu) {
  let c = cpu.regs.get_c();
  let carry = c & 0x80 != 0;
  let result = (c << 1) | if cpu.regs.get_flags_c() { 1 } else { 0 };
  cpu.regs.set_flags_znhc(result == 0, false, false, carry);
  cpu.regs.set_c(result);
}

pub fn rl_d(cpu: &mut Cpu) {
  let d = cpu.regs.get_d();
  let carry = d & 0x80 != 0;
  let result = (d << 1) | if cpu.regs.get_flags_c() { 1 } else { 0 };
  cpu.regs.set_flags_znhc(result == 0, false, false, carry);
  cpu.regs.set_d(result);
}

pub fn rl_e(cpu: &mut Cpu) {
  let e = cpu.regs.get_e();
  let carry = e & 0x80 != 0;
  let result = (e << 1) | if cpu.regs.get_flags_c() { 1 } else { 0 };
  cpu.regs.set_flags_znhc(result == 0, false, false, carry);
  cpu.regs.set_e(result);
}

pub fn rl_h(cpu: &mut Cpu) {
  let h = cpu.regs.get_h();
  let carry = h & 0x80 != 0;
  let result = (h << 1) | if cpu.regs.get_flags_c() { 1 } else { 0 };
  cpu.regs.set_flags_znhc(result == 0, false, false, carry);
  cpu.regs.set_h(result);
}

pub fn rl_l(cpu: &mut Cpu) {
  let l = cpu.regs.get_l();
  let carry = l & 0x80 != 0;
  let result = (l << 1) | if cpu.regs.get_flags_c() { 1 } else { 0 };
  cpu.regs.set_flags_znhc(result == 0, false, false, carry);
  cpu.regs.set_l(result);
}

pub fn rl_mhl(cpu: &mut Cpu) {
  let hl = cpu.regs.get_hl();
  let value = cpu.read(hl);
  let carry = value & 0x80 != 0;
  let result = (value << 1) | if cpu.regs.get_flags_c() { 1 } else { 0 };
  cpu.regs.set_flags_znhc(result == 0, false, false, carry);
  cpu.write(hl, result);
}

pub fn rl_a(cpu: &mut Cpu) {
  let a = cpu.regs.get_a();
  let carry = a & 0x80 != 0;
  let result = (a << 1) | if cpu.regs.get_flags_c() { 1 } else { 0 };
  cpu.regs.set_flags_znhc(result == 0, false, false, carry);
  cpu.regs.set_a(result);
}

pub fn rr_b(cpu: &mut Cpu) {
  let b = cpu.regs.get_b();
  let carry = b & 0x01 != 0;
  let result = (b >> 1) | if cpu.regs.get_flags_c() { 0x80 } else { 0 };
  cpu.regs.set_flags_znhc(result == 0, false, false, carry);
  cpu.regs.set_b(result);
}

pub fn rr_c(cpu: &mut Cpu) {
  let c = cpu.regs.get_c();
  let carry = c & 0x01 != 0;
  let result = (c >> 1) | if cpu.regs.get_flags_c() { 0x80 } else { 0 };
  cpu.regs.set_flags_znhc(result == 0, false, false, carry);
  cpu.regs.set_c(result);
}

pub fn rr_d(cpu: &mut Cpu) {
  let d = cpu.regs.get_d();
  let carry = d & 0x01 != 0;
  let result = (d >> 1) | if cpu.regs.get_flags_c() { 0x80 } else { 0 };
  cpu.regs.set_flags_znhc(result == 0, false, false, carry);
  cpu.regs.set_d(result);
}

pub fn rr_e(cpu: &mut Cpu) {
  let e = cpu.regs.get_e();
  let carry = e & 0x01 != 0;
  let result = (e >> 1) | if cpu.regs.get_flags_c() { 0x80 } else { 0 };
  cpu.regs.set_flags_znhc(result == 0, false, false, carry);
  cpu.regs.set_e(result);
}

pub fn rr_h(cpu: &mut Cpu) {
  let h = cpu.regs.get_h();
  let carry = h & 0x01 != 0;
  let result = (h >> 1) | if cpu.regs.get_flags_c() { 0x80 } else { 0 };
  cpu.regs.set_flags_znhc(result == 0, false, false, carry);
  cpu.regs.set_h(result);
}

pub fn rr_l(cpu: &mut Cpu) {
  let l = cpu.regs.get_l();
  let carry = l & 0x01 != 0;
  let result = (l >> 1) | if cpu.regs.get_flags_c() { 0x80 } else { 0 };
  cpu.regs.set_flags_znhc(result == 0, false, false, carry);
  cpu.regs.set_l(result);
}

pub fn rr_mhl(cpu: &mut Cpu) {
  let hl = cpu.regs.get_hl();
  let value = cpu.read(hl);
  let carry = value & 0x01 != 0;
  let result = (value >> 1) | if cpu.regs.get_flags_c() { 0x80 } else { 0 };
  cpu.regs.set_flags_znhc(result == 0, false, false, carry);
  cpu.write(hl, result);
}

pub fn rr_a(cpu: &mut Cpu) {
  let a = cpu.regs.get_a();
  let carry = a & 0x01 != 0;
  let result = (a >> 1) | if cpu.regs.get_flags_c() { 0x80 } else { 0 };
  cpu.regs.set_flags_znhc(result == 0, false, false, carry);
  cpu.regs.set_a(result);
}

pub fn sla_b(cpu: &mut Cpu) {
  let b = cpu.regs.get_b();
  let carry = b & 0x80 != 0;
  let result = b << 1;
  cpu.regs.set_flags_znhc(result == 0, false, false, carry);
  cpu.regs.set_b(result);
}

pub fn sla_c(cpu: &mut Cpu) {
  let c = cpu.regs.get_c();
  let carry = c & 0x80 != 0;
  let result = c << 1;
  cpu.regs.set_flags_znhc(result == 0, false, false, carry);
  cpu.regs.set_c(result);
}

pub fn sla_d(cpu: &mut Cpu) {
  let d = cpu.regs.get_d();
  let carry = d & 0x80 != 0;
  let result = d << 1;
  cpu.regs.set_flags_znhc(result == 0, false, false, carry);
  cpu.regs.set_d(result);
}

pub fn sla_e(cpu: &mut Cpu) {
  let e = cpu.regs.get_e();
  let carry = e & 0x80 != 0;
  let result = e << 1;
  cpu.regs.set_flags_znhc(result == 0, false, false, carry);
  cpu.regs.set_e(result);
}

pub fn sla_h(cpu: &mut Cpu) {
  let h = cpu.regs.get_h();
  let carry = h & 0x80 != 0;
  let result = h << 1;
  cpu.regs.set_flags_znhc(result == 0, false, false, carry);
  cpu.regs.set_h(result);
}

pub fn sla_l(cpu: &mut Cpu) {
  let l = cpu.regs.get_l();
  let carry = l & 0x80 != 0;
  let result = l << 1;
  cpu.regs.set_flags_znhc(result == 0, false, false, carry);
  cpu.regs.set_l(result);
}

pub fn sla_mhl(cpu: &mut Cpu) {
  let hl = cpu.regs.get_hl();
  let value = cpu.read(hl);
  let carry = value & 0x80 != 0;
  let result = value << 1;
  cpu.regs.set_flags_znhc(result == 0, false, false, carry);
  cpu.write(hl, result);
}

pub fn sla_a(cpu: &mut Cpu) {
  let a = cpu.regs.get_a();
  let carry = a & 0x80 != 0;
  let result = a << 1;
  cpu.regs.set_flags_znhc(result == 0, false, false, carry);
  cpu.regs.set_a(result);
}

pub fn sra_b(cpu: &mut Cpu) {
  let b = cpu.regs.get_b();
  let carry = b & 0x01 != 0;
  let result = (b & 0x80) | (b >> 1);
  cpu.regs.set_flags_znhc(result == 0, false, false, carry);
  cpu.regs.set_b(result);
}

pub fn sra_c(cpu: &mut Cpu) {
  let c = cpu.regs.get_c();
  let carry = c & 0x01 != 0;
  let result = (c & 0x80) | (c >> 1);
  cpu.regs.set_flags_znhc(result == 0, false, false, carry);
  cpu.regs.set_c(result);
}

pub fn sra_d(cpu: &mut Cpu) {
  let d = cpu.regs.get_d();
  let carry = d & 0x01 != 0;
  let result = (d & 0x80) | (d >> 1);
  cpu.regs.set_flags_znhc(result == 0, false, false, carry);
  cpu.regs.set_d(result);
}

pub fn sra_e(cpu: &mut Cpu) {
  let e = cpu.regs.get_e();
  let carry = e & 0x01 != 0;
  let result = (e & 0x80) | (e >> 1);
  cpu.regs.set_flags_znhc(result == 0, false, false, carry);
  cpu.regs.set_e(result);
}

pub fn sra_h(cpu: &mut Cpu) {
  let h = cpu.regs.get_h();
  let carry = h & 0x01 != 0;
  let result = (h & 0x80) | (h >> 1);
  cpu.regs.set_flags_znhc(result == 0, false, false, carry);
  cpu.regs.set_h(result);
}

pub fn sra_l(cpu: &mut Cpu) {
  let l = cpu.regs.get_l();
  let carry = l & 0x01 != 0;
  let result = (l & 0x80) | (l >> 1);
  cpu.regs.set_flags_znhc(result == 0, false, false, carry);
  cpu.regs.set_l(result);
}

pub fn sra_mhl(cpu: &mut Cpu) {
  let hl = cpu.regs.get_hl();
  let value = cpu.read(hl);
  let carry = value & 0x01 != 0;
  let result = (value & 0x80) | (value >> 1);
  cpu.regs.set_flags_znhc(result == 0, false, false, carry);
  cpu.write(hl, result);
}

pub fn sra_a(cpu: &mut Cpu) {
  let a = cpu.regs.get_a();
  let carry = a & 0x01 != 0;
  let result = (a & 0x80) | (a >> 1);
  cpu.regs.set_flags_znhc(result == 0, false, false, carry);
  cpu.regs.set_a(result);
}

pub fn swap_b(cpu: &mut Cpu) {
  let b = cpu.regs.get_b();
  let result = (b << 4) | (b >> 4);
  cpu.regs.set_flags_znhc(result == 0, false, false, false);
  cpu.regs.set_b(result);
}

pub fn swap_c(cpu: &mut Cpu) {
  let c = cpu.regs.get_c();
  let result = (c << 4) | (c >> 4);
  cpu.regs.set_flags_znhc(result == 0, false, false, false);
  cpu.regs.set_c(result);
}

pub fn swap_d(cpu: &mut Cpu) {
  let d = cpu.regs.get_d();
  let result = (d << 4) | (d >> 4);
  cpu.regs.set_flags_znhc(result == 0, false, false, false);
  cpu.regs.set_d(result);
}

pub fn swap_e(cpu: &mut Cpu) {
  let e = cpu.regs.get_e();
  let result = (e << 4) | (e >> 4);
  cpu.regs.set_flags_znhc(result == 0, false, false, false);
  cpu.regs.set_e(result);
}

pub fn swap_h(cpu: &mut Cpu) {
  let h = cpu.regs.get_h();
  let result = (h << 4) | (h >> 4);
  cpu.regs.set_flags_znhc(result == 0, false, false, false);
  cpu.regs.set_h(result);
}

pub fn swap_l(cpu: &mut Cpu) {
  let l = cpu.regs.get_l();
  let result = (l << 4) | (l >> 4);
  cpu.regs.set_flags_znhc(result == 0, false, false, false);
  cpu.regs.set_l(result);
}

pub fn swap_mhl(cpu: &mut Cpu) {
  let hl = cpu.regs.get_hl();
  let value = cpu.read(hl);
  let result = (value << 4) | (value >> 4);
  cpu.regs.set_flags_znhc(result == 0, false, false, false);
  cpu.write(hl, result);
}

pub fn swap_a(cpu: &mut Cpu) {
  let a = cpu.regs.get_a();
  let result = (a << 4) | (a >> 4);
  cpu.regs.set_flags_znhc(result == 0, false, false, false);
  cpu.regs.set_a(result);
}

pub fn srl_b(cpu: &mut Cpu) {
  let b = cpu.regs.get_b();
  let carry = b & 0x01 != 0;
  let result = b >> 1;
  cpu.regs.set_flags_znhc(result == 0, false, false, carry);
  cpu.regs.set_b(result);
}

pub fn srl_c(cpu: &mut Cpu) {
  let c = cpu.regs.get_c();
  let carry = c & 0x01 != 0;
  let result = c >> 1;
  cpu.regs.set_flags_znhc(result == 0, false, false, carry);
  cpu.regs.set_c(result);
}

pub fn srl_d(cpu: &mut Cpu) {
  let d = cpu.regs.get_d();
  let carry = d & 0x01 != 0;
  let result = d >> 1;
  cpu.regs.set_flags_znhc(result == 0, false, false, carry);
  cpu.regs.set_d(result);
}

pub fn srl_e(cpu: &mut Cpu) {
  let e = cpu.regs.get_e();
  let carry = e & 0x01 != 0;
  let result = e >> 1;
  cpu.regs.set_flags_znhc(result == 0, false, false, carry);
  cpu.regs.set_e(result);
}

pub fn srl_h(cpu: &mut Cpu) {
  let h = cpu.regs.get_h();
  let carry = h & 0x01 != 0;
  let result = h >> 1;
  cpu.regs.set_flags_znhc(result == 0, false, false, carry);
  cpu.regs.set_h(result);
}

pub fn srl_l(cpu: &mut Cpu) {
  let l = cpu.regs.get_l();
  let carry = l & 0x01 != 0;
  let result = l >> 1;
  cpu.regs.set_flags_znhc(result == 0, false, false, carry);
  cpu.regs.set_l(result);
}

pub fn srl_mhl(cpu: &mut Cpu) {
  let hl = cpu.regs.get_hl();
  let value = cpu.read(hl);
  let carry = value & 0x01 != 0;
  let result = value >> 1;
  cpu.regs.set_flags_znhc(result == 0, false, false, carry);
  cpu.write(hl, result);
}

pub fn srl_a(cpu: &mut Cpu) {
  let a = cpu.regs.get_a();
  let carry = a & 0x01 != 0;
  let result = a >> 1;
  cpu.regs.set_flags_znhc(result == 0, false, false, carry);
  cpu.regs.set_a(result);
}

pub fn bit_0_b(cpu: &mut Cpu) {
  let b = cpu.regs.get_b();
  let result = b & (1 << 0) == 0;
  cpu
    .regs
    .set_flags_znhc(result, false, true, cpu.regs.get_flags_c());
}

pub fn bit_0_c(cpu: &mut Cpu) {
  let c = cpu.regs.get_c();
  let result = c & (1 << 0) == 0;
  cpu
    .regs
    .set_flags_znhc(result, false, true, cpu.regs.get_flags_c());
}

pub fn bit_0_d(cpu: &mut Cpu) {
  let d = cpu.regs.get_d();
  let result = d & (1 << 0) == 0;
  cpu
    .regs
    .set_flags_znhc(result, false, true, cpu.regs.get_flags_c());
}

pub fn bit_0_e(cpu: &mut Cpu) {
  let e = cpu.regs.get_e();
  let result = e & (1 << 0) == 0;
  cpu
    .regs
    .set_flags_znhc(result, false, true, cpu.regs.get_flags_c());
}

pub fn bit_0_h(cpu: &mut Cpu) {
  let h = cpu.regs.get_h();
  let result = h & (1 << 0) == 0;
  cpu
    .regs
    .set_flags_znhc(result, false, true, cpu.regs.get_flags_c());
}

pub fn bit_0_l(cpu: &mut Cpu) {
  let l = cpu.regs.get_l();
  let result = l & (1 << 0) == 0;
  cpu
    .regs
    .set_flags_znhc(result, false, true, cpu.regs.get_flags_c());
}

pub fn bit_0_mhl(cpu: &mut Cpu) {
  let hl = cpu.regs.get_hl();
  let value = cpu.read(hl);
  let result = value & (1 << 0) == 0;
  cpu
    .regs
    .set_flags_znhc(result, false, true, cpu.regs.get_flags_c());
}

pub fn bit_0_a(cpu: &mut Cpu) {
  let a = cpu.regs.get_a();
  let result = a & (1 << 0) == 0;
  cpu
    .regs
    .set_flags_znhc(result, false, true, cpu.regs.get_flags_c());
}

pub fn bit_1_b(cpu: &mut Cpu) {
  let b = cpu.regs.get_b();
  let result = b & (1 << 1) == 0;
  cpu
    .regs
    .set_flags_znhc(result, false, true, cpu.regs.get_flags_c());
}

pub fn bit_1_c(cpu: &mut Cpu) {
  let c = cpu.regs.get_c();
  let result = c & (1 << 1) == 0;
  cpu
    .regs
    .set_flags_znhc(result, false, true, cpu.regs.get_flags_c());
}

pub fn bit_1_d(cpu: &mut Cpu) {
  let d = cpu.regs.get_d();
  let result = d & (1 << 1) == 0;
  cpu
    .regs
    .set_flags_znhc(result, false, true, cpu.regs.get_flags_c());
}

pub fn bit_1_e(cpu: &mut Cpu) {
  let e = cpu.regs.get_e();
  let result = e & (1 << 1) == 0;
  cpu
    .regs
    .set_flags_znhc(result, false, true, cpu.regs.get_flags_c());
}

pub fn bit_1_h(cpu: &mut Cpu) {
  let h = cpu.regs.get_h();
  let result = h & (1 << 1) == 0;
  cpu
    .regs
    .set_flags_znhc(result, false, true, cpu.regs.get_flags_c());
}

pub fn bit_1_l(cpu: &mut Cpu) {
  let l = cpu.regs.get_l();
  let result = l & (1 << 1) == 0;
  cpu
    .regs
    .set_flags_znhc(result, false, true, cpu.regs.get_flags_c());
}

pub fn bit_1_mhl(cpu: &mut Cpu) {
  let hl = cpu.regs.get_hl();
  let value = cpu.read(hl);
  let result = value & (1 << 1) == 0;
  cpu
    .regs
    .set_flags_znhc(result, false, true, cpu.regs.get_flags_c());
}

pub fn bit_1_a(cpu: &mut Cpu) {
  let a = cpu.regs.get_a();
  let result = a & (1 << 1) == 0;
  cpu
    .regs
    .set_flags_znhc(result, false, true, cpu.regs.get_flags_c());
}

pub fn bit_2_b(cpu: &mut Cpu) {
  let b = cpu.regs.get_b();
  let result = b & (1 << 2) == 0;
  cpu
    .regs
    .set_flags_znhc(result, false, true, cpu.regs.get_flags_c());
}

pub fn bit_2_c(cpu: &mut Cpu) {
  let c = cpu.regs.get_c();
  let result = c & (1 << 2) == 0;
  cpu
    .regs
    .set_flags_znhc(result, false, true, cpu.regs.get_flags_c());
}

pub fn bit_2_d(cpu: &mut Cpu) {
  let d = cpu.regs.get_d();
  let result = d & (1 << 2) == 0;
  cpu
    .regs
    .set_flags_znhc(result, false, true, cpu.regs.get_flags_c());
}

pub fn bit_2_e(cpu: &mut Cpu) {
  let e = cpu.regs.get_e();
  let result = e & (1 << 2) == 0;
  cpu
    .regs
    .set_flags_znhc(result, false, true, cpu.regs.get_flags_c());
}

pub fn bit_2_h(cpu: &mut Cpu) {
  let h = cpu.regs.get_h();
  let result = h & (1 << 2) == 0;
  cpu
    .regs
    .set_flags_znhc(result, false, true, cpu.regs.get_flags_c());
}

pub fn bit_2_l(cpu: &mut Cpu) {
  let l = cpu.regs.get_l();
  let result = l & (1 << 2) == 0;
  cpu
    .regs
    .set_flags_znhc(result, false, true, cpu.regs.get_flags_c());
}

pub fn bit_2_mhl(cpu: &mut Cpu) {
  let hl = cpu.regs.get_hl();
  let value = cpu.read(hl);
  let result = value & (1 << 2) == 0;
  cpu
    .regs
    .set_flags_znhc(result, false, true, cpu.regs.get_flags_c());
}

pub fn bit_2_a(cpu: &mut Cpu) {
  let a = cpu.regs.get_a();
  let result = a & (1 << 2) == 0;
  cpu
    .regs
    .set_flags_znhc(result, false, true, cpu.regs.get_flags_c());
}

pub fn bit_3_b(cpu: &mut Cpu) {
  let b = cpu.regs.get_b();
  let result = b & (1 << 3) == 0;
  cpu
    .regs
    .set_flags_znhc(result, false, true, cpu.regs.get_flags_c());
}

pub fn bit_3_c(cpu: &mut Cpu) {
  let c = cpu.regs.get_c();
  let result = c & (1 << 3) == 0;
  cpu
    .regs
    .set_flags_znhc(result, false, true, cpu.regs.get_flags_c());
}

pub fn bit_3_d(cpu: &mut Cpu) {
  let d = cpu.regs.get_d();
  let result = d & (1 << 3) == 0;
  cpu
    .regs
    .set_flags_znhc(result, false, true, cpu.regs.get_flags_c());
}

pub fn bit_3_e(cpu: &mut Cpu) {
  let e = cpu.regs.get_e();
  let result = e & (1 << 3) == 0;
  cpu
    .regs
    .set_flags_znhc(result, false, true, cpu.regs.get_flags_c());
}

pub fn bit_3_h(cpu: &mut Cpu) {
  let h = cpu.regs.get_h();
  let result = h & (1 << 3) == 0;
  cpu
    .regs
    .set_flags_znhc(result, false, true, cpu.regs.get_flags_c());
}

pub fn bit_3_l(cpu: &mut Cpu) {
  let l = cpu.regs.get_l();
  let result = l & (1 << 3) == 0;
  cpu
    .regs
    .set_flags_znhc(result, false, true, cpu.regs.get_flags_c());
}

pub fn bit_3_mhl(cpu: &mut Cpu) {
  let hl = cpu.regs.get_hl();
  let value = cpu.read(hl);
  let result = value & (1 << 3) == 0;
  cpu
    .regs
    .set_flags_znhc(result, false, true, cpu.regs.get_flags_c());
}

pub fn bit_3_a(cpu: &mut Cpu) {
  let a = cpu.regs.get_a();
  let result = a & (1 << 3) == 0;
  cpu
    .regs
    .set_flags_znhc(result, false, true, cpu.regs.get_flags_c());
}

pub fn bit_4_b(cpu: &mut Cpu) {
  let b = cpu.regs.get_b();
  let result = b & (1 << 4) == 0;
  cpu
    .regs
    .set_flags_znhc(result, false, true, cpu.regs.get_flags_c());
}

pub fn bit_4_c(cpu: &mut Cpu) {
  let c = cpu.regs.get_c();
  let result = c & (1 << 4) == 0;
  cpu
    .regs
    .set_flags_znhc(result, false, true, cpu.regs.get_flags_c());
}

pub fn bit_4_d(cpu: &mut Cpu) {
  let d = cpu.regs.get_d();
  let result = d & (1 << 4) == 0;
  cpu
    .regs
    .set_flags_znhc(result, false, true, cpu.regs.get_flags_c());
}

pub fn bit_4_e(cpu: &mut Cpu) {
  let e = cpu.regs.get_e();
  let result = e & (1 << 4) == 0;
  cpu
    .regs
    .set_flags_znhc(result, false, true, cpu.regs.get_flags_c());
}

pub fn bit_4_h(cpu: &mut Cpu) {
  let h = cpu.regs.get_h();
  let result = h & (1 << 4) == 0;
  cpu
    .regs
    .set_flags_znhc(result, false, true, cpu.regs.get_flags_c());
}

pub fn bit_4_l(cpu: &mut Cpu) {
  let l = cpu.regs.get_l();
  let result = l & (1 << 4) == 0;
  cpu
    .regs
    .set_flags_znhc(result, false, true, cpu.regs.get_flags_c());
}

pub fn bit_4_mhl(cpu: &mut Cpu) {
  let hl = cpu.regs.get_hl();
  let value = cpu.read(hl);
  let result = value & (1 << 4) == 0;
  cpu
    .regs
    .set_flags_znhc(result, false, true, cpu.regs.get_flags_c());
}

pub fn bit_4_a(cpu: &mut Cpu) {
  let a = cpu.regs.get_a();
  let result = a & (1 << 4) == 0;
  cpu
    .regs
    .set_flags_znhc(result, false, true, cpu.regs.get_flags_c());
}

pub fn bit_5_b(cpu: &mut Cpu) {
  let b = cpu.regs.get_b();
  let result = b & (1 << 5) == 0;
  cpu
    .regs
    .set_flags_znhc(result, false, true, cpu.regs.get_flags_c());
}

pub fn bit_5_c(cpu: &mut Cpu) {
  let c = cpu.regs.get_c();
  let result = c & (1 << 5) == 0;
  cpu
    .regs
    .set_flags_znhc(result, false, true, cpu.regs.get_flags_c());
}

pub fn bit_5_d(cpu: &mut Cpu) {
  let d = cpu.regs.get_d();
  let result = d & (1 << 5) == 0;
  cpu
    .regs
    .set_flags_znhc(result, false, true, cpu.regs.get_flags_c());
}

pub fn bit_5_e(cpu: &mut Cpu) {
  let e = cpu.regs.get_e();
  let result = e & (1 << 5) == 0;
  cpu
    .regs
    .set_flags_znhc(result, false, true, cpu.regs.get_flags_c());
}

pub fn bit_5_h(cpu: &mut Cpu) {
  let h = cpu.regs.get_h();
  let result = h & (1 << 5) == 0;
  cpu
    .regs
    .set_flags_znhc(result, false, true, cpu.regs.get_flags_c());
}

pub fn bit_5_l(cpu: &mut Cpu) {
  let l = cpu.regs.get_l();
  let result = l & (1 << 5) == 0;
  cpu
    .regs
    .set_flags_znhc(result, false, true, cpu.regs.get_flags_c());
}

pub fn bit_5_mhl(cpu: &mut Cpu) {
  let hl = cpu.regs.get_hl();
  let value = cpu.read(hl);
  let result = value & (1 << 5) == 0;
  cpu
    .regs
    .set_flags_znhc(result, false, true, cpu.regs.get_flags_c());
}

pub fn bit_5_a(cpu: &mut Cpu) {
  let a = cpu.regs.get_a();
  let result = a & (1 << 5) == 0;
  cpu
    .regs
    .set_flags_znhc(result, false, true, cpu.regs.get_flags_c());
}

pub fn bit_6_b(cpu: &mut Cpu) {
  let b = cpu.regs.get_b();
  let result = b & (1 << 6) == 0;
  cpu
    .regs
    .set_flags_znhc(result, false, true, cpu.regs.get_flags_c());
}

pub fn bit_6_c(cpu: &mut Cpu) {
  let c = cpu.regs.get_c();
  let result = c & (1 << 6) == 0;
  cpu
    .regs
    .set_flags_znhc(result, false, true, cpu.regs.get_flags_c());
}

pub fn bit_6_d(cpu: &mut Cpu) {
  let d = cpu.regs.get_d();
  let result = d & (1 << 6) == 0;
  cpu
    .regs
    .set_flags_znhc(result, false, true, cpu.regs.get_flags_c());
}

pub fn bit_6_e(cpu: &mut Cpu) {
  let e = cpu.regs.get_e();
  let result = e & (1 << 6) == 0;
  cpu
    .regs
    .set_flags_znhc(result, false, true, cpu.regs.get_flags_c());
}

pub fn bit_6_h(cpu: &mut Cpu) {
  let h = cpu.regs.get_h();
  let result = h & (1 << 6) == 0;
  cpu
    .regs
    .set_flags_znhc(result, false, true, cpu.regs.get_flags_c());
}

pub fn bit_6_l(cpu: &mut Cpu) {
  let l = cpu.regs.get_l();
  let result = l & (1 << 6) == 0;
  cpu
    .regs
    .set_flags_znhc(result, false, true, cpu.regs.get_flags_c());
}

pub fn bit_6_mhl(cpu: &mut Cpu) {
  let hl = cpu.regs.get_hl();
  let value = cpu.read(hl);
  let result = value & (1 << 6) == 0;
  cpu
    .regs
    .set_flags_znhc(result, false, true, cpu.regs.get_flags_c());
}

pub fn bit_6_a(cpu: &mut Cpu) {
  let a = cpu.regs.get_a();
  let result = a & (1 << 6) == 0;
  cpu
    .regs
    .set_flags_znhc(result, false, true, cpu.regs.get_flags_c());
}

pub fn bit_7_b(cpu: &mut Cpu) {
  let b = cpu.regs.get_b();
  let result = b & (1 << 7) == 0;
  cpu
    .regs
    .set_flags_znhc(result, false, true, cpu.regs.get_flags_c());
}

pub fn bit_7_c(cpu: &mut Cpu) {
  let c = cpu.regs.get_c();
  let result = c & (1 << 7) == 0;
  cpu
    .regs
    .set_flags_znhc(result, false, true, cpu.regs.get_flags_c());
}

pub fn bit_7_d(cpu: &mut Cpu) {
  let d = cpu.regs.get_d();
  let result = d & (1 << 7) == 0;
  cpu
    .regs
    .set_flags_znhc(result, false, true, cpu.regs.get_flags_c());
}

pub fn bit_7_e(cpu: &mut Cpu) {
  let e = cpu.regs.get_e();
  let result = e & (1 << 7) == 0;
  cpu
    .regs
    .set_flags_znhc(result, false, true, cpu.regs.get_flags_c());
}

pub fn bit_7_h(cpu: &mut Cpu) {
  let h = cpu.regs.get_h();
  let result = h & (1 << 7) == 0;
  cpu
    .regs
    .set_flags_znhc(result, false, true, cpu.regs.get_flags_c());
}

pub fn bit_7_l(cpu: &mut Cpu) {
  let l = cpu.regs.get_l();
  let result = l & (1 << 7) == 0;
  cpu
    .regs
    .set_flags_znhc(result, false, true, cpu.regs.get_flags_c());
}

pub fn bit_7_mhl(cpu: &mut Cpu) {
  let hl = cpu.regs.get_hl();
  let value = cpu.read(hl);
  let result = value & (1 << 7) == 0;
  cpu
    .regs
    .set_flags_znhc(result, false, true, cpu.regs.get_flags_c());
}

pub fn bit_7_a(cpu: &mut Cpu) {
  let a = cpu.regs.get_a();
  let result = a & (1 << 7) == 0;
  cpu
    .regs
    .set_flags_znhc(result, false, true, cpu.regs.get_flags_c());
}

pub fn res_0_b(cpu: &mut Cpu) {
  let value = cpu.regs.get_b();
  cpu.regs.set_b(value & !(1 << 0));
}

pub fn res_0_c(cpu: &mut Cpu) {
  let value = cpu.regs.get_c();
  cpu.regs.set_c(value & !(1 << 0));
}

pub fn res_0_d(cpu: &mut Cpu) {
  let value = cpu.regs.get_d();
  cpu.regs.set_d(value & !(1 << 0));
}

pub fn res_0_e(cpu: &mut Cpu) {
  let value = cpu.regs.get_e();
  cpu.regs.set_e(value & !(1 << 0));
}

pub fn res_0_h(cpu: &mut Cpu) {
  let value = cpu.regs.get_h();
  cpu.regs.set_h(value & !(1 << 0));
}

pub fn res_0_l(cpu: &mut Cpu) {
  let value = cpu.regs.get_l();
  cpu.regs.set_l(value & !(1 << 0));
}

pub fn res_0_mhl(cpu: &mut Cpu) {
  let hl = cpu.regs.get_hl();
  let value = cpu.read(hl);
  cpu.write(hl, value & !(1 << 0));
}

pub fn res_0_a(cpu: &mut Cpu) {
  let value = cpu.regs.get_a();
  cpu.regs.set_a(value & !(1 << 0));
}

pub fn res_1_b(cpu: &mut Cpu) {
  let value = cpu.regs.get_b();
  cpu.regs.set_b(value & !(1 << 1));
}

pub fn res_1_c(cpu: &mut Cpu) {
  let value = cpu.regs.get_c();
  cpu.regs.set_c(value & !(1 << 1));
}

pub fn res_1_d(cpu: &mut Cpu) {
  let value = cpu.regs.get_d();
  cpu.regs.set_d(value & !(1 << 1));
}

pub fn res_1_e(cpu: &mut Cpu) {
  let value = cpu.regs.get_e();
  cpu.regs.set_e(value & !(1 << 1));
}

pub fn res_1_h(cpu: &mut Cpu) {
  let value = cpu.regs.get_h();
  cpu.regs.set_h(value & !(1 << 1));
}

pub fn res_1_l(cpu: &mut Cpu) {
  let value = cpu.regs.get_l();
  cpu.regs.set_l(value & !(1 << 1));
}

pub fn res_1_mhl(cpu: &mut Cpu) {
  let hl = cpu.regs.get_hl();
  let value = cpu.read(hl);
  cpu.write(hl, value & !(1 << 1));
}

pub fn res_1_a(cpu: &mut Cpu) {
  let value = cpu.regs.get_a();
  cpu.regs.set_a(value & !(1 << 1));
}

pub fn res_2_b(cpu: &mut Cpu) {
  let value = cpu.regs.get_b();
  cpu.regs.set_b(value & !(1 << 2));
}

pub fn res_2_c(cpu: &mut Cpu) {
  let value = cpu.regs.get_c();
  cpu.regs.set_c(value & !(1 << 2));
}

pub fn res_2_d(cpu: &mut Cpu) {
  let value = cpu.regs.get_d();
  cpu.regs.set_d(value & !(1 << 2));
}

pub fn res_2_e(cpu: &mut Cpu) {
  let value = cpu.regs.get_e();
  cpu.regs.set_e(value & !(1 << 2));
}

pub fn res_2_h(cpu: &mut Cpu) {
  let value = cpu.regs.get_h();
  cpu.regs.set_h(value & !(1 << 2));
}

pub fn res_2_l(cpu: &mut Cpu) {
  let value = cpu.regs.get_l();
  cpu.regs.set_l(value & !(1 << 2));
}

pub fn res_2_mhl(cpu: &mut Cpu) {
  let hl = cpu.regs.get_hl();
  let value = cpu.read(hl);
  cpu.write(hl, value & !(1 << 2));
}

pub fn res_2_a(cpu: &mut Cpu) {
  let value = cpu.regs.get_a();
  cpu.regs.set_a(value & !(1 << 2));
}

pub fn res_3_b(cpu: &mut Cpu) {
  let value = cpu.regs.get_b();
  cpu.regs.set_b(value & !(1 << 3));
}

pub fn res_3_c(cpu: &mut Cpu) {
  let value = cpu.regs.get_c();
  cpu.regs.set_c(value & !(1 << 3));
}

pub fn res_3_d(cpu: &mut Cpu) {
  let value = cpu.regs.get_d();
  cpu.regs.set_d(value & !(1 << 3));
}

pub fn res_3_e(cpu: &mut Cpu) {
  let value = cpu.regs.get_e();
  cpu.regs.set_e(value & !(1 << 3));
}

pub fn res_3_h(cpu: &mut Cpu) {
  let value = cpu.regs.get_h();
  cpu.regs.set_h(value & !(1 << 3));
}

pub fn res_3_l(cpu: &mut Cpu) {
  let value = cpu.regs.get_l();
  cpu.regs.set_l(value & !(1 << 3));
}

pub fn res_3_mhl(cpu: &mut Cpu) {
  let hl = cpu.regs.get_hl();
  let value = cpu.read(hl);
  cpu.write(hl, value & !(1 << 3));
}

pub fn res_3_a(cpu: &mut Cpu) {
  let value = cpu.regs.get_a();
  cpu.regs.set_a(value & !(1 << 3));
}

pub fn res_4_b(cpu: &mut Cpu) {
  let value = cpu.regs.get_b();
  cpu.regs.set_b(value & !(1 << 4));
}

pub fn res_4_c(cpu: &mut Cpu) {
  let value = cpu.regs.get_c();
  cpu.regs.set_c(value & !(1 << 4));
}

pub fn res_4_d(cpu: &mut Cpu) {
  let value = cpu.regs.get_d();
  cpu.regs.set_d(value & !(1 << 4));
}

pub fn res_4_e(cpu: &mut Cpu) {
  let value = cpu.regs.get_e();
  cpu.regs.set_e(value & !(1 << 4));
}

pub fn res_4_h(cpu: &mut Cpu) {
  let value = cpu.regs.get_h();
  cpu.regs.set_h(value & !(1 << 4));
}

pub fn res_4_l(cpu: &mut Cpu) {
  let value = cpu.regs.get_l();
  cpu.regs.set_l(value & !(1 << 4));
}

pub fn res_4_mhl(cpu: &mut Cpu) {
  let hl = cpu.regs.get_hl();
  let value = cpu.read(hl);
  cpu.write(hl, value & !(1 << 4));
}

pub fn res_4_a(cpu: &mut Cpu) {
  let value = cpu.regs.get_a();
  cpu.regs.set_a(value & !(1 << 4));
}

pub fn res_5_b(cpu: &mut Cpu) {
  let value = cpu.regs.get_b();
  cpu.regs.set_b(value & !(1 << 5));
}

pub fn res_5_c(cpu: &mut Cpu) {
  let value = cpu.regs.get_c();
  cpu.regs.set_c(value & !(1 << 5));
}

pub fn res_5_d(cpu: &mut Cpu) {
  let value = cpu.regs.get_d();
  cpu.regs.set_d(value & !(1 << 5));
}

pub fn res_5_e(cpu: &mut Cpu) {
  let value = cpu.regs.get_e();
  cpu.regs.set_e(value & !(1 << 5));
}

pub fn res_5_h(cpu: &mut Cpu) {
  let value = cpu.regs.get_h();
  cpu.regs.set_h(value & !(1 << 5));
}

pub fn res_5_l(cpu: &mut Cpu) {
  let value = cpu.regs.get_l();
  cpu.regs.set_l(value & !(1 << 5));
}

pub fn res_5_mhl(cpu: &mut Cpu) {
  let hl = cpu.regs.get_hl();
  let value = cpu.read(hl);
  cpu.write(hl, value & !(1 << 5));
}

pub fn res_5_a(cpu: &mut Cpu) {
  let value = cpu.regs.get_a();
  cpu.regs.set_a(value & !(1 << 5));
}

pub fn res_6_b(cpu: &mut Cpu) {
  let value = cpu.regs.get_b();
  cpu.regs.set_b(value & !(1 << 6));
}

pub fn res_6_c(cpu: &mut Cpu) {
  let value = cpu.regs.get_c();
  cpu.regs.set_c(value & !(1 << 6));
}

pub fn res_6_d(cpu: &mut Cpu) {
  let value = cpu.regs.get_d();
  cpu.regs.set_d(value & !(1 << 6));
}

pub fn res_6_e(cpu: &mut Cpu) {
  let value = cpu.regs.get_e();
  cpu.regs.set_e(value & !(1 << 6));
}

pub fn res_6_h(cpu: &mut Cpu) {
  let value = cpu.regs.get_h();
  cpu.regs.set_h(value & !(1 << 6));
}

pub fn res_6_l(cpu: &mut Cpu) {
  let value = cpu.regs.get_l();
  cpu.regs.set_l(value & !(1 << 6));
}

pub fn res_6_mhl(cpu: &mut Cpu) {
  let hl = cpu.regs.get_hl();
  let value = cpu.read(hl);
  cpu.write(hl, value & !(1 << 6));
}

pub fn res_6_a(cpu: &mut Cpu) {
  let value = cpu.regs.get_a();
  cpu.regs.set_a(value & !(1 << 6));
}

pub fn res_7_b(cpu: &mut Cpu) {
  let value = cpu.regs.get_b();
  cpu.regs.set_b(value & !(1 << 7));
}

pub fn res_7_c(cpu: &mut Cpu) {
  let value = cpu.regs.get_c();
  cpu.regs.set_c(value & !(1 << 7));
}

pub fn res_7_d(cpu: &mut Cpu) {
  let value = cpu.regs.get_d();
  cpu.regs.set_d(value & !(1 << 7));
}

pub fn res_7_e(cpu: &mut Cpu) {
  let value = cpu.regs.get_e();
  cpu.regs.set_e(value & !(1 << 7));
}

pub fn res_7_h(cpu: &mut Cpu) {
  let value = cpu.regs.get_h();
  cpu.regs.set_h(value & !(1 << 7));
}

pub fn res_7_l(cpu: &mut Cpu) {
  let value = cpu.regs.get_l();
  cpu.regs.set_l(value & !(1 << 7));
}

pub fn res_7_mhl(cpu: &mut Cpu) {
  let hl = cpu.regs.get_hl();
  let value = cpu.read(hl);
  cpu.write(hl, value & !(1 << 7));
}

pub fn res_7_a(cpu: &mut Cpu) {
  let value = cpu.regs.get_a();
  cpu.regs.set_a(value & !(1 << 7));
}

pub fn set_0_b(cpu: &mut Cpu) {
  let value = cpu.regs.get_b();
  cpu.regs.set_b(value | (1 << 0));
}

pub fn set_0_c(cpu: &mut Cpu) {
  let value = cpu.regs.get_c();
  cpu.regs.set_c(value | (1 << 0));
}

pub fn set_0_d(cpu: &mut Cpu) {
  let value = cpu.regs.get_d();
  cpu.regs.set_d(value | (1 << 0));
}

pub fn set_0_e(cpu: &mut Cpu) {
  let value = cpu.regs.get_e();
  cpu.regs.set_e(value | (1 << 0));
}

pub fn set_0_h(cpu: &mut Cpu) {
  let value = cpu.regs.get_h();
  cpu.regs.set_h(value | (1 << 0));
}

pub fn set_0_l(cpu: &mut Cpu) {
  let value = cpu.regs.get_l();
  cpu.regs.set_l(value | (1 << 0));
}

pub fn set_0_mhl(cpu: &mut Cpu) {
  let hl = cpu.regs.get_hl();
  let value = cpu.read(hl);
  cpu.write(hl, value | (1 << 0));
}

pub fn set_0_a(cpu: &mut Cpu) {
  let value = cpu.regs.get_a();
  cpu.regs.set_a(value | (1 << 0));
}

pub fn set_1_b(cpu: &mut Cpu) {
  let value = cpu.regs.get_b();
  cpu.regs.set_b(value | (1 << 1));
}

pub fn set_1_c(cpu: &mut Cpu) {
  let value = cpu.regs.get_c();
  cpu.regs.set_c(value | (1 << 1));
}

pub fn set_1_d(cpu: &mut Cpu) {
  let value = cpu.regs.get_d();
  cpu.regs.set_d(value | (1 << 1));
}

pub fn set_1_e(cpu: &mut Cpu) {
  let value = cpu.regs.get_e();
  cpu.regs.set_e(value | (1 << 1));
}

pub fn set_1_h(cpu: &mut Cpu) {
  let value = cpu.regs.get_h();
  cpu.regs.set_h(value | (1 << 1));
}

pub fn set_1_l(cpu: &mut Cpu) {
  let value = cpu.regs.get_l();
  cpu.regs.set_l(value | (1 << 1));
}

pub fn set_1_mhl(cpu: &mut Cpu) {
  let hl = cpu.regs.get_hl();
  let value = cpu.read(hl);
  cpu.write(hl, value | (1 << 1));
}

pub fn set_1_a(cpu: &mut Cpu) {
  let value = cpu.regs.get_a();
  cpu.regs.set_a(value | (1 << 1));
}

pub fn set_2_b(cpu: &mut Cpu) {
  let value = cpu.regs.get_b();
  cpu.regs.set_b(value | (1 << 2));
}

pub fn set_2_c(cpu: &mut Cpu) {
  let value = cpu.regs.get_c();
  cpu.regs.set_c(value | (1 << 2));
}

pub fn set_2_d(cpu: &mut Cpu) {
  let value = cpu.regs.get_d();
  cpu.regs.set_d(value | (1 << 2));
}

pub fn set_2_e(cpu: &mut Cpu) {
  let value = cpu.regs.get_e();
  cpu.regs.set_e(value | (1 << 2));
}

pub fn set_2_h(cpu: &mut Cpu) {
  let value = cpu.regs.get_h();
  cpu.regs.set_h(value | (1 << 2));
}

pub fn set_2_l(cpu: &mut Cpu) {
  let value = cpu.regs.get_l();
  cpu.regs.set_l(value | (1 << 2));
}

pub fn set_2_mhl(cpu: &mut Cpu) {
  let hl = cpu.regs.get_hl();
  let value = cpu.read(hl);
  cpu.write(hl, value | (1 << 2));
}

pub fn set_2_a(cpu: &mut Cpu) {
  let value = cpu.regs.get_a();
  cpu.regs.set_a(value | (1 << 2));
}

pub fn set_3_b(cpu: &mut Cpu) {
  let value = cpu.regs.get_b();
  cpu.regs.set_b(value | (1 << 3));
}

pub fn set_3_c(cpu: &mut Cpu) {
  let value = cpu.regs.get_c();
  cpu.regs.set_c(value | (1 << 3));
}

pub fn set_3_d(cpu: &mut Cpu) {
  let value = cpu.regs.get_d();
  cpu.regs.set_d(value | (1 << 3));
}

pub fn set_3_e(cpu: &mut Cpu) {
  let value = cpu.regs.get_e();
  cpu.regs.set_e(value | (1 << 3));
}

pub fn set_3_h(cpu: &mut Cpu) {
  let value = cpu.regs.get_h();
  cpu.regs.set_h(value | (1 << 3));
}

pub fn set_3_l(cpu: &mut Cpu) {
  let value = cpu.regs.get_l();
  cpu.regs.set_l(value | (1 << 3));
}

pub fn set_3_mhl(cpu: &mut Cpu) {
  let hl = cpu.regs.get_hl();
  let value = cpu.read(hl);
  cpu.write(hl, value | (1 << 3));
}

pub fn set_3_a(cpu: &mut Cpu) {
  let value = cpu.regs.get_a();
  cpu.regs.set_a(value | (1 << 3));
}

pub fn set_4_b(cpu: &mut Cpu) {
  let value = cpu.regs.get_b();
  cpu.regs.set_b(value | (1 << 4));
}

pub fn set_4_c(cpu: &mut Cpu) {
  let value = cpu.regs.get_c();
  cpu.regs.set_c(value | (1 << 4));
}

pub fn set_4_d(cpu: &mut Cpu) {
  let value = cpu.regs.get_d();
  cpu.regs.set_d(value | (1 << 4));
}

pub fn set_4_e(cpu: &mut Cpu) {
  let value = cpu.regs.get_e();
  cpu.regs.set_e(value | (1 << 4));
}

pub fn set_4_h(cpu: &mut Cpu) {
  let value = cpu.regs.get_h();
  cpu.regs.set_h(value | (1 << 4));
}

pub fn set_4_l(cpu: &mut Cpu) {
  let value = cpu.regs.get_l();
  cpu.regs.set_l(value | (1 << 4));
}

pub fn set_4_mhl(cpu: &mut Cpu) {
  let hl = cpu.regs.get_hl();
  let value = cpu.read(hl);
  cpu.write(hl, value | (1 << 4));
}

pub fn set_4_a(cpu: &mut Cpu) {
  let value = cpu.regs.get_a();
  cpu.regs.set_a(value | (1 << 4));
}

pub fn set_5_b(cpu: &mut Cpu) {
  let value = cpu.regs.get_b();
  cpu.regs.set_b(value | (1 << 5));
}

pub fn set_5_c(cpu: &mut Cpu) {
  let value = cpu.regs.get_c();
  cpu.regs.set_c(value | (1 << 5));
}

pub fn set_5_d(cpu: &mut Cpu) {
  let value = cpu.regs.get_d();
  cpu.regs.set_d(value | (1 << 5));
}

pub fn set_5_e(cpu: &mut Cpu) {
  let value = cpu.regs.get_e();
  cpu.regs.set_e(value | (1 << 5));
}

pub fn set_5_h(cpu: &mut Cpu) {
  let value = cpu.regs.get_h();
  cpu.regs.set_h(value | (1 << 5));
}

pub fn set_5_l(cpu: &mut Cpu) {
  let value = cpu.regs.get_l();
  cpu.regs.set_l(value | (1 << 5));
}

pub fn set_5_mhl(cpu: &mut Cpu) {
  let hl = cpu.regs.get_hl();
  let value = cpu.read(hl);
  cpu.write(hl, value | (1 << 5));
}

pub fn set_5_a(cpu: &mut Cpu) {
  let value = cpu.regs.get_a();
  cpu.regs.set_a(value | (1 << 5));
}

pub fn set_6_b(cpu: &mut Cpu) {
  let value = cpu.regs.get_b();
  cpu.regs.set_b(value | (1 << 6));
}

pub fn set_6_c(cpu: &mut Cpu) {
  let value = cpu.regs.get_c();
  cpu.regs.set_c(value | (1 << 6));
}

pub fn set_6_d(cpu: &mut Cpu) {
  let value = cpu.regs.get_d();
  cpu.regs.set_d(value | (1 << 6));
}

pub fn set_6_e(cpu: &mut Cpu) {
  let value = cpu.regs.get_e();
  cpu.regs.set_e(value | (1 << 6));
}

pub fn set_6_h(cpu: &mut Cpu) {
  let value = cpu.regs.get_h();
  cpu.regs.set_h(value | (1 << 6));
}

pub fn set_6_l(cpu: &mut Cpu) {
  let value = cpu.regs.get_l();
  cpu.regs.set_l(value | (1 << 6));
}

pub fn set_6_mhl(cpu: &mut Cpu) {
  let hl = cpu.regs.get_hl();
  let value = cpu.read(hl);
  cpu.write(hl, value | (1 << 6));
}

pub fn set_6_a(cpu: &mut Cpu) {
  let value = cpu.regs.get_a();
  cpu.regs.set_a(value | (1 << 6));
}

pub fn set_7_b(cpu: &mut Cpu) {
  let value = cpu.regs.get_b();
  cpu.regs.set_b(value | (1 << 7));
}

pub fn set_7_c(cpu: &mut Cpu) {
  let value = cpu.regs.get_c();
  cpu.regs.set_c(value | (1 << 7));
}

pub fn set_7_d(cpu: &mut Cpu) {
  let value = cpu.regs.get_d();
  cpu.regs.set_d(value | (1 << 7));
}

pub fn set_7_e(cpu: &mut Cpu) {
  let value = cpu.regs.get_e();
  cpu.regs.set_e(value | (1 << 7));
}

pub fn set_7_h(cpu: &mut Cpu) {
  let value = cpu.regs.get_h();
  cpu.regs.set_h(value | (1 << 7));
}

pub fn set_7_l(cpu: &mut Cpu) {
  let value = cpu.regs.get_l();
  cpu.regs.set_l(value | (1 << 7));
}

pub fn set_7_mhl(cpu: &mut Cpu) {
  let hl = cpu.regs.get_hl();
  let value = cpu.read(hl);
  cpu.write(hl, value | (1 << 7));
}

pub fn set_7_a(cpu: &mut Cpu) {
  let value = cpu.regs.get_a();
  cpu.regs.set_a(value | (1 << 7));
}
