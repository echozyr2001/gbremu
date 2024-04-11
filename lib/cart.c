#include <cart.h>

const char *cart_lic_name();
const char *cart_type_name();
bool check_logo();

static cart_context ctx;

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

  if (!check_logo()) {
    printf("Invalid logo...\n");
    return -3;
  }

  printf("Cartridge Loaded:\n");
  printf("\t Title    : %s\n", ctx.header->title);
  printf("\t Type     : %2.2X (%s)\n", ctx.header->cart_type, cart_type_name());
  printf("\t ROM Size : %d KB\n", 32 << ctx.header->rom_size);
  printf("\t RAM Size : %2.2X\n", ctx.header->ram_size);
  printf("\t New LIC Code : %2.2X (%s)\n", ctx.header->new_lic_code,
         cart_lic_name());
  printf("\t Old LIC Code : %2.2X\n", ctx.header->old_lic_code);
  printf("\t ROM Version : %2.2X\n", ctx.header->version);

  // calculate the checksum
  u8 checksum = 0;
  for (u16 address = 0x134; address <= 0x14C; address++) {
    checksum = checksum - ctx.rom_data[address] - 1;
  }

  printf("\t Checksum : %2.2X (%s)\n", checksum,
         (checksum & 0xFF) == ctx.header->checksum ? "PASSED" : "FAILED");

  return true;
}

const char *cart_lic_name() {
  int status = ctx.header->old_lic_code == 0x33 ? 1 : 0;
  status = 2;
  switch (status) {
  case 0: { // old LIC code
    switch (ctx.header->old_lic_code) {
    case 0x00:
      return "none";
    case 0x01:
      return "Nintendo";
    case 0x08:
      return "Capcom";
    case 0x09:
      return "Hot-B";
    case 0x0A:
      return "Jaleco";
    case 0x0B:
      return "Coconuts";
    case 0x0C:
      return "Elite Systems";
    case 0x13:
      return "Electronic Arts";
    case 0x18:
      return "Hudsonsoft";
    case 0x19:
      return "ITC Entertainment";
    case 0x1A:
      return "Yanoman";
    case 0x1D:
      return "Clary";
    case 0x1F:
      return "Virgin";
    case 0x20:
      return "KSS";
    case 0x24:
      return "PCM Complete";
    case 0x25:
      return "San-X";
    case 0x28:
      return "Kotobuki Systems";
    case 0x29:
      return "Seta";
    case 0x30:
      return "Infogrames";
    case 0x31:
      return "Nintendo";
    case 0x32:
      return "Bandai";
    case 0x33:
      UN_REACHABLE;
    case 0x34:
      return "Konami";
    case 0x35:
      return "Hector";
    case 0x38:
      return "Capcom";
    case 0x39:
      return "Banpresto";
    case 0x3C:
      return "*Entertainment I";
    case 0x3E:
      return "Gremlin";
    case 0x41:
      return "Ubisoft";
    case 0x42:
      return "Atlus";
    case 0x44:
      return "Malibu";
    case 0x46:
      return "Angel";
    case 0x47:
      return "Spectrum Holoby";
    case 0x49:
      return "Irem";
    case 0x4A:
      return "Virgin";
    case 0x4D:
      return "Malibu";
    case 0x4F:
      return "U.S. Gold";
    case 0x50:
      return "Absolute";
    case 0x51:
      return "Acclaim";
    case 0x52:
      return "Activision";
    case 0x53:
      return "American Sammy";
    case 0x54:
      return "GameTek";
    case 0x55:
      return "Park Place";
    case 0x56:
      return "LJN";
    case 0x57:
      return "Matchbox";
    case 0x59:
      return "Milton Bradley";
    case 0x5A:
      return "Mindscape";
    case 0x5B:
      return "Romstar";
    case 0x5C:
      return "Naxat Soft";
    case 0x5D:
      return "Tradewest";
    case 0x60:
      return "Titus";
    case 0x61:
      return "Virgin";
    case 0x67:
      return "Ocean";
    case 0x69:
      return "Electronic Arts";
    case 0x6E:
      return "Elite Systems";
    case 0x6F:
      return "Electro Brain";
    case 0x70:
      return "Infogrames";
    case 0x71:
      return "Interplay";
    case 0x72:
      return "Broderbund";
    case 0x73:
      return "Sculptered Soft";
    case 0x75:
      return "The Sales Curve";
    case 0x78:
      return "T*HQ";
    case 0x79:
      return "Accolade";
    case 0x7A:
      return "Triffix Entertainment";
    case 0x7C:
      return "Microprose";
    case 0x7F:
      return "Kemco";
    case 0x80:
      return "Misawa Entertainment";
    case 0x83:
      return "Lozc";
    case 0x86:
      return "Tokuma Shoten Intermedia";
    case 0x8B:
      return "Bullet-Proof Software";
    case 0x8C:
      return "Vic Tokai";
    case 0x8E:
      return "Ape";
    case 0x8F:
      return "I'Max";
    case 0x91:
      return "Chun Soft";
    case 0x92:
      return "Video System";
    case 0x93:
      return "Tsuburava";
    case 0x95:
      return "Varie";
    case 0x96:
      return "Yonezawa/S'pal";
    case 0x97:
      return "Kaneko";
    case 0x99:
      return "Arc";
    case 0x9A:
      return "Nihon Bussan";
    case 0x9B:
      return "Tecmo";
    case 0x9C:
      return "Imagineer";
    case 0x9D:
      return "Banpresto";
    case 0x9F:
      return "Nova";
    case 0xA1:
      return "Hori Electric";
    case 0xA2:
      return "Bandai";
    case 0xA4:
      return "Konami";
    case 0xA6:
      return "Kawada";
    case 0xA7:
      return "Takara";
    case 0xA9:
      return "Technos Japan";
    case 0xAA:
      return "Broderbund";
    case 0xAC:
      return "Toei Animation";
    case 0xAD:
      return "Toho";
    case 0xAF:
      return "Namco";
    case 0xB0:
      return "Acclaim";
    case 0xB1:
      return "Ascii or Nexoft";
    case 0xB2:
      return "Bandai";
    case 0xB4:
      return "Enix";
    case 0xB6:
      return "Hal";
    case 0xB7:
      return "SNK";
    case 0xB9:
      return "Pony Canyon";
    case 0xBA:
      return "*Culture Brain";
    case 0xBB:
      return "Sunsoft";
    case 0xBD:
      return "Sony Imagesoft";
    case 0xBF:
      return "Sammy";
    case 0xC0:
      return "Taito";
    case 0xC2:
      return "Kemco";
    case 0xC3:
      return "Squaresoft";
    case 0xC4:
      return "Tokuma Shoten Intermedia";
    case 0xC5:
      return "Data East";
    case 0xC6:
      return "Tonkin House";
    case 0xC8:
      return "Koei";
    case 0xC9:
      return "UFL";
    case 0xCA:
      return "Ultra";
    case 0xCB:
      return "Vap";
    case 0xCC:
      return "Use";
    case 0xCD:
      return "Meldac";
    case 0xCE:
      return "*Pony Canyon or";
    case 0xCF:
      return "Angel";
    case 0xD0:
      return "Taito";
    case 0xD1:
      return "Sofel";
    case 0xD2:
      return "Quest";
    case 0xD3:
      return "Sigma Enterprises";
    case 0xD4:
      return "ASK Kodansha";
    case 0xD6:
      return "Naxat Soft";
    case 0xD7:
      return "Copya Systems";
    case 0xD9:
      return "Banpresto";
    case 0xDA:
      return "Tomy";
    case 0xDB:
      return "LJN";
    case 0xDD:
      return "NCS";
    case 0xDE:
      return "Human";
    case 0xDF:
      return "Altron";
    case 0xE0:
      return "Jaleco";
    case 0xE1:
      return "Towachiki";
    case 0xE2:
      return "yutaka";
    case 0xE3:
      return "Varie";
    case 0xE5:
      return "Epoch";
    case 0xE7:
      return "Athena";
    case 0xE8:
      return "Asmik";
    case 0xE9:
      return "Natsume";
    case 0xEA:
      return "King Records";
    case 0xEB:
      return "Atlus";
    case 0xEC:
      return "Epic/Sony Records";
    case 0xEE:
      return "IGS";
    case 0xF0:
      return "A Wave";
    case 0xF3:
      return "Extreme Entertainment";
    case 0xFF:
      return "LJN";
    default:
      return "UNKNOWN";
    }
  }
  case 1: { // new LIC code
    switch (ctx.header->new_lic_code) {
    case 0x3030:
      return "none";
    case 0x3130:
      return "Nintendo R&D1";
    case 0x3830:
      return "Capcom";
    case 0x3331:
      return "Electronic Arts";
    case 0x3831:
      return "Hudson Soft";
    case 0x3931:
      return "b-ai";
    case 0x3032:
      return "kss";
    case 0x3232:
      return "pow";
    case 0x3432:
      return "PCM Complete";
    case 0x3532:
      return "san-x";
    case 0x3832:
      return "Kemco Japan";
    case 0x3932:
      return "seta";
    case 0x3033:
      return "Viacom";
    case 0x3133:
      return "Nintendo";
    case 0x3233:
      return "Bandai";
    case 0x3333:
      return "Ocean/Acclaim";
    case 0x3433:
      return "Konami";
    case 0x3533:
      return "Hector";
    case 0x3733:
      return "Tatio";
    case 0x3833:
      return "Hudson";
    case 0x3933:
      return "Banpresto";
    case 0x3134:
      return "Ubi Soft";
    case 0x3234:
      return "Atlus";
    case 0x3434:
      return "Malibu";
    case 0x3634:
      return "angel";
    case 0x3734:
      return "Bullet-Proof";
    case 0x3934:
      return "irem";
    case 0x3035:
      return "Absolute";
    case 0x3135:
      return "Acclaim";
    case 0x3235:
      return "Activision";
    case 0x3335:
      return "American sammy";
    case 0x3435:
      return "Konami";
    case 0x3535:
      return "Hi tech entertainment";
    case 0x3635:
      return "LJN";
    case 0x3735:
      return "Matchbox";
    case 0x3835:
      return "Mattel";
    case 0x3935:
      return "Milton Bradley";
    case 0x3036:
      return "Titus";
    case 0x3136:
      return "Virgin";
    case 0x3436:
      return "LucasArts";
    case 0x3736:
      return "Ocean";
    case 0x3936:
      return "Electronic Arts";
    case 0x3037:
      return "Infogrames";
    case 0x3137:
      return "Interplay";
    case 0x3237:
      return "Broderbund";
    case 0x3337:
      return "sculptured";
    case 0x3537:
      return "sci";
    case 0x3837:
      return "THQ";
    case 0x3937:
      return "Accolade";
    case 0x3038:
      return "misawa";
    case 0x3338:
      return "lozc";
    case 0x3638:
      return "tokuma shoten i*";
    case 0x3738:
      return "tsukuda ori*";
    case 0x3139:
      return "Chunsoft";
    case 0x3239:
      return "Video system";
    case 0x3339:
      return "Ocean/Acclaim";
    case 0x3539:
      return "Varie";
    case 0x3639:
      return "Yonezawa/s'pal";
    case 0x3739:
      return "Kaneko";
    case 0x3939:
      return "Pack in soft";
    case 0x3441:
      return "Konami(Yu-Gi-Oh!)";
    default:
      return "UNKNOWN";
    }
  }
  default:
    UN_REACHABLE;
  }
}

const char *cart_type_name() {
  switch (ctx.header->cart_type) {
  case 0x00:
    return "ROM ONLY";
  case 0x01:
    return "MBC1";
  case 0x02:
    return "MBC1+RAM";
  case 0x03:
    return "MBC1+RAM+BATTERY";
  case 0x05:
    return "MBC2";
  case 0x06:
    return "MBC2+BATTERY";
  case 0x08:
    return "ROM+RAM";
  case 0x09:
    return "ROM+RAM+BATTERY";
  case 0x0B:
    return "MMM01";
  case 0x0C:
    return "MMM01+RAM";
  case 0x0D:
    return "MMM01+RAM+BATTERY";
  case 0x0F:
    return "MBC3+TIMER+BATTERY";
  case 0x10:
    return "MBC3+TIMER+RAM+BATTERY";
  case 0x11:
    return "MBC3";
  case 0x12:
    return "MBC3+RAM";
  case 0x13:
    return "MBC3+RAM+BATTERY";
  case 0x19:
    return "MBC5";
  case 0x1A:
    return "MBC5+RAM";
  case 0x1B:
    return "MBC5+RAM+BATTERY";
  case 0x1C:
    return "MBC5+RUMBLE";
  case 0x1D:
    return "MBC5+RUMBLE+RAM";
  case 0x1E:
    return "MBC5+RUMBLE+RAM+BATTERY";
  case 0x20:
    return "MBC6";
  case 0x22:
    return "MBC7+SENSOR+RUMBLE+RAM+BATTERY";
  case 0xFC:
    return "POCKER CAMERA";
  case 0xFD:
    return "BANDAI TAMA5";
  case 0xFE:
    return "HuC3";
  case 0xFF:
    return "HuC1+RAM+BATTERY";
  default:
    return "UNKNOWN";
  }
}

bool check_logo() {
  for (int i = 0; i < 0x30; i++) {
    if (ctx.header->logo[i] != expected_logo[i]) {
      return false;
    }
  }
  return true;
}
