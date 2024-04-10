#ifndef CART_H
#define CART_H

#include <common.h>

typedef struct {
  u8 entry[0x04]; // 0x0100-0x0103
  u8 logo[0x30];  // 0x0104-0x0133
  u8 title[0x10]; // 0x0134-0x0143
  // Manufacturer code // 0x013F-0x0142
  // u8 cgb_flag;         // 0x0143
  u16 new_lic_code;    // 0x0144-0x0145
  u8 sgb_flag;         // 0x0146
  u8 cart_type;        // 0x0147
  u8 rom_size;         // 0x0148
  u8 ram_size;         // 0x0149
  u8 dest_code;        // 0x014A
  u8 old_lic_code;     // 0x014B
  u8 version;          // 0x014C
  u8 checksum;         // 0x014D
  u16 global_checksum; // 0x014E-0x014F
} rom_header;

typedef struct {
  char filename[1024];
  u32 rom_size;
  u8 *rom_data;
  rom_header *header;
} cart_context;

bool cart_load(char *cart);

#endif