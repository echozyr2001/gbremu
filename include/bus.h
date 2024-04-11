#ifndef BUS_H
#define BUS_H

#include <common.h>

u8 bus_read(u16 addr);
void bus_write(u16 addr, u8 data);

#endif