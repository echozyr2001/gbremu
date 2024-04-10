#include <cart.h>

static const char *CART_TYPES[] = {
    [0x00] = "ROM ONLY",
    [0x01] = "MBC1",
    [0x02] = "MBC1+RAM",
    [0x03] = "MBC1+RAM+BATTERY",
    [0x05] = "MBC2",
    [0x06] = "MBC2+BATTERY",
    [0x08] = "ROM+RAM",
    [0x09] = "ROM+RAM+BATTERY",
    [0x0B] = "MMM01",
    [0x0C] = "MMM01+RAM",
    [0x0D] = "MMM01+RAM+BATTERY",
    [0x0F] = "MBC3+TIMER+BATTERY",
    [0x10] = "MBC3+TIMER+RAM+BATTERY",
    [0x11] = "MBC3",
    [0x12] = "MBC3+RAM",
    [0x13] = "MBC3+RAM+BATTERY",
    [0x19] = "MBC5",
    [0x1A] = "MBC5+RAM",
    [0x1B] = "MBC5+RAM+BATTERY",
    [0x1C] = "MBC5+RUMBLE",
    [0x1D] = "MBC5+RUMBLE+RAM",
    [0x1E] = "MBC5+RUMBLE+RAM+BATTERY",
    [0x20] = "MBC6",
    [0x22] = "MBC7+SENSOR+RUMBLE+RAM+BATTERY",
    [0xFC] = "POCKER CAMERA",
    [0xFD] = "BANDAI TAMA5",
    [0xFE] = "HuC3",
    [0xFF] = "HuC1+RAM+BATTERY",
};

static cart_context ctx;

// TODO: Implement this function
const char *cart_lic_name() { return "UNKNOWN"; }

const char *cart_type_name() {
  if (ctx.header->cart_type <= 0xFF) {
    return CART_TYPES[ctx.header->cart_type];
  }
  return "UNKNOWN";
}

bool cart_load(char *cart) {
  FILE *fp = fopen(cart, "rb");
  if (!fp) {
    printf("Failed to open file %s\n", cart);
    return false;
  }

  // formated output to ctx.filename
  sprintf(ctx.filename, "%s", cart);
  printf("Opened file %s\n", ctx.filename);

  // repostiton a stream to the end of the file
  fseek(fp, 0, SEEK_END);
  // get the current position of a stream, which is the size of the file
  ctx.rom_size = ftell(fp);
  // reposition a stream to the beginning of the file
  rewind(fp);

  // this vale is in the heap
  ctx.rom_data = malloc(ctx.rom_size);
  fread(ctx.rom_data, ctx.rom_size, 1, fp);
  fclose(fp);

  // set the header to the address of the rom_data + 0x100
  ctx.header = (rom_header *)(ctx.rom_data + 0x100);

  printf("Cartridge Loaded:\n");
  printf("\t Title    : %s\n", ctx.header->title);
  printf("\t Type     : %2.2X (%s)\n", ctx.header->cart_type, cart_type_name());
  printf("\t ROM Size : %d KB\n", 32 << ctx.header->rom_size);
  printf("\t RAM Size : %2.2X\n", ctx.header->ram_size);
  // TODO: match new LIC code to name
  printf("\t LIC Code : %2.2X (%s)\n", ctx.header->new_lic_code,
         cart_lic_name());
  // TODO: Display Old LIC info
  printf("\t Old LIC Code : %2.2X\n", ctx.header->old_lic_code);
  printf("\t ROM Vers : %2.2X\n", ctx.header->version);

  // calculate the checksum
  u8 checksum = 0;
  for (u16 address = 0x134; address <= 0x14C; address++) {
    checksum = checksum - ctx.rom_data[address] - 1;
  }

  printf("\t Checksum : %2.2X (%s)\n", checksum,
         (checksum & 0xFF) == ctx.header->checksum ? "PASSED" : "FAILED");

  return true;
}