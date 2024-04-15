#ifndef CPU_H
#define CPU_H

#include <common.h>
#include <instruction.h>

typedef struct {
  u8 a, b, c, d, e, h, l, f;
  u16 pc, sp;
  // bool ime;
} registers_t;

typedef struct {
  registers_t regs;

  u16 fetch_data;
  u16 mem_dest;
  u8 cur_opcode;
  instruction *cur_inst;

  bool halted;
  bool stepping;
} cpu_context;

void cpu_init();
bool cpu_step();

#endif