mod handler;

use super::{
  register::{Register16, Register8},
  CPU,
};
use handler::*;

struct Instruction {
  func: fn(&mut CPU),
  length: u8,
  t_cycles: u8,
  description: &'static str,
}

const INSTRUCTIONS: [Instruction; 256] = [
  // 0x00
  Instruction {
    func: nop,
    length: 1,
    t_cycles: 4,
    description: "NOP",
  },
  // 0x01
  Instruction {
    func: ld_bc_u16,
    length: 3,
    t_cycles: 12,
    description: "LD BC, u16",
  },
  // 0x02
  Instruction {
    func: ld_mbc_a,
    length: 1,
    t_cycles: 8,
    description: "LD [BC], A",
  },
  // 0x03
  Instruction {
    func: inc_bc,
    length: 1,
    t_cycles: 8,
    description: "INC BC",
  },
  // 0x04
  Instruction {
    func: inc_b,
    length: 1,
    t_cycles: 4,
    description: "INC B",
  },
  // 0x05
  Instruction {
    func: dec_b,
    length: 1,
    t_cycles: 4,
    description: "DEC B",
  },
  // 0x06
  Instruction {
    func: ld_b_u8,
    length: 2,
    t_cycles: 8,
    description: "LD B, u8",
  },
  // 0x07
  Instruction {
    func: rlca,
    length: 1,
    t_cycles: 4,
    description: "RLCA",
  },
  // 0x08
  Instruction {
    func: ld_mu16_sp,
    length: 3,
    t_cycles: 20,
    description: "LD [u16], SP",
  },
  // 0x09
  Instruction {
    func: add_hl_bc,
    length: 1,
    t_cycles: 8,
    description: "ADD HL, BC",
  },
  // 0x0A
  Instruction {
    func: ld_a_mbc,
    length: 1,
    t_cycles: 8,
    description: "LD A, [BC]",
  },
  // 0x0B
  Instruction {
    func: dec_bc,
    length: 1,
    t_cycles: 8,
    description: "LD A, [BC]",
  },
  // 0x0C
  Instruction {
    func: inc_c,
    length: 1,
    t_cycles: 4,
    description: "INC C",
  },
  // 0x0D
  Instruction {
    func: dec_c,
    length: 1,
    t_cycles: 4,
    description: "DEC C",
  },
  // 0x0E
  Instruction {
    func: ld_c_u8,
    length: 2,
    t_cycles: 8,
    description: "LD C, u8",
  },
  // 0x0F
  Instruction {
    func: rrca,
    length: 1,
    t_cycles: 4,
    description: "RRCA",
  },
  // 0x10
  Instruction {
    func: stop_u8,
    length: 2,
    t_cycles: 4,
    description: "STOP u8",
  },
  // 0x11
  Instruction {
    func: ld_de_u16,
    length: 3,
    t_cycles: 12,
    description: "LD DE, u16",
  },
  // 0x12
  Instruction {
    func: ld_mde_a,
    length: 1,
    t_cycles: 8,
    description: "LD [DE], A",
  },
  // 0x13
  Instruction {
    func: inc_de,
    length: 1,
    t_cycles: 8,
    description: "INC DE",
  },
  // 0x14
  Instruction {
    func: inc_d,
    length: 1,
    t_cycles: 4,
    description: "INC D",
  },
  // 0x15
  Instruction {
    func: dec_d,
    length: 1,
    t_cycles: 4,
    description: "DEC D",
  },
  // 0x16
  Instruction {
    func: ld_d_u8,
    length: 2,
    t_cycles: 8,
    description: "LD D, u8",
  },
  // 0x17
  Instruction {
    func: rla,
    length: 1,
    t_cycles: 4,
    description: "RLA",
  },
  // 0x18
  Instruction {
    func: jr_i8,
    length: 2,
    t_cycles: 12,
    description: "JR i8",
  },
  // 0x19
  Instruction {
    func: add_hl_de,
    length: 1,
    t_cycles: 8,
    description: "ADD HL, DE",
  },
  // 0x1A
  Instruction {
    func: ld_a_mde,
    length: 1,
    t_cycles: 8,
    description: "LD A, [DE]",
  },
  // 0x1B
  Instruction {
    func: dec_de,
    length: 1,
    t_cycles: 8,
    description: "DEC DE",
  },
  // 0x1C
  Instruction {
    func: inc_e,
    length: 1,
    t_cycles: 4,
    description: "INC E",
  },
  // 0x1D
  Instruction {
    func: dec_e,
    length: 1,
    t_cycles: 4,
    description: "DEC E",
  },
  // 0x1E
  Instruction {
    func: ld_e_u8,
    length: 2,
    t_cycles: 8,
    description: "LD E, u8",
  },
  // 0x1F
  Instruction {
    func: rra,
    length: 1,
    t_cycles: 4,
    description: "RRA",
  },
  // 0x20
  // TODO: fix t_cycles
  Instruction {
    func: jr_nz_i8,
    length: 2,
    t_cycles: 8,
    description: "JR NZ, i8",
  },
  // 0x21
  Instruction {
    func: ld_hl_u16,
    length: 3,
    t_cycles: 12,
    description: "LD HL, u16",
  },
  // 0x22
  Instruction {
    func: ld_mhli_a,
    length: 1,
    t_cycles: 8,
    description: "LD [HL+], A",
  },
  // 0x23
  Instruction {
    func: inc_hl,
    length: 1,
    t_cycles: 8,
    description: "INC HL",
  },
  // 0x24
  Instruction {
    func: inc_h,
    length: 1,
    t_cycles: 4,
    description: "INC H",
  },
  // 0x25
  Instruction {
    func: dec_h,
    length: 1,
    t_cycles: 4,
    description: "DEC H",
  },
  // 0x26
  Instruction {
    func: ld_h_u8,
    length: 2,
    t_cycles: 8,
    description: "LD H, u8",
  },
  // 0x27
  Instruction {
    func: daa,
    length: 1,
    t_cycles: 4,
    description: "DAA",
  },
  // 0x28
  // TODO: fix t_cycles
  Instruction {
    func: jr_z_i8,
    length: 2,
    t_cycles: 8,
    description: "JR Z, i8",
  },
  // 0x29
  Instruction {
    func: add_hl_hl,
    length: 1,
    t_cycles: 8,
    description: "ADD HL, HL",
  },
  // 0x2A
  Instruction {
    func: ld_a_mhli,
    length: 1,
    t_cycles: 8,
    description: "LDI A, [HL+]",
  },
  // 0x2B
  Instruction {
    func: dec_hl,
    length: 1,
    t_cycles: 8,
    description: "DEC HL",
  },
  // 0x2C
  Instruction {
    func: inc_l,
    length: 1,
    t_cycles: 4,
    description: "INC L",
  },
  // 0x2D
  Instruction {
    func: dec_l,
    length: 1,
    t_cycles: 4,
    description: "DEC L",
  },
  // 0x2E
  Instruction {
    func: ld_l_u8,
    length: 2,
    t_cycles: 8,
    description: "LD L, u8",
  },
  // 0x2F
  Instruction {
    func: cpl,
    length: 1,
    t_cycles: 4,
    description: "CPL",
  },
  // 0x30
  // TODO: fix t_cycles
  Instruction {
    func: jr_nc_i8,
    length: 2,
    t_cycles: 8,
    description: "JR NC, i8",
  },
  // 0x31
  Instruction {
    func: ld_sp_u16,
    length: 3,
    t_cycles: 12,
    description: "LD SP, u16",
  },
  // 0x32
  Instruction {
    func: ld_mhld_a,
    length: 1,
    t_cycles: 8,
    description: "LD [HL-], A",
  },
  // 0x33
  Instruction {
    func: inc_sp,
    length: 1,
    t_cycles: 8,
    description: "INC SP",
  },
  // 0x34
  Instruction {
    func: inc_mhl,
    length: 1,
    t_cycles: 12,
    description: "INC [HL]",
  },
  // 0x35
  Instruction {
    func: dec_mhl,
    length: 1,
    t_cycles: 12,
    description: "DEC [HL]",
  },
  // 0x36
  Instruction {
    func: ld_mhl_u8,
    length: 2,
    t_cycles: 12,
    description: "LD [HL], u8",
  },
  // 0x37
  Instruction {
    func: scf,
    length: 1,
    t_cycles: 4,
    description: "SCF",
  },
  // 0x38
  // TODO: fix t_cycles
  Instruction {
    func: jr_c_i8,
    length: 2,
    t_cycles: 8,
    description: "JR C, i8",
  },
  // 0x39
  Instruction {
    func: add_hl_sp,
    length: 1,
    t_cycles: 8,
    description: "ADD HL, SP",
  },
  // 0x3A
  Instruction {
    func: ld_a_mhld,
    length: 1,
    t_cycles: 8,
    description: "LD A, [HL-]",
  },
  // 0x3B
  Instruction {
    func: dec_sp,
    length: 1,
    t_cycles: 8,
    description: "DEC SP",
  },
  // 0x3C
  Instruction {
    func: inc_a,
    length: 1,
    t_cycles: 4,
    description: "INC A",
  },
  // 0x3D
  Instruction {
    func: dec_a,
    length: 1,
    t_cycles: 4,
    description: "DEC A",
  },
  // 0x3E
  Instruction {
    func: ld_a_u8,
    length: 2,
    t_cycles: 8,
    description: "LD A, u8",
  },
  // 0x3F
  Instruction {
    func: ccf,
    length: 1,
    t_cycles: 4,
    description: "CCF",
  },
  // 0x40
  Instruction {
    func: ld_b_b,
    length: 1,
    t_cycles: 4,
    description: "LD B, B",
  },
  // 0x41
  Instruction {
    func: ld_b_c,
    length: 1,
    t_cycles: 4,
    description: "LD B, C",
  },
  // 0x42
  Instruction {
    func: ld_b_d,
    length: 1,
    t_cycles: 4,
    description: "LD B, D",
  },
  // 0x43
  Instruction {
    func: ld_b_e,
    length: 1,
    t_cycles: 4,
    description: "LD B, E",
  },
  // 0x44
  Instruction {
    func: ld_b_h,
    length: 1,
    t_cycles: 4,
    description: "LD B, H",
  },
  // 0x45
  Instruction {
    func: ld_b_l,
    length: 1,
    t_cycles: 4,
    description: "LD B, L",
  },
  // 0x46
  Instruction {
    func: ld_b_mhl,
    length: 1,
    t_cycles: 8,
    description: "LD B, [HL]",
  },
  // 0x47
  Instruction {
    func: ld_b_a,
    length: 1,
    t_cycles: 4,
    description: "LD B, A",
  },
  // 0x48
  Instruction {
    func: ld_c_b,
    length: 1,
    t_cycles: 4,
    description: "LD C, B",
  },
  // 0x49
  Instruction {
    func: ld_c_c,
    length: 1,
    t_cycles: 4,
    description: "LD C, C",
  },
  // 0x4A
  Instruction {
    func: ld_c_d,
    length: 1,
    t_cycles: 4,
    description: "LD C, D",
  },
  // 0x4B
  Instruction {
    func: ld_c_e,
    length: 1,
    t_cycles: 4,
    description: "LD C, E",
  },
  // 0x4C
  Instruction {
    func: ld_c_h,
    length: 1,
    t_cycles: 4,
    description: "LD C, H",
  },
  // 0x4D
  Instruction {
    func: ld_c_l,
    length: 1,
    t_cycles: 4,
    description: "LD C, L",
  },
  // 0x4E
  Instruction {
    func: ld_c_mhl,
    length: 1,
    t_cycles: 8,
    description: "LD C, [HL]",
  },
  // 0x4F
  Instruction {
    func: ld_c_a,
    length: 1,
    t_cycles: 4,
    description: "LD C, A",
  },
  // 0x50
  Instruction {
    func: ld_d_b,
    length: 1,
    t_cycles: 4,
    description: "LD D, B",
  },
  // 0x51
  Instruction {
    func: ld_d_c,
    length: 1,
    t_cycles: 4,
    description: "LD D, C",
  },
  // 0x52
  Instruction {
    func: ld_d_d,
    length: 1,
    t_cycles: 4,
    description: "LD D, D",
  },
  // 0x53
  Instruction {
    func: ld_d_e,
    length: 1,
    t_cycles: 4,
    description: "LD D, E",
  },
  // 0x54
  Instruction {
    func: ld_d_h,
    length: 1,
    t_cycles: 4,
    description: "LD D, H",
  },
  // 0x55
  Instruction {
    func: ld_d_l,
    length: 1,
    t_cycles: 4,
    description: "LD D, L",
  },
  // 0x56
  Instruction {
    func: ld_d_mhl,
    length: 1,
    t_cycles: 8,
    description: "LD D, [HL]",
  },
  // 0x57
  Instruction {
    func: ld_d_a,
    length: 1,
    t_cycles: 4,
    description: "LD D, A",
  },
  // 0x58
  Instruction {
    func: ld_e_b,
    length: 1,
    t_cycles: 4,
    description: "LD E, B",
  },
  // 0x59
  Instruction {
    func: ld_e_c,
    length: 1,
    t_cycles: 4,
    description: "LD E, C",
  },
  // 0x5A
  Instruction {
    func: ld_e_d,
    length: 1,
    t_cycles: 4,
    description: "LD E, D",
  },
  // 0x5B
  Instruction {
    func: ld_e_e,
    length: 1,
    t_cycles: 4,
    description: "LD E, E",
  },
  // 0x5C
  Instruction {
    func: ld_e_h,
    length: 1,
    t_cycles: 4,
    description: "LD E, H",
  },
  // 0x5D
  Instruction {
    func: ld_e_l,
    length: 1,
    t_cycles: 4,
    description: "LD E, L",
  },
  // 0x5E
  Instruction {
    func: ld_e_mhl,
    length: 1,
    t_cycles: 8,
    description: "LD E, [HL]",
  },
  // 0x5F
  Instruction {
    func: ld_e_a,
    length: 1,
    t_cycles: 4,
    description: "LD E, A",
  },
  // 0x60
  Instruction {
    func: ld_h_b,
    length: 1,
    t_cycles: 4,
    description: "LD H, B",
  },
  // 0x61
  Instruction {
    func: ld_h_c,
    length: 1,
    t_cycles: 4,
    description: "LD H, C",
  },
  // 0x62
  Instruction {
    func: ld_h_d,
    length: 1,
    t_cycles: 4,
    description: "LD H, D",
  },
  // 0x63
  Instruction {
    func: ld_h_e,
    length: 1,
    t_cycles: 4,
    description: "LD H, E",
  },
  // 0x64
  Instruction {
    func: ld_h_h,
    length: 1,
    t_cycles: 4,
    description: "LD H, H",
  },
  // 0x65
  Instruction {
    func: ld_h_l,
    length: 1,
    t_cycles: 4,
    description: "LD H, L",
  },
  // 0x66
  Instruction {
    func: ld_h_mhl,
    length: 1,
    t_cycles: 8,
    description: "LD H, [HL]",
  },
  // 0x67
  Instruction {
    func: ld_h_a,
    length: 1,
    t_cycles: 4,
    description: "LD H, A",
  },
  // 0x68
  Instruction {
    func: ld_l_b,
    length: 1,
    t_cycles: 4,
    description: "LD L, B",
  },
  // 0x69
  Instruction {
    func: ld_l_c,
    length: 1,
    t_cycles: 4,
    description: "LD L, C",
  },
  // 0x6A
  Instruction {
    func: ld_l_d,
    length: 1,
    t_cycles: 4,
    description: "LD L, D",
  },
  // 0x6B
  Instruction {
    func: ld_l_e,
    length: 1,
    t_cycles: 4,
    description: "LD L, E",
  },
  // 0x6C
  Instruction {
    func: ld_l_h,
    length: 1,
    t_cycles: 4,
    description: "LD L, H",
  },
  // 0x6D
  Instruction {
    func: ld_l_l,
    length: 1,
    t_cycles: 4,
    description: "LD L, L",
  },
  // 0x6E
  Instruction {
    func: ld_l_mhl,
    length: 1,
    t_cycles: 8,
    description: "LD L, [HL]",
  },
  // 0x6F
  Instruction {
    func: ld_l_a,
    length: 1,
    t_cycles: 4,
    description: "LD L, A",
  },
  // 0x70
  Instruction {
    func: ld_mhl_b,
    length: 1,
    t_cycles: 8,
    description: "LD [HL], B",
  },
  // 0x71
  Instruction {
    func: ld_mhl_c,
    length: 1,
    t_cycles: 8,
    description: "LD [HL], C",
  },
  // 0x72
  Instruction {
    func: ld_mhl_d,
    length: 1,
    t_cycles: 8,
    description: "LD [HL], D",
  },
  // 0x73
  Instruction {
    func: ld_mhl_e,
    length: 1,
    t_cycles: 8,
    description: "LD [HL], E",
  },
  // 0x74
  Instruction {
    func: ld_mhl_h,
    length: 1,
    t_cycles: 8,
    description: "LD [HL], H",
  },
  // 0x75
  Instruction {
    func: ld_mhl_l,
    length: 1,
    t_cycles: 8,
    description: "LD [HL], L",
  },
  // 0x76
  Instruction {
    func: halt,
    length: 1,
    t_cycles: 4,
    description: "HALT",
  },
  // 0x77
  Instruction {
    func: ld_mhl_a,
    length: 1,
    t_cycles: 8,
    description: "LD [HL], A",
  },
  // 0x78
  Instruction {
    func: ld_a_b,
    length: 1,
    t_cycles: 4,
    description: "LD A, B",
  },
  // 0x79
  Instruction {
    func: ld_a_c,
    length: 1,
    t_cycles: 4,
    description: "LD A, C",
  },
  // 0x7A
  Instruction {
    func: ld_a_d,
    length: 1,
    t_cycles: 4,
    description: "LD A, D",
  },
  // 0x7B
  Instruction {
    func: ld_a_e,
    length: 1,
    t_cycles: 4,
    description: "LD A, E",
  },
  // 0x7C
  Instruction {
    func: ld_a_h,
    length: 1,
    t_cycles: 4,
    description: "LD A, H",
  },
  // 0x7D
  Instruction {
    func: ld_a_l,
    length: 1,
    t_cycles: 4,
    description: "LD A, L",
  },
  // 0x7E
  Instruction {
    func: ld_a_mhl,
    length: 1,
    t_cycles: 8,
    description: "LD A, [HL]",
  },
  // 0x7F
  Instruction {
    func: ld_a_a,
    length: 1,
    t_cycles: 4,
    description: "LD A, A",
  },
  // 0x80
  Instruction {
    func: add_a_b,
    length: 1,
    t_cycles: 4,
    description: "ADD A, B",
  },
  // 0x81
  Instruction {
    func: add_a_c,
    length: 1,
    t_cycles: 4,
    description: "ADD A, C",
  },
  // 0x82
  Instruction {
    func: add_a_d,
    length: 1,
    t_cycles: 4,
    description: "ADD A, D",
  },
  // 0x83
  Instruction {
    func: add_a_e,
    length: 1,
    t_cycles: 4,
    description: "ADD A, E",
  },
  // 0x84
  Instruction {
    func: add_a_h,
    length: 1,
    t_cycles: 4,
    description: "ADD A, H",
  },
  // 0x85
  Instruction {
    func: add_a_l,
    length: 1,
    t_cycles: 4,
    description: "ADD A, L",
  },
  // 0x86
  Instruction {
    func: add_a_mhl,
    length: 1,
    t_cycles: 8,
    description: "ADD A, [HL]",
  },
  // 0x87
  Instruction {
    func: add_a_a,
    length: 1,
    t_cycles: 4,
    description: "ADD A, A",
  },
  // 0x88
  Instruction {
    func: adc_a_b,
    length: 1,
    t_cycles: 4,
    description: "ADC A, B",
  },
  // 0x89
  Instruction {
    func: adc_a_c,
    length: 1,
    t_cycles: 4,
    description: "ADC A, C",
  },
  // 0x8A
  Instruction {
    func: adc_a_d,
    length: 1,
    t_cycles: 4,
    description: "ADC A, D",
  },
  // 0x8B
  Instruction {
    func: adc_a_e,
    length: 1,
    t_cycles: 4,
    description: "ADC A, E",
  },
  // 0x8C
  Instruction {
    func: adc_a_h,
    length: 1,
    t_cycles: 4,
    description: "ADC A, H",
  },
  // 0x8D
  Instruction {
    func: adc_a_l,
    length: 1,
    t_cycles: 4,
    description: "ADC A, L",
  },
  // 0x8E
  Instruction {
    func: adc_a_mhl,
    length: 1,
    t_cycles: 8,
    description: "ADC A, [HL]",
  },
  // 0x8F
  Instruction {
    func: adc_a_a,
    length: 1,
    t_cycles: 4,
    description: "ADC A, A",
  },
  // 0x90
  Instruction {
    func: sub_a_b,
    length: 1,
    t_cycles: 4,
    description: "SUB B",
  },
  // 0x91
  Instruction {
    func: sub_a_c,
    length: 1,
    t_cycles: 4,
    description: "SUB C",
  },
  // 0x92
  Instruction {
    func: sub_a_d,
    length: 1,
    t_cycles: 4,
    description: "SUB D",
  },
  // 0x93
  Instruction {
    func: sub_a_e,
    length: 1,
    t_cycles: 4,
    description: "SUB E",
  },
  // 0x94
  Instruction {
    func: sub_a_h,
    length: 1,
    t_cycles: 4,
    description: "SUB H",
  },
  // 0x95
  Instruction {
    func: sub_a_l,
    length: 1,
    t_cycles: 4,
    description: "SUB L",
  },
  // 0x96
  Instruction {
    func: sub_a_mhl,
    length: 1,
    t_cycles: 8,
    description: "SUB [HL]",
  },
  // 0x97
  Instruction {
    func: sub_a_a,
    length: 1,
    t_cycles: 4,
    description: "SUB A",
  },
  // 0x98
  Instruction {
    func: sbc_a_b,
    length: 1,
    t_cycles: 4,
    description: "SBC A, B",
  },
  // 0x99
  Instruction {
    func: sbc_a_c,
    length: 1,
    t_cycles: 4,
    description: "SBC A, C",
  },
  // 0x9A
  Instruction {
    func: sbc_a_d,
    length: 1,
    t_cycles: 4,
    description: "SBC A, D",
  },
  // 0x9B
  Instruction {
    func: sbc_a_e,
    length: 1,
    t_cycles: 4,
    description: "SBC A, E",
  },
  // 0x9C
  Instruction {
    func: sbc_a_h,
    length: 1,
    t_cycles: 4,
    description: "SBC A, H",
  },
  // 0x9D
  Instruction {
    func: sbc_a_l,
    length: 1,
    t_cycles: 4,
    description: "SBC A, L",
  },
  // 0x9E
  Instruction {
    func: sbc_a_mhl,
    length: 1,
    t_cycles: 8,
    description: "SBC A, [HL]",
  },
  // 0x9F
  Instruction {
    func: sbc_a_a,
    length: 1,
    t_cycles: 4,
    description: "SBC A, A",
  },
  // 0xA0
  Instruction {
    func: and_a_b,
    length: 1,
    t_cycles: 4,
    description: "AND A, B",
  },
  // 0xA1
  Instruction {
    func: and_a_c,
    length: 1,
    t_cycles: 4,
    description: "AND A, C",
  },
  // 0xA2
  Instruction {
    func: and_a_d,
    length: 1,
    t_cycles: 4,
    description: "AND A, D",
  },
  // 0xA3
  Instruction {
    func: and_a_e,
    length: 1,
    t_cycles: 4,
    description: "AND A, E",
  },
  // 0xA4
  Instruction {
    func: and_a_h,
    length: 1,
    t_cycles: 4,
    description: "AND A, H",
  },
  // 0xA5
  Instruction {
    func: and_a_l,
    length: 1,
    t_cycles: 4,
    description: "AND A, L",
  },
  // 0xA6
  Instruction {
    func: and_a_mhl,
    length: 1,
    t_cycles: 8,
    description: "AND A, [HL]",
  },
  // 0xA7
  Instruction {
    func: and_a_a,
    length: 1,
    t_cycles: 4,
    description: "AND A, A",
  },
  // 0xA8
  Instruction {
    func: xor_a_b,
    length: 1,
    t_cycles: 4,
    description: "XOR A, B",
  },
  // 0xA9
  Instruction {
    func: xor_a_c,
    length: 1,
    t_cycles: 4,
    description: "XOR A, C",
  },
  // 0xAA
  Instruction {
    func: xor_a_d,
    length: 1,
    t_cycles: 4,
    description: "XOR A, D",
  },
  // 0xAB
  Instruction {
    func: xor_a_e,
    length: 1,
    t_cycles: 4,
    description: "XOR A, E",
  },
  // 0xAC
  Instruction {
    func: xor_a_h,
    length: 1,
    t_cycles: 4,
    description: "XOR A, H",
  },
  // 0xAD
  Instruction {
    func: xor_a_l,
    length: 1,
    t_cycles: 4,
    description: "XOR A, L",
  },
  // 0xAE
  Instruction {
    func: xor_a_mhl,
    length: 1,
    t_cycles: 8,
    description: "XOR A, [HL]",
  },
  // 0xAF
  Instruction {
    func: xor_a_a,
    length: 1,
    t_cycles: 4,
    description: "XOR A, A",
  },
  // 0xB0
  Instruction {
    func: or_a_b,
    length: 1,
    t_cycles: 4,
    description: "OR A, B",
  },
  // 0xB1
  Instruction {
    func: or_a_c,
    length: 1,
    t_cycles: 4,
    description: "OR A, C",
  },
  // 0xB2
  Instruction {
    func: or_a_d,
    length: 1,
    t_cycles: 4,
    description: "OR A, D",
  },
  // 0xB3
  Instruction {
    func: or_a_e,
    length: 1,
    t_cycles: 4,
    description: "OR A, E",
  },
  // 0xB4
  Instruction {
    func: or_a_h,
    length: 1,
    t_cycles: 4,
    description: "OR A, H",
  },
  // 0xB5
  Instruction {
    func: or_a_l,
    length: 1,
    t_cycles: 4,
    description: "OR A, L",
  },
  // 0xB6
  Instruction {
    func: or_a_mhl,
    length: 1,
    t_cycles: 8,
    description: "OR A, [HL]",
  },
  // 0xB7
  Instruction {
    func: or_a_a,
    length: 1,
    t_cycles: 4,
    description: "OR A, A",
  },
  // 0xB8
  Instruction {
    func: cp_a_b,
    length: 1,
    t_cycles: 4,
    description: "CP A, B",
  },
  // 0xB9
  Instruction {
    func: cp_a_c,
    length: 1,
    t_cycles: 4,
    description: "CP A, C",
  },
  // 0xBA
  Instruction {
    func: cp_a_d,
    length: 1,
    t_cycles: 4,
    description: "CP A, D",
  },
  // 0xBB
  Instruction {
    func: cp_a_e,
    length: 1,
    t_cycles: 4,
    description: "CP A, E",
  },
  // 0xBC
  Instruction {
    func: cp_a_h,
    length: 1,
    t_cycles: 4,
    description: "CP A, H",
  },
  // 0xBD
  Instruction {
    func: cp_a_l,
    length: 1,
    t_cycles: 4,
    description: "CP A, L",
  },
  // 0xBE
  Instruction {
    func: cp_a_mhl,
    length: 1,
    t_cycles: 8,
    description: "CP A, [HL]",
  },
  // 0xBF
  Instruction {
    func: cp_a_a,
    length: 1,
    t_cycles: 4,
    description: "CP A, A",
  },
  // 0xC0
  // TODO: fix t_cycles
  Instruction {
    func: ret_nz,
    length: 1,
    t_cycles: 8,
    description: "RET NZ",
  },
  // 0xC1
  Instruction {
    func: pop_bc,
    length: 1,
    t_cycles: 12,
    description: "POP BC",
  },
  // 0xC2
  // TODO: fix t_cycles
  Instruction {
    func: jp_nz_u16,
    length: 3,
    t_cycles: 12,
    description: "JP NZ, u16",
  },
  // 0xC3
  Instruction {
    func: jp_u16,
    length: 3,
    t_cycles: 16,
    description: "JP u16",
  },
  // 0xC4
  // TODO: fix t_cycles
  Instruction {
    func: call_nz_u16,
    length: 3,
    t_cycles: 12,
    description: "CALL NZ, u16",
  },
  // 0xC5
  Instruction {
    func: push_bc,
    length: 1,
    t_cycles: 16,
    description: "PUSH BC",
  },
  // 0xC6
  Instruction {
    func: add_a_u8,
    length: 2,
    t_cycles: 8,
    description: "ADD A, u8",
  },
  // 0xC7
  Instruction {
    func: rst_00h,
    length: 1,
    t_cycles: 16,
    description: "RST 00h",
  },
  // 0xC8
  // TODO: fix t_cycles
  Instruction {
    func: ret_z,
    length: 1,
    t_cycles: 8,
    description: "RET Z",
  },
  // 0xC9
  Instruction {
    func: ret,
    length: 1,
    t_cycles: 16,
    description: "RET",
  },
  // 0xCA
  // TODO: fix t_cycles
  Instruction {
    func: jp_z_u16,
    length: 3,
    t_cycles: 12,
    description: "JP Z, u16",
  },
  // 0xCB
  Instruction {
    func: prefix_cb,
    length: 1,
    t_cycles: 4,
    description: "PREFIX CB",
  },
  // 0xCC
  // TODO: fix t_cycles
  Instruction {
    func: call_z_u16,
    length: 3,
    t_cycles: 12,
    description: "CALL Z, u16",
  },
  // 0xCD
  Instruction {
    func: call_u16,
    length: 3,
    t_cycles: 24,
    description: "CALL u16",
  },
  // 0xCE
  Instruction {
    func: adc_a_u8,
    length: 2,
    t_cycles: 8,
    description: "ADC A, u8",
  },
  // 0xCF
  Instruction {
    func: rst_08h,
    length: 1,
    t_cycles: 16,
    description: "RST 08h",
  },
  // 0xD0
  // TODO: fix t_cycles
  Instruction {
    func: ret_nc,
    length: 1,
    t_cycles: 8,
    description: "RET NC",
  },
  // 0xD1
  Instruction {
    func: pop_de,
    length: 1,
    t_cycles: 12,
    description: "POP DE",
  },
  // 0xD2
  // TODO: fix t_cycles
  Instruction {
    func: jp_nc_u16,
    length: 3,
    t_cycles: 12,
    description: "JP NC, u16",
  },
  // 0xD3
  Instruction {
    func: illegal,
    length: 0,
    t_cycles: 0,
    description: "illegal",
  },
  // 0xD4
  // TODO: fix t_cycles
  Instruction {
    func: call_nc_u16,
    length: 3,
    t_cycles: 12,
    description: "CALL NC, u16",
  },
  // 0xD5
  Instruction {
    func: push_de,
    length: 1,
    t_cycles: 16,
    description: "PUSH DE",
  },
  // 0xD6
  Instruction {
    func: sub_a_u8,
    length: 2,
    t_cycles: 8,
    description: "SUB A, u8",
  },
  // 0xD7
  Instruction {
    func: rst_10h,
    length: 1,
    t_cycles: 16,
    description: "RST 10h",
  },
  // 0xD8
  // TODO: fix t_cycles
  Instruction {
    func: ret_c,
    length: 1,
    t_cycles: 8,
    description: "RET C",
  },
  // 0xD9
  Instruction {
    func: reti,
    length: 1,
    t_cycles: 16,
    description: "RETI",
  },
  // 0xDA
  // TODO: fix t_cycles
  Instruction {
    func: jp_c_u16,
    length: 3,
    t_cycles: 12,
    description: "JP C, u16",
  },
  // 0xDB
  Instruction {
    func: illegal,
    length: 0,
    t_cycles: 0,
    description: "illegal",
  },
  // 0xDC
  // TODO: fix t_cycles
  Instruction {
    func: call_c_u16,
    length: 3,
    t_cycles: 12,
    description: "CALL C, u16",
  },
  // 0xDD
  Instruction {
    func: illegal,
    length: 0,
    t_cycles: 0,
    description: "illegal",
  },
  // 0xDE
  Instruction {
    func: sbc_a_u8,
    length: 2,
    t_cycles: 8,
    description: "SBC A, u8",
  },
  // 0xDF
  Instruction {
    func: rst_18h,
    length: 1,
    t_cycles: 16,
    description: "RST 18h",
  },
  // 0xE0
  Instruction {
    func: ld_mff00_u8_a,
    length: 2,
    t_cycles: 12,
    description: "LD [FF00+u8], A",
  },
  // 0xE1
  Instruction {
    func: pop_hl,
    length: 1,
    t_cycles: 12,
    description: "POP HL",
  },
  // 0xE2
  Instruction {
    func: ld_mff00_c_a,
    length: 1,
    t_cycles: 8,
    description: "LD [FF00+C], A",
  },
  // 0xE3
  Instruction {
    func: illegal,
    length: 0,
    t_cycles: 0,
    description: "illegal",
  },
  // 0xE4
  Instruction {
    func: illegal,
    length: 0,
    t_cycles: 0,
    description: "illegal",
  },
  // 0xE5
  Instruction {
    func: push_hl,
    length: 1,
    t_cycles: 16,
    description: "PUSH HL",
  },
  // 0xE6
  Instruction {
    func: and_a_u8,
    length: 2,
    t_cycles: 8,
    description: "AND A, u8",
  },
  // 0xE7
  Instruction {
    func: rst_20h,
    length: 1,
    t_cycles: 16,
    description: "RST 20h",
  },
  // 0xE8
  Instruction {
    func: add_sp_i8,
    length: 2,
    t_cycles: 16,
    description: "ADD SP, i8",
  },
  // 0xE9
  Instruction {
    func: jp_hl,
    length: 1,
    t_cycles: 4,
    description: "JP HL",
  },
  // 0xEA
  // TODO: fix name
  Instruction {
    func: ld_m_u16_a,
    length: 3,
    t_cycles: 16,
    description: "LD [u16], A",
  },
  // 0xEB
  Instruction {
    func: illegal,
    length: 0,
    t_cycles: 0,
    description: "illegal",
  },
  // 0xEC
  Instruction {
    func: illegal,
    length: 0,
    t_cycles: 0,
    description: "illegal",
  },
  // 0xED
  Instruction {
    func: illegal,
    length: 0,
    t_cycles: 0,
    description: "illegal",
  },
  // 0xEE
  Instruction {
    func: xor_a_u8,
    length: 2,
    t_cycles: 8,
    description: "XOR A, u8",
  },
  // 0xEF
  Instruction {
    func: rst_28h,
    length: 1,
    t_cycles: 16,
    description: "RST 28h",
  },
  // 0xF0
  // TODO: fix name
  Instruction {
    func: ld_a_mff00_u8,
    length: 2,
    t_cycles: 12,
    description: "LD A, [FF00+u8]",
  },
  // 0xF1
  Instruction {
    func: pop_af,
    length: 1,
    t_cycles: 12,
    description: "POP AF",
  },
  // 0xF2
  Instruction {
    func: ld_a_mff00_c,
    length: 1,
    t_cycles: 8,
    description: "LD A, [FF00+C]",
  },
  // 0xF3
  Instruction {
    func: di,
    length: 1,
    t_cycles: 4,
    description: "DI",
  },
  // 0xF4
  Instruction {
    func: illegal,
    length: 0,
    t_cycles: 0,
    description: "illegal",
  },
  // 0xF5
  Instruction {
    func: push_af,
    length: 1,
    t_cycles: 16,
    description: "PUSH AF",
  },
  // 0xF6
  Instruction {
    func: or_a_u8,
    length: 2,
    t_cycles: 8,
    description: "OR A, u8",
  },
  // 0xF7
  Instruction {
    func: rst_30h,
    length: 1,
    t_cycles: 16,
    description: "RST 30h",
  },
  // 0xF8
  Instruction {
    func: ld_hl_sp_i8,
    length: 2,
    t_cycles: 12,
    description: "LD HL, SP+i8",
  },
  // 0xF9
  Instruction {
    func: ld_sp_hl,
    length: 1,
    t_cycles: 8,
    description: "LD SP, HL",
  },
  // 0xFA
  // TODO: fix name
  Instruction {
    func: ld_a_m_u16,
    length: 3,
    t_cycles: 16,
    description: "LD A, [u16]",
  },
  // 0xFB
  Instruction {
    func: ei,
    length: 1,
    t_cycles: 4,
    description: "EI",
  },
  // 0xFC
  Instruction {
    func: illegal,
    length: 0,
    t_cycles: 0,
    description: "illegal",
  },
  // 0xFD
  Instruction {
    func: illegal,
    length: 0,
    t_cycles: 0,
    description: "illegal",
  },
  // 0xFE
  Instruction {
    func: cp_a_u8,
    length: 2,
    t_cycles: 8,
    description: "CP A, u8",
  },
  // 0xFF
  Instruction {
    func: rst_38h,
    length: 1,
    t_cycles: 16,
    description: "RST 38h",
  },
];

const PREFIXED: [Instruction; 256] = [
  // 0x00
  Instruction {
    func: rlc_b,
    length: 2,
    t_cycles: 8,
    description: "RLC B",
  },
  // 0x01
  Instruction {
    func: rlc_c,
    length: 2,
    t_cycles: 8,
    description: "RLC C",
  },
  // 0x02
  Instruction {
    func: rlc_d,
    length: 2,
    t_cycles: 8,
    description: "RLC D",
  },
  // 0x03
  Instruction {
    func: rlc_e,
    length: 2,
    t_cycles: 8,
    description: "RLC E",
  },
  // 0x04
  Instruction {
    func: rlc_h,
    length: 2,
    t_cycles: 8,
    description: "RLC H",
  },
  // 0x05
  Instruction {
    func: rlc_l,
    length: 2,
    t_cycles: 8,
    description: "RLC L",
  },
  // 0x06
  Instruction {
    func: rlc_mhl,
    length: 2,
    t_cycles: 16,
    description: "RLC [HL]",
  },
  // 0x07
  Instruction {
    func: rlc_a,
    length: 2,
    t_cycles: 8,
    description: "RLC A",
  },
  // 0x08
  Instruction {
    func: rrc_b,
    length: 2,
    t_cycles: 8,
    description: "RRC B",
  },
  // 0x09
  Instruction {
    func: rrc_c,
    length: 2,
    t_cycles: 8,
    description: "RRC C",
  },
  // 0x0A
  Instruction {
    func: rrc_d,
    length: 2,
    t_cycles: 8,
    description: "RRC D",
  },
  // 0x0B
  Instruction {
    func: rrc_e,
    length: 2,
    t_cycles: 8,
    description: "RRC E",
  },
  // 0x0C
  Instruction {
    func: rrc_h,
    length: 2,
    t_cycles: 8,
    description: "RRC H",
  },
  // 0x0D
  Instruction {
    func: rrc_l,
    length: 2,
    t_cycles: 8,
    description: "RRC L",
  },
  // 0x0E
  Instruction {
    func: rrc_mhl,
    length: 2,
    t_cycles: 16,
    description: "RRC [HL]",
  },
  // 0x0F
  Instruction {
    func: rrc_a,
    length: 2,
    t_cycles: 8,
    description: "RRC A",
  },
  // 0x10
  Instruction {
    func: rl_b,
    length: 2,
    t_cycles: 8,
    description: "RL B",
  },
  // 0x11
  Instruction {
    func: rl_c,
    length: 2,
    t_cycles: 8,
    description: "RL C",
  },
  // 0x12
  Instruction {
    func: rl_d,
    length: 2,
    t_cycles: 8,
    description: "RL D",
  },
  // 0x13
  Instruction {
    func: rl_e,
    length: 2,
    t_cycles: 8,
    description: "RL E",
  },
  // 0x14
  Instruction {
    func: rl_h,
    length: 2,
    t_cycles: 8,
    description: "RL H",
  },
  // 0x15
  Instruction {
    func: rl_l,
    length: 2,
    t_cycles: 8,
    description: "RL L",
  },
  // 0x16
  Instruction {
    func: rl_mhl,
    length: 2,
    t_cycles: 16,
    description: "RL [HL]",
  },
  // 0x17
  Instruction {
    func: rl_a,
    length: 2,
    t_cycles: 8,
    description: "RL A",
  },
  // 0x18
  Instruction {
    func: rr_b,
    length: 2,
    t_cycles: 8,
    description: "RR B",
  },
  // 0x19
  Instruction {
    func: rr_c,
    length: 2,
    t_cycles: 8,
    description: "RR C",
  },
  // 0x1A
  Instruction {
    func: rr_d,
    length: 2,
    t_cycles: 8,
    description: "RR D",
  },
  // 0x1B
  Instruction {
    func: rr_e,
    length: 2,
    t_cycles: 8,
    description: "RR E",
  },
  // 0x1C
  Instruction {
    func: rr_h,
    length: 2,
    t_cycles: 8,
    description: "RR H",
  },
  // 0x1D
  Instruction {
    func: rr_l,
    length: 2,
    t_cycles: 8,
    description: "RR L",
  },
  // 0x1E
  Instruction {
    func: rr_mhl,
    length: 2,
    t_cycles: 16,
    description: "RR [HL]",
  },
  // 0x1F
  Instruction {
    func: rr_a,
    length: 2,
    t_cycles: 8,
    description: "RR A",
  },
  // 0x20
  Instruction {
    func: sla_b,
    length: 2,
    t_cycles: 8,
    description: "SLA B",
  },
  // 0x21
  Instruction {
    func: sla_c,
    length: 2,
    t_cycles: 8,
    description: "SLA C",
  },
  // 0x22
  Instruction {
    func: sla_d,
    length: 2,
    t_cycles: 8,
    description: "SLA D",
  },
  // 0x23
  Instruction {
    func: sla_e,
    length: 2,
    t_cycles: 8,
    description: "SLA E",
  },
  // 0x24
  Instruction {
    func: sla_h,
    length: 2,
    t_cycles: 8,
    description: "SLA H",
  },
  // 0x25
  Instruction {
    func: sla_l,
    length: 2,
    t_cycles: 8,
    description: "SLA L",
  },
  // 0x26
  Instruction {
    func: sla_mhl,
    length: 2,
    t_cycles: 16,
    description: "SLA [HL]",
  },
  // 0x27
  Instruction {
    func: sla_a,
    length: 2,
    t_cycles: 8,
    description: "SLA A",
  },
  // 0x28
  Instruction {
    func: sra_b,
    length: 2,
    t_cycles: 8,
    description: "SRA B",
  },
  // 0x29
  Instruction {
    func: sra_c,
    length: 2,
    t_cycles: 8,
    description: "SRA C",
  },
  // 0x2A
  Instruction {
    func: sra_d,
    length: 2,
    t_cycles: 8,
    description: "SRA D",
  },
  // 0x2B
  Instruction {
    func: sra_e,
    length: 2,
    t_cycles: 8,
    description: "SRA E",
  },
  // 0x2C
  Instruction {
    func: sra_h,
    length: 2,
    t_cycles: 8,
    description: "SRA H",
  },
  // 0x2D
  Instruction {
    func: sra_l,
    length: 2,
    t_cycles: 8,
    description: "SRA L",
  },
  // 0x2E
  Instruction {
    func: sra_mhl,
    length: 2,
    t_cycles: 16,
    description: "SRA [HL]",
  },
  // 0x2F
  Instruction {
    func: sra_a,
    length: 2,
    t_cycles: 8,
    description: "SRA A",
  },
  // 0x30
  Instruction {
    func: swap_b,
    length: 2,
    t_cycles: 8,
    description: "SWAP B",
  },
  // 0x31
  Instruction {
    func: swap_c,
    length: 2,
    t_cycles: 8,
    description: "SWAP C",
  },
  // 0x32
  Instruction {
    func: swap_d,
    length: 2,
    t_cycles: 8,
    description: "SWAP D",
  },
  // 0x33
  Instruction {
    func: swap_e,
    length: 2,
    t_cycles: 8,
    description: "SWAP E",
  },
  // 0x34
  Instruction {
    func: swap_h,
    length: 2,
    t_cycles: 8,
    description: "SWAP H",
  },
  // 0x35
  Instruction {
    func: swap_l,
    length: 2,
    t_cycles: 8,
    description: "SWAP L",
  },
  // 0x36
  Instruction {
    func: swap_mhl,
    length: 2,
    t_cycles: 16,
    description: "SWAP [HL]",
  },
  // 0x37
  Instruction {
    func: swap_a,
    length: 2,
    t_cycles: 8,
    description: "SWAP A",
  },
  // 0x38
  Instruction {
    func: srl_b,
    length: 2,
    t_cycles: 8,
    description: "SRL B",
  },
  // 0x39
  Instruction {
    func: srl_c,
    length: 2,
    t_cycles: 8,
    description: "SRL C",
  },
  // 0x3A
  Instruction {
    func: srl_d,
    length: 2,
    t_cycles: 8,
    description: "SRL D",
  },
  // 0x3B
  Instruction {
    func: srl_e,
    length: 2,
    t_cycles: 8,
    description: "SRL E",
  },
  // 0x3C
  Instruction {
    func: srl_h,
    length: 2,
    t_cycles: 8,
    description: "SRL H",
  },
  // 0x3D
  Instruction {
    func: srl_l,
    length: 2,
    t_cycles: 8,
    description: "SRL L",
  },
  // 0x3E
  Instruction {
    func: srl_mhl,
    length: 2,
    t_cycles: 16,
    description: "SRL [HL]",
  },
  // 0x3F
  Instruction {
    func: srl_a,
    length: 2,
    t_cycles: 8,
    description: "SRL A",
  },
  // 0x40
  Instruction {
    func: bit_0_b,
    length: 2,
    t_cycles: 8,
    description: "BIT 0, B",
  },
  // 0x41
  Instruction {
    func: bit_0_c,
    length: 2,
    t_cycles: 8,
    description: "BIT 0, C",
  },
  // 0x42
  Instruction {
    func: bit_0_d,
    length: 2,
    t_cycles: 8,
    description: "BIT 0, D",
  },
  // 0x43
  Instruction {
    func: bit_0_e,
    length: 2,
    t_cycles: 8,
    description: "BIT 0, E",
  },
  // 0x44
  Instruction {
    func: bit_0_h,
    length: 2,
    t_cycles: 8,
    description: "BIT 0, H",
  },
  // 0x45
  Instruction {
    func: bit_0_l,
    length: 2,
    t_cycles: 8,
    description: "BIT 0, L",
  },
  // 0x46
  Instruction {
    func: bit_0_mhl,
    length: 2,
    t_cycles: 12,
    description: "BIT 0, [HL]",
  },
  // 0x47
  Instruction {
    func: bit_0_a,
    length: 2,
    t_cycles: 8,
    description: "BIT 0, A",
  },
  // 0x48
  Instruction {
    func: bit_1_b,
    length: 2,
    t_cycles: 8,
    description: "BIT 1, B",
  },
  // 0x49
  Instruction {
    func: bit_1_c,
    length: 2,
    t_cycles: 8,
    description: "BIT 1, C",
  },
  // 0x4A
  Instruction {
    func: bit_1_d,
    length: 2,
    t_cycles: 8,
    description: "BIT 1, D",
  },
  // 0x4B
  Instruction {
    func: bit_1_e,
    length: 2,
    t_cycles: 8,
    description: "BIT 1, E",
  },
  // 0x4C
  Instruction {
    func: bit_1_h,
    length: 2,
    t_cycles: 8,
    description: "BIT 1, H",
  },
  // 0x4D
  Instruction {
    func: bit_1_l,
    length: 2,
    t_cycles: 8,
    description: "BIT 1, L",
  },
  // 0x4E
  Instruction {
    func: bit_1_mhl,
    length: 2,
    t_cycles: 12,
    description: "BIT 1, [HL]",
  },
  // 0x4F
  Instruction {
    func: bit_1_a,
    length: 2,
    t_cycles: 8,
    description: "BIT 1, A",
  },
  // 0x50
  Instruction {
    func: bit_2_b,
    length: 2,
    t_cycles: 8,
    description: "BIT 2, B",
  },
  // 0x51
  Instruction {
    func: bit_2_c,
    length: 2,
    t_cycles: 8,
    description: "BIT 2, C",
  },
  // 0x52
  Instruction {
    func: bit_2_d,
    length: 2,
    t_cycles: 8,
    description: "BIT 2, D",
  },
  // 0x53
  Instruction {
    func: bit_2_e,
    length: 2,
    t_cycles: 8,
    description: "BIT 2, E",
  },
  // 0x54
  Instruction {
    func: bit_2_h,
    length: 2,
    t_cycles: 8,
    description: "BIT 2, H",
  },
  // 0x55
  Instruction {
    func: bit_2_l,
    length: 2,
    t_cycles: 8,
    description: "BIT 2, L",
  },
  // 0x56
  Instruction {
    func: bit_2_mhl,
    length: 2,
    t_cycles: 12,
    description: "BIT 2, [HL]",
  },
  // 0x57
  Instruction {
    func: bit_2_a,
    length: 2,
    t_cycles: 8,
    description: "BIT 2, A",
  },
  // 0x58
  Instruction {
    func: bit_3_b,
    length: 2,
    t_cycles: 8,
    description: "BIT 3, B",
  },
  // 0x59
  Instruction {
    func: bit_3_c,
    length: 2,
    t_cycles: 8,
    description: "BIT 3, C",
  },
  // 0x5A
  Instruction {
    func: bit_3_d,
    length: 2,
    t_cycles: 8,
    description: "BIT 3, D",
  },
  // 0x5B
  Instruction {
    func: bit_3_e,
    length: 2,
    t_cycles: 8,
    description: "BIT 3, E",
  },
  // 0x5C
  Instruction {
    func: bit_3_h,
    length: 2,
    t_cycles: 8,
    description: "BIT 3, H",
  },
  // 0x5D
  Instruction {
    func: bit_3_l,
    length: 2,
    t_cycles: 8,
    description: "BIT 3, L",
  },
  // 0x5E
  Instruction {
    func: bit_3_mhl,
    length: 2,
    t_cycles: 12,
    description: "BIT 3, [HL]",
  },
  // 0x5F
  Instruction {
    func: bit_3_a,
    length: 2,
    t_cycles: 8,
    description: "BIT 3, A",
  },
  // 0x60
  Instruction {
    func: bit_4_b,
    length: 2,
    t_cycles: 8,
    description: "BIT 4, B",
  },
  // 0x61
  Instruction {
    func: bit_4_c,
    length: 2,
    t_cycles: 8,
    description: "BIT 4, C",
  },
  // 0x62
  Instruction {
    func: bit_4_d,
    length: 2,
    t_cycles: 8,
    description: "BIT 4, D",
  },
  // 0x63
  Instruction {
    func: bit_4_e,
    length: 2,
    t_cycles: 8,
    description: "BIT 4, E",
  },
  // 0x64
  Instruction {
    func: bit_4_h,
    length: 2,
    t_cycles: 8,
    description: "BIT 4, H",
  },
  // 0x65
  Instruction {
    func: bit_4_l,
    length: 2,
    t_cycles: 8,
    description: "BIT 4, L",
  },
  // 0x66
  Instruction {
    func: bit_4_mhl,
    length: 2,
    t_cycles: 12,
    description: "BIT 4, [HL]",
  },
  // 0x67
  Instruction {
    func: bit_4_a,
    length: 2,
    t_cycles: 8,
    description: "BIT 4, A",
  },
  // 0x68
  Instruction {
    func: bit_5_b,
    length: 2,
    t_cycles: 8,
    description: "BIT 5, B",
  },
  // 0x69
  Instruction {
    func: bit_5_c,
    length: 2,
    t_cycles: 8,
    description: "BIT 5, C",
  },
  // 0x6A
  Instruction {
    func: bit_5_d,
    length: 2,
    t_cycles: 8,
    description: "BIT 5, D",
  },
  // 0x6B
  Instruction {
    func: bit_5_e,
    length: 2,
    t_cycles: 8,
    description: "BIT 5, E",
  },
  // 0x6C
  Instruction {
    func: bit_5_h,
    length: 2,
    t_cycles: 8,
    description: "BIT 5, H",
  },
  // 0x6D
  Instruction {
    func: bit_5_l,
    length: 2,
    t_cycles: 8,
    description: "BIT 5, L",
  },
  // 0x6E
  Instruction {
    func: bit_5_mhl,
    length: 2,
    t_cycles: 12,
    description: "BIT 5, [HL]",
  },
  // 0x6F
  Instruction {
    func: bit_5_a,
    length: 2,
    t_cycles: 8,
    description: "BIT 5, A",
  },
  // 0x70
  Instruction {
    func: bit_6_b,
    length: 2,
    t_cycles: 8,
    description: "BIT 6, B",
  },
  // 0x71
  Instruction {
    func: bit_6_c,
    length: 2,
    t_cycles: 8,
    description: "BIT 6, C",
  },
  // 0x72
  Instruction {
    func: bit_6_d,
    length: 2,
    t_cycles: 8,
    description: "BIT 6, D",
  },
  // 0x73
  Instruction {
    func: bit_6_e,
    length: 2,
    t_cycles: 8,
    description: "BIT 6, E",
  },
  // 0x74
  Instruction {
    func: bit_6_h,
    length: 2,
    t_cycles: 8,
    description: "BIT 6, H",
  },
  // 0x75
  Instruction {
    func: bit_6_l,
    length: 2,
    t_cycles: 8,
    description: "BIT 6, L",
  },
  // 0x76
  Instruction {
    func: bit_6_mhl,
    length: 2,
    t_cycles: 12,
    description: "BIT 6, [HL]",
  },
  // 0x77
  Instruction {
    func: bit_6_a,
    length: 2,
    t_cycles: 8,
    description: "BIT 6, A",
  },
  // 0x78
  Instruction {
    func: bit_7_b,
    length: 2,
    t_cycles: 8,
    description: "BIT 7, B",
  },
  // 0x79
  Instruction {
    func: bit_7_c,
    length: 2,
    t_cycles: 8,
    description: "BIT 7, C",
  },
  // 0x7A
  Instruction {
    func: bit_7_d,
    length: 2,
    t_cycles: 8,
    description: "BIT 7, D",
  },
  // 0x7B
  Instruction {
    func: bit_7_e,
    length: 2,
    t_cycles: 8,
    description: "BIT 7, E",
  },
  // 0x7C
  Instruction {
    func: bit_7_h,
    length: 2,
    t_cycles: 8,
    description: "BIT 7, H",
  },
  // 0x7D
  Instruction {
    func: bit_7_l,
    length: 2,
    t_cycles: 8,
    description: "BIT 7, L",
  },
  // 0x7E
  Instruction {
    func: bit_7_mhl,
    length: 2,
    t_cycles: 12,
    description: "BIT 7, [HL]",
  },
  // 0x7F
  Instruction {
    func: bit_7_a,
    length: 2,
    t_cycles: 8,
    description: "BIT 7, A",
  },
  // 0x80
  Instruction {
    func: res_0_b,
    length: 2,
    t_cycles: 8,
    description: "RES 0, B",
  },
  // 0x81
  Instruction {
    func: res_0_c,
    length: 2,
    t_cycles: 8,
    description: "RES 0, C",
  },
  // 0x82
  Instruction {
    func: res_0_d,
    length: 2,
    t_cycles: 8,
    description: "RES 0, D",
  },
  // 0x83
  Instruction {
    func: res_0_e,
    length: 2,
    t_cycles: 8,
    description: "RES 0, E",
  },
  // 0x84
  Instruction {
    func: res_0_h,
    length: 2,
    t_cycles: 8,
    description: "RES 0, H",
  },
  // 0x85
  Instruction {
    func: res_0_l,
    length: 2,
    t_cycles: 8,
    description: "RES 0, L",
  },
  // 0x86
  Instruction {
    func: res_0_mhl,
    length: 2,
    t_cycles: 16,
    description: "RES 0, [HL]",
  },
  // 0x87
  Instruction {
    func: res_0_a,
    length: 2,
    t_cycles: 8,
    description: "RES 0, A",
  },
  // 0x88
  Instruction {
    func: res_1_b,
    length: 2,
    t_cycles: 8,
    description: "RES 1, B",
  },
  // 0x89
  Instruction {
    func: res_1_c,
    length: 2,
    t_cycles: 8,
    description: "RES 1, C",
  },
  // 0x8A
  Instruction {
    func: res_1_d,
    length: 2,
    t_cycles: 8,
    description: "RES 1, D",
  },
  // 0x8B
  Instruction {
    func: res_1_e,
    length: 2,
    t_cycles: 8,
    description: "RES 1, E",
  },
  // 0x8C
  Instruction {
    func: res_1_h,
    length: 2,
    t_cycles: 8,
    description: "RES 1, H",
  },
  // 0x8D
  Instruction {
    func: res_1_l,
    length: 2,
    t_cycles: 8,
    description: "RES 1, L",
  },
  // 0x8E
  Instruction {
    func: res_1_mhl,
    length: 2,
    t_cycles: 16,
    description: "RES 1, [HL]",
  },
  // 0x8F
  Instruction {
    func: res_1_a,
    length: 2,
    t_cycles: 8,
    description: "RES 1, A",
  },
  // 0x90
  Instruction {
    func: res_2_b,
    length: 2,
    t_cycles: 8,
    description: "RES 2, B",
  },
  // 0x91
  Instruction {
    func: res_2_c,
    length: 2,
    t_cycles: 8,
    description: "RES 2, C",
  },
  // 0x92
  Instruction {
    func: res_2_d,
    length: 2,
    t_cycles: 8,
    description: "RES 2, D",
  },
  // 0x93
  Instruction {
    func: res_2_e,
    length: 2,
    t_cycles: 8,
    description: "RES 2, E",
  },
  // 0x94
  Instruction {
    func: res_2_h,
    length: 2,
    t_cycles: 8,
    description: "RES 2, H",
  },
  // 0x95
  Instruction {
    func: res_2_l,
    length: 2,
    t_cycles: 8,
    description: "RES 2, L",
  },
  // 0x96
  Instruction {
    func: res_2_mhl,
    length: 2,
    t_cycles: 16,
    description: "RES 2, [HL]",
  },
  // 0x97
  Instruction {
    func: res_2_a,
    length: 2,
    t_cycles: 8,
    description: "RES 2, A",
  },
  // 0x98
  Instruction {
    func: res_3_b,
    length: 2,
    t_cycles: 8,
    description: "RES 3, B",
  },
  // 0x99
  Instruction {
    func: res_3_c,
    length: 2,
    t_cycles: 8,
    description: "RES 3, C",
  },
  // 0x9A
  Instruction {
    func: res_3_d,
    length: 2,
    t_cycles: 8,
    description: "RES 3, D",
  },
  // 0x9B
  Instruction {
    func: res_3_e,
    length: 2,
    t_cycles: 8,
    description: "RES 3, E",
  },
  // 0x9C
  Instruction {
    func: res_3_h,
    length: 2,
    t_cycles: 8,
    description: "RES 3, H",
  },
  // 0x9D
  Instruction {
    func: res_3_l,
    length: 2,
    t_cycles: 8,
    description: "RES 3, L",
  },
  // 0x9E
  Instruction {
    func: res_3_mhl,
    length: 2,
    t_cycles: 16,
    description: "RES 3, [HL]",
  },
  // 0x9F
  Instruction {
    func: res_3_a,
    length: 2,
    t_cycles: 8,
    description: "RES 3, A",
  },
  // 0xA0
  Instruction {
    func: res_4_b,
    length: 2,
    t_cycles: 8,
    description: "RES 4, B",
  },
  // 0xA1
  Instruction {
    func: res_4_c,
    length: 2,
    t_cycles: 8,
    description: "RES 4, C",
  },
  // 0xA2
  Instruction {
    func: res_4_d,
    length: 2,
    t_cycles: 8,
    description: "RES 4, D",
  },
  // 0xA3
  Instruction {
    func: res_4_e,
    length: 2,
    t_cycles: 8,
    description: "RES 4, E",
  },
  // 0xA4
  Instruction {
    func: res_4_h,
    length: 2,
    t_cycles: 8,
    description: "RES 4, H",
  },
  // 0xA5
  Instruction {
    func: res_4_l,
    length: 2,
    t_cycles: 8,
    description: "RES 4, L",
  },
  // 0xA6
  Instruction {
    func: res_4_mhl,
    length: 2,
    t_cycles: 16,
    description: "RES 4, [HL]",
  },
  // 0xA7
  Instruction {
    func: res_4_a,
    length: 2,
    t_cycles: 8,
    description: "RES 4, A",
  },
  // 0xA8
  Instruction {
    func: res_5_b,
    length: 2,
    t_cycles: 8,
    description: "RES 5, B",
  },
  // 0xA9
  Instruction {
    func: res_5_c,
    length: 2,
    t_cycles: 8,
    description: "RES 5, C",
  },
  // 0xAA
  Instruction {
    func: res_5_d,
    length: 2,
    t_cycles: 8,
    description: "RES 5, D",
  },
  // 0xAB
  Instruction {
    func: res_5_e,
    length: 2,
    t_cycles: 8,
    description: "RES 5, E",
  },
  // 0xAC
  Instruction {
    func: res_5_h,
    length: 2,
    t_cycles: 8,
    description: "RES 5, H",
  },
  // 0xAD
  Instruction {
    func: res_5_l,
    length: 2,
    t_cycles: 8,
    description: "RES 5, L",
  },
  // 0xAE
  Instruction {
    func: res_5_mhl,
    length: 2,
    t_cycles: 16,
    description: "RES 5, [HL]",
  },
  // 0xAF
  Instruction {
    func: res_5_a,
    length: 2,
    t_cycles: 8,
    description: "RES 5, A",
  },
  // 0xB0
  Instruction {
    func: res_6_b,
    length: 2,
    t_cycles: 8,
    description: "RES 6, B",
  },
  // 0xB1
  Instruction {
    func: res_6_c,
    length: 2,
    t_cycles: 8,
    description: "RES 6, C",
  },
  // 0xB2
  Instruction {
    func: res_6_d,
    length: 2,
    t_cycles: 8,
    description: "RES 6, D",
  },
  // 0xB3
  Instruction {
    func: res_6_e,
    length: 2,
    t_cycles: 8,
    description: "RES 6, E",
  },
  // 0xB4
  Instruction {
    func: res_6_h,
    length: 2,
    t_cycles: 8,
    description: "RES 6, H",
  },
  // 0xB5
  Instruction {
    func: res_6_l,
    length: 2,
    t_cycles: 8,
    description: "RES 6, L",
  },
  // 0xB6
  Instruction {
    func: res_6_mhl,
    length: 2,
    t_cycles: 16,
    description: "RES 6, [HL]",
  },
  // 0xB7
  Instruction {
    func: res_6_a,
    length: 2,
    t_cycles: 8,
    description: "RES 6, A",
  },
  // 0xB8
  Instruction {
    func: res_7_b,
    length: 2,
    t_cycles: 8,
    description: "RES 7, B",
  },
  // 0xB9
  Instruction {
    func: res_7_c,
    length: 2,
    t_cycles: 8,
    description: "RES 7, C",
  },
  // 0xBA
  Instruction {
    func: res_7_d,
    length: 2,
    t_cycles: 8,
    description: "RES 7, D",
  },
  // 0xBB
  Instruction {
    func: res_7_e,
    length: 2,
    t_cycles: 8,
    description: "RES 7, E",
  },
  // 0xBC
  Instruction {
    func: res_7_h,
    length: 2,
    t_cycles: 8,
    description: "RES 7, H",
  },
  // 0xBD
  Instruction {
    func: res_7_l,
    length: 2,
    t_cycles: 8,
    description: "RES 7, L",
  },
  // 0xBE
  Instruction {
    func: res_7_mhl,
    length: 2,
    t_cycles: 16,
    description: "RES 7, [HL]",
  },
  // 0xBF
  Instruction {
    func: res_7_a,
    length: 2,
    t_cycles: 8,
    description: "RES 7, A",
  },
  // 0xC0
  Instruction {
    func: set_0_b,
    length: 2,
    t_cycles: 8,
    description: "SET 0, B",
  },
  // 0xC1
  Instruction {
    func: set_0_c,
    length: 2,
    t_cycles: 8,
    description: "SET 0, C",
  },
  // 0xC2
  Instruction {
    func: set_0_d,
    length: 2,
    t_cycles: 8,
    description: "SET 0, D",
  },
  // 0xC3
  Instruction {
    func: set_0_e,
    length: 2,
    t_cycles: 8,
    description: "SET 0, E",
  },
  // 0xC4
  Instruction {
    func: set_0_h,
    length: 2,
    t_cycles: 8,
    description: "SET 0, H",
  },
  // 0xC5
  Instruction {
    func: set_0_l,
    length: 2,
    t_cycles: 8,
    description: "SET 0, L",
  },
  // 0xC6
  Instruction {
    func: set_0_mhl,
    length: 2,
    t_cycles: 16,
    description: "SET 0, [HL]",
  },
  // 0xC7
  Instruction {
    func: set_0_a,
    length: 2,
    t_cycles: 8,
    description: "SET 0, A",
  },
  // 0xC8
  Instruction {
    func: set_1_b,
    length: 2,
    t_cycles: 8,
    description: "SET 1, B",
  },
  // 0xC9
  Instruction {
    func: set_1_c,
    length: 2,
    t_cycles: 8,
    description: "SET 1, C",
  },
  // 0xCA
  Instruction {
    func: set_1_d,
    length: 2,
    t_cycles: 8,
    description: "SET 1, D",
  },
  // 0xCB
  Instruction {
    func: set_1_e,
    length: 2,
    t_cycles: 8,
    description: "SET 1, E",
  },
  // 0xCC
  Instruction {
    func: set_1_h,
    length: 2,
    t_cycles: 8,
    description: "SET 1, H",
  },
  // 0xCD
  Instruction {
    func: set_1_l,
    length: 2,
    t_cycles: 8,
    description: "SET 1, L",
  },
  // 0xCE
  Instruction {
    func: set_1_mhl,
    length: 2,
    t_cycles: 16,
    description: "SET 1, [HL]",
  },
  // 0xCF
  Instruction {
    func: set_1_a,
    length: 2,
    t_cycles: 8,
    description: "SET 1, A",
  },
  // 0xD0
  Instruction {
    func: set_2_b,
    length: 2,
    t_cycles: 8,
    description: "SET 2, B",
  },
  // 0xD1
  Instruction {
    func: set_2_c,
    length: 2,
    t_cycles: 8,
    description: "SET 2, C",
  },
  // 0xD2
  Instruction {
    func: set_2_d,
    length: 2,
    t_cycles: 8,
    description: "SET 2, D",
  },
  // 0xD3
  Instruction {
    func: set_2_e,
    length: 2,
    t_cycles: 8,
    description: "SET 2, E",
  },
  // 0xD4
  Instruction {
    func: set_2_h,
    length: 2,
    t_cycles: 8,
    description: "SET 2, H",
  },
  // 0xD5
  Instruction {
    func: set_2_l,
    length: 2,
    t_cycles: 8,
    description: "SET 2, L",
  },
  // 0xD6
  Instruction {
    func: set_2_mhl,
    length: 2,
    t_cycles: 16,
    description: "SET 2, [HL]",
  },
  // 0xD7
  Instruction {
    func: set_2_a,
    length: 2,
    t_cycles: 8,
    description: "SET 2, A",
  },
  // 0xD8
  Instruction {
    func: set_3_b,
    length: 2,
    t_cycles: 8,
    description: "SET 3, B",
  },
  // 0xD9
  Instruction {
    func: set_3_c,
    length: 2,
    t_cycles: 8,
    description: "SET 3, C",
  },
  // 0xDA
  Instruction {
    func: set_3_d,
    length: 2,
    t_cycles: 8,
    description: "SET 3, D",
  },
  // 0xDB
  Instruction {
    func: set_3_e,
    length: 2,
    t_cycles: 8,
    description: "SET 3, E",
  },
  // 0xDC
  Instruction {
    func: set_3_h,
    length: 2,
    t_cycles: 8,
    description: "SET 3, H",
  },
  // 0xDD
  Instruction {
    func: set_3_l,
    length: 2,
    t_cycles: 8,
    description: "SET 3, L",
  },
  // 0xDE
  Instruction {
    func: set_3_mhl,
    length: 2,
    t_cycles: 16,
    description: "SET 3, [HL]",
  },
  // 0xDF
  Instruction {
    func: set_3_a,
    length: 2,
    t_cycles: 8,
    description: "SET 3, A",
  },
  // 0xE0
  Instruction {
    func: set_4_b,
    length: 2,
    t_cycles: 8,
    description: "SET 4, B",
  },
  // 0xE1
  Instruction {
    func: set_4_c,
    length: 2,
    t_cycles: 8,
    description: "SET 4, C",
  },
  // 0xE2
  Instruction {
    func: set_4_d,
    length: 2,
    t_cycles: 8,
    description: "SET 4, D",
  },
  // 0xE3
  Instruction {
    func: set_4_e,
    length: 2,
    t_cycles: 8,
    description: "SET 4, E",
  },
  // 0xE4
  Instruction {
    func: set_4_h,
    length: 2,
    t_cycles: 8,
    description: "SET 4, H",
  },
  // 0xE5
  Instruction {
    func: set_4_l,
    length: 2,
    t_cycles: 8,
    description: "SET 4, L",
  },
  // 0xE6
  Instruction {
    func: set_4_mhl,
    length: 2,
    t_cycles: 16,
    description: "SET 4, [HL]",
  },
  // 0xE7
  Instruction {
    func: set_4_a,
    length: 2,
    t_cycles: 8,
    description: "SET 4, A",
  },
  // 0xE8
  Instruction {
    func: set_5_b,
    length: 2,
    t_cycles: 8,
    description: "SET 5, B",
  },
  // 0xE9
  Instruction {
    func: set_5_c,
    length: 2,
    t_cycles: 8,
    description: "SET 5, C",
  },
  // 0xEA
  Instruction {
    func: set_5_d,
    length: 2,
    t_cycles: 8,
    description: "SET 5, D",
  },
  // 0xEB
  Instruction {
    func: set_5_e,
    length: 2,
    t_cycles: 8,
    description: "SET 5, E",
  },
  // 0xEC
  Instruction {
    func: set_5_h,
    length: 2,
    t_cycles: 8,
    description: "SET 5, H",
  },
  // 0xED
  Instruction {
    func: set_5_l,
    length: 2,
    t_cycles: 8,
    description: "SET 5, L",
  },
  // 0xEE
  Instruction {
    func: set_5_mhl,
    length: 2,
    t_cycles: 16,
    description: "SET 5, [HL]",
  },
  // 0xEF
  Instruction {
    func: set_5_a,
    length: 2,
    t_cycles: 8,
    description: "SET 5, A",
  },
  // 0xF0
  Instruction {
    func: set_6_b,
    length: 2,
    t_cycles: 8,
    description: "SET 6, B",
  },
  // 0xF1
  Instruction {
    func: set_6_c,
    length: 2,
    t_cycles: 8,
    description: "SET 6, C",
  },
  // 0xF2
  Instruction {
    func: set_6_d,
    length: 2,
    t_cycles: 8,
    description: "SET 6, D",
  },
  // 0xF3
  Instruction {
    func: set_6_e,
    length: 2,
    t_cycles: 8,
    description: "SET 6, E",
  },
  // 0xF4
  Instruction {
    func: set_6_h,
    length: 2,
    t_cycles: 8,
    description: "SET 6, H",
  },
  // 0xF5
  Instruction {
    func: set_6_l,
    length: 2,
    t_cycles: 8,
    description: "SET 6, L",
  },
  // 0xF6
  Instruction {
    func: set_6_mhl,
    length: 2,
    t_cycles: 16,
    description: "SET 6, [HL]",
  },
  // 0xF7
  Instruction {
    func: set_6_a,
    length: 2,
    t_cycles: 8,
    description: "SET 6, A",
  },
  // 0xF8
  Instruction {
    func: set_7_b,
    length: 2,
    t_cycles: 8,
    description: "SET 7, B",
  },
  // 0xF9
  Instruction {
    func: set_7_c,
    length: 2,
    t_cycles: 8,
    description: "SET 7, C",
  },
  // 0xFA
  Instruction {
    func: set_7_d,
    length: 2,
    t_cycles: 8,
    description: "SET 7, D",
  },
  // 0xFB
  Instruction {
    func: set_7_e,
    length: 2,
    t_cycles: 8,
    description: "SET 7, E",
  },
  // 0xFC
  Instruction {
    func: set_7_h,
    length: 2,
    t_cycles: 8,
    description: "SET 7, H",
  },
  // 0xFD
  Instruction {
    func: set_7_l,
    length: 2,
    t_cycles: 8,
    description: "SET 7, L",
  },
  // 0xFE
  Instruction {
    func: set_7_mhl,
    length: 2,
    t_cycles: 16,
    description: "SET 7, [HL]",
  },
  // 0xFF
  Instruction {
    func: set_7_a,
    length: 2,
    t_cycles: 8,
    description: "SET 7, A",
  },
];

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
