#include <instruction.h>

instruction instructions[0x100] = {
    [0x00] = {IN_NOP, AM_IMP},
    [0x01] = {IN_LD, RT_BC, AM_D16},
};