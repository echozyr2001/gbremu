mod sdl;

use clap::Parser;
use libemu::{GameBoy, CPU_FREQ, VISUAL_FREQ};
use sdl::SdlSystem;
use sdl2::{event::Event, keyboard::Keycode, pixels::PixelFormatEnum, Sdl};
use std::{cmp::max, env::set_var, path::Path};

const DEFAULT_ROM_PATH: &str = "/Users/echo/dev/gbremu/roms/pocket.gb";
const SCREEN_SCALE: f32 = 3.0;

fn main() {
  // init logger system
  set_var("RUST_LOG", "trace");
  env_logger::init();

  // parse command line arguments
  let args = Args::parse();

  let path = Path::new(&args.rom_path);
  if args.rom_path == DEFAULT_ROM_PATH && !path.exists() {
    println!("No ROM file provided, please provide one using the [ROM_PATH] argument");
    return;
  }

  let gb = GameBoy::new();
  // gb.load_dmg();
  println!("========= name test =========\n gb description test");

  let mut emu = Emulator::new(gb);
  emu.start(SCREEN_SCALE);
  emu.load_cart(Some(&args.rom_path));

  emu.run();
}

struct Emulator {
  system: GameBoy,
  sdl: Option<SdlSystem>,
  title: String,

  logic_frequency: u32,
  visual_frequency: f32,

  next_tick_time: f32,
  next_tick_time_i: u32,

  cart_path: String,
}

impl Emulator {
  fn new(system: GameBoy) -> Self {
    Self {
      system,
      sdl: None,
      title: String::from("GameBoy Emulator"),

      logic_frequency: CPU_FREQ,
      visual_frequency: VISUAL_FREQ,

      next_tick_time: 0.0,
      next_tick_time_i: 0,

      cart_path: String::from("invalid"),
    }
  }

  #[cfg(not(feature = "slow"))]
  pub fn start_base(&mut self) {}

  #[cfg(feature = "slow")]
  pub fn start_base(&mut self) {
    self.logic_frequency = 100;
  }

  fn start(&mut self, screen_scale: f32) {
    self.start_base();

    let sdl = sdl2::init().unwrap();

    self.start_graphics(&sdl, screen_scale);
  }

  fn start_graphics(&mut self, sdl: &Sdl, screen_scale: f32) {
    self.sdl = Some(SdlSystem::new(
      sdl,
      &self.title,
      self.system.display_width() as u32,
      self.system.display_height() as u32,
      screen_scale,
    ));
  }

  fn load_cart(&mut self, path: Option<&str>) {
    let cart_path = path.unwrap_or(&self.cart_path);

    self.system.load_cart_file(cart_path);
  }

  fn run(&mut self) {
    let (width, height) = (self.system.display_width(), self.system.display_height());

    self.sdl.as_mut().unwrap().canvas.present();

    let texture_creator = self.sdl.as_mut().unwrap().canvas.texture_creator();

    let mut texture = texture_creator
      .create_texture_streaming(PixelFormatEnum::RGB24, width as u32, height as u32)
      .unwrap();

    // 上一个时钟周期中未完成的周期数
    let mut pending_cycles = 0u32;

    'main: loop {
      while let Some(event) = self.sdl.as_mut().unwrap().event_pump.poll_event() {
        match event {
          Event::KeyDown {
            keycode: Some(Keycode::Escape),
            ..
          } => break 'main,
          Event::KeyDown {
            keycode: Some(Keycode::R),
            ..
          } => println!("Reset"),
          Event::KeyDown {
            keycode: Some(Keycode::I),
            ..
          } => println!("Save image"),
          _ => {},
        }
      }

      let current_time = self.sdl.as_mut().unwrap().timer_subsystem.ticks();

      if current_time >= self.next_tick_time_i {
        let mut counter_cycles = pending_cycles;
        let mut last_frame = self.system.ppu_frame();
        let mut frame_dirty = false;

        let cycle_limit = (self.logic_frequency as f32 / self.visual_frequency).round() as u32;

        loop {
          if counter_cycles >= cycle_limit {
            pending_cycles = counter_cycles - cycle_limit;
            break;
          }

          counter_cycles += self.system.cycle() as u32;

          if self.system.ppu_frame() != last_frame {
            let frame_buffer = self.system.frame_buffer().as_ref();
            texture.update(None, frame_buffer, width * 3).unwrap();

            last_frame = self.system.ppu_frame();
            frame_dirty = true;
          }
        }

        if frame_dirty {
          self.sdl.as_mut().unwrap().canvas.clear();

          self
            .sdl
            .as_mut()
            .unwrap()
            .canvas
            .copy(&texture, None, None)
            .unwrap();

          self.sdl.as_mut().unwrap().canvas.present();
        }

        if self.next_tick_time == 0.0 {
          self.next_tick_time = current_time as f32;
        }
        let mut ticks = ((current_time as f32 - self.next_tick_time)
          / ((1.0 / self.visual_frequency) * 1000.0))
          .ceil() as u8;
        ticks = max(ticks, 1);

        self.next_tick_time += (1000.0 / self.visual_frequency) * ticks as f32;
        self.next_tick_time_i = self.next_tick_time.ceil() as u32;
      }

      let current_time = self.sdl.as_mut().unwrap().timer_subsystem.ticks();

      let pending_time = self.next_tick_time_i.saturating_sub(current_time);

      self
        .sdl
        .as_mut()
        .unwrap()
        .timer_subsystem
        .delay(pending_time);
    }
  }
}

#[derive(Parser, Debug)]
struct Args {
  #[arg(default_value_t = String::from(DEFAULT_ROM_PATH), help = "Path to the ROM file to be loaded")]
  rom_path: String,
}
