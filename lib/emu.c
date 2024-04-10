#include <SDL.h>
#include <SDL_ttf.h>
#include <cart.h>
#include <cpu.h>
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

void delay(u32 ms) { SDL_Delay(ms); }

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

  cpu_init();

  ctx.running = true;
  ctx.paused = false;
  ctx.ticks = 0;

  while (ctx.running) {
    if (ctx.paused) {
      delay(10);
      continue;
    }
    if (!cpu_step()) {
      printf("CPU step failed...\n");
      return -3;
    }

    ctx.ticks++;
  }

  return 0;
}