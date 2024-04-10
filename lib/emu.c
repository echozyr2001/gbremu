#include <SDL.h>
#include <SDL_ttf.h>
#include <cart.h>
#include <emu.h>

/*
  emu components

  |Cart|        game cartridge
  |CPU|         central processing unit
  |Address Bus| address bus
  |PPU|         picture processing unit
  |Timer|       timer

*/

static emu_context ctx;

int emu_run(int argc, char *argv[]) {
  if (argc < 2) {
    printf("Usage: gbemu <ROM file>\n");
    return -1;
  }

  if (!cart_load(argv[1])) {
    printf("Failed to load ROM file: %s\n", argv[1]);
    return -2;
  }

  printf("Cart loaded...\n");

  SDL_Init(SDL_INIT_VIDEO);
  printf("SDL initialized...\n");
  TTF_Init();
  printf("TTF initialized...\n");

  return 0;
}