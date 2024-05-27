pub mod sdl;

use clap::Parser;
use libemu::{
  error::Error,
  gb::GameBoy,
  pad::PadKey,
  soc::ppu::palette::PaletteInfo,
  util::{replace_ext, write_file},
};
use sdl::SdlSystem;
use sdl2::{event::Event, keyboard::Keycode, pixels::PixelFormatEnum, Sdl};
use std::{cmp::max, env::set_var, path::Path};

const SCREEN_SCALE: f32 = 3.0;
const STORE_RATE: u8 = 5;
// const DEFAULT_ROM_PATH: &str = "../../res/roms/demo/pocket.gb";
const DEFAULT_ROM_PATH: &str = "../../res/roms/game/thebouncingball.gb";

pub struct Emulator {
  system: GameBoy,
  sdl: Option<SdlSystem>,
  rom_path: String,
  ram_path: String,
  dir_path: String,
  logic_frequency: u32,
  visual_frequency: f32,
  next_tick_time: f32,
  next_tick_time_i: u32,
  fast: bool,
  palettes: [PaletteInfo; 7],
  palette_index: usize,
}

impl Emulator {
  pub fn new(system: GameBoy) -> Self {
    Self {
      system,
      sdl: None,
      rom_path: String::from("invalid"),
      ram_path: String::from("invalid"),
      dir_path: String::from("invalid"),
      logic_frequency: GameBoy::CPU_FREQ,
      visual_frequency: GameBoy::VISUAL_FREQ,
      next_tick_time: 0.0,
      next_tick_time_i: 0,
      fast: false,
      palettes: [
        PaletteInfo::new(
          "basic",
          [
            [0xff, 0xff, 0xff],
            [0xc0, 0xc0, 0xc0],
            [0x60, 0x60, 0x60],
            [0x00, 0x00, 0x00],
          ],
        ),
        PaletteInfo::new(
          "hogwards",
          [
            [0xb6, 0xa5, 0x71],
            [0x8b, 0x7e, 0x56],
            [0x55, 0x4d, 0x35],
            [0x20, 0x1d, 0x13],
          ],
        ),
        PaletteInfo::new(
          "christmas",
          [
            [0xe8, 0xe7, 0xdf],
            [0x8b, 0xab, 0x95],
            [0x9e, 0x5c, 0x5e],
            [0x53, 0x4d, 0x57],
          ],
        ),
        PaletteInfo::new(
          "goldsilver",
          [
            [0xc5, 0xc6, 0x6d],
            [0x97, 0xa1, 0xb0],
            [0x58, 0x5e, 0x67],
            [0x23, 0x52, 0x29],
          ],
        ),
        PaletteInfo::new(
          "pacman",
          [
            [0xff, 0xff, 0x00],
            [0xff, 0xb8, 0x97],
            [0x37, 0x32, 0xff],
            [0x00, 0x00, 0x00],
          ],
        ),
        PaletteInfo::new(
          "mariobros",
          [
            [0xf7, 0xce, 0xc3],
            [0xcc, 0x9e, 0x22],
            [0x92, 0x34, 0x04],
            [0x00, 0x00, 0x00],
          ],
        ),
        PaletteInfo::new(
          "pokemon",
          [
            [0xf8, 0x78, 0x00],
            [0xb8, 0x60, 0x00],
            [0x78, 0x38, 0x00],
            [0x00, 0x00, 0x00],
          ],
        ),
      ],
      palette_index: 0,
    }
  }

  pub fn start(&mut self, screen_scale: f32) {
    let sdl = sdl2::init().unwrap();
    self.start_graphics(&sdl, screen_scale);
  }

  pub fn start_graphics(&mut self, sdl: &Sdl, screen_scale: f32) {
    self.sdl = Some(SdlSystem::new(
      sdl,
      "GBREMU",
      self.system.display_width() as u32,
      self.system.display_height() as u32,
      screen_scale,
    ));
  }

  pub fn load_cart(&mut self, path: Option<&str>) -> Result<(), Error> {
    let rom_path: &str = path.unwrap_or(&self.rom_path);
    let ram_path = replace_ext(rom_path, "sav").unwrap_or_else(|| "invalid".to_string());
    let cart = self.system.load_cart_file(
      rom_path,
      if Path::new(&ram_path).exists() {
        Some(&ram_path)
      } else {
        None
      },
    )?;
    println!(
      "========= Cartridge =========\n{}\n=============================",
      cart.header()
    );
    if let Some(ref mut sdl) = self.sdl {
      sdl
        .window_mut()
        .set_title(format!("{} [{}]", "GBREMU", cart.title()).as_str())
        .unwrap();
    }
    self.rom_path = String::from(rom_path);
    self.ram_path = ram_path;
    self.dir_path = Path::new(&self.rom_path)
      .parent()
      .unwrap()
      .to_str()
      .unwrap()
      .to_string();
    Ok(())
  }

  pub fn reset(&mut self) -> Result<(), Error> {
    self.system.reset();
    self.system.load_dmg();
    self.load_cart(None)?;
    Ok(())
  }

  pub fn toggle_palette(&mut self) {
    self
      .system
      .ppu_mut()
      .set_palette_colors(self.palettes[self.palette_index].colors());
    self.palette_index = (self.palette_index + 1) % self.palettes.len();
  }

  pub fn toggle_fullscreen(&mut self) {
    let window = self.sdl.as_mut().unwrap().window_mut();
    if window.fullscreen_state() == sdl2::video::FullscreenType::Off {
      window
        .set_fullscreen(sdl2::video::FullscreenType::Desktop)
        .unwrap()
    } else {
      window
        .set_fullscreen(sdl2::video::FullscreenType::Off)
        .unwrap()
    }
  }

  pub fn run(&mut self) {
    let (width, height) = (self.system.display_width(), self.system.display_height());
    self.sdl.as_mut().unwrap().canvas.present();
    let texture_creator = self.sdl.as_mut().unwrap().canvas.texture_creator();
    let mut texture = texture_creator
      .create_texture_streaming(PixelFormatEnum::RGB24, width as u32, height as u32)
      .unwrap();

    let store_count = (self.visual_frequency * STORE_RATE as f32).round() as u32;

    // starts the variable that will control the number of cycles that
    // are going to move (because of overflow) from one tick to another
    let mut pending_cycles = 0u32;

    // allocates space for the loop ticks counter to be used in each
    // iteration cycle
    let mut counter = 0u32;

    'main: loop {
      counter = counter.wrapping_add(1);
      if counter % store_count == 0 && self.system.cart().has_battery() {
        let cart = self.system.cart();
        let ram_data = cart.ram();
        write_file(&self.ram_path, ram_data.inner()).unwrap();
      }

      while let Some(event) = self.sdl.as_mut().unwrap().event_pump.poll_event() {
        match event {
          Event::Quit { .. } => break 'main,
          Event::KeyDown {
            keycode: Some(Keycode::Escape),
            ..
          } => break 'main,
          Event::KeyDown {
            keycode: Some(Keycode::R),
            ..
          } => self.reset().unwrap(),
          Event::KeyDown {
            keycode: Some(Keycode::P),
            ..
          } => self.toggle_palette(),
          Event::KeyDown {
            keycode: Some(Keycode::E),
            ..
          } => {
            if !self.fast {
              self.fast = true;
              self.logic_frequency *= 8;
            }
          },
          Event::KeyUp {
            keycode: Some(Keycode::E),
            ..
          } => {
            if self.fast {
              self.fast = false;
              self.logic_frequency /= 8;
            }
          },
          Event::KeyDown {
            keycode: Some(Keycode::F),
            ..
          } => self.toggle_fullscreen(),
          Event::DropFile { filename, .. } => {
            self.system.reset();
            self.system.load_dmg();
            self.load_cart(Some(&filename)).unwrap();
          },
          Event::KeyDown {
            keycode: Some(keycode),
            ..
          } => {
            if let Some(key) = key_to_pad(keycode) {
              self.system.key_press(key)
            }
          },
          Event::KeyUp {
            keycode: Some(keycode),
            ..
          } => {
            if let Some(key) = key_to_pad(keycode) {
              self.system.key_lift(key)
            }
          },
          _ => (),
        }
      }

      let current_time = self.sdl.as_mut().unwrap().timer_subsystem.ticks();

      if current_time >= self.next_tick_time_i {
        // re-starts the counter cycles with the number of pending cycles
        // from the previous tick and the last frame with the system PPU
        // frame index to be overridden in case there's at least one new frame
        // being drawn in the current tick
        let mut counter_cycles = pending_cycles;
        let mut last_frame = self.system.ppu_frame();
        let mut frame_dirty = false;

        // calculates the number of cycles that are meant to be the target
        // for the current "tick" operation this is basically the current
        // logic frequency divided by the visual one, this operation also
        // takes into account the current Game Boy speed multiplier (GBC)
        let cycle_limit =
          (self.logic_frequency as f32 * 1_f32 / self.visual_frequency).round() as u32;

        loop {
          // limits the number of ticks to the typical number
          // of cycles expected for the current logic cycle
          if counter_cycles >= cycle_limit {
            pending_cycles = counter_cycles - cycle_limit;
            break;
          }

          // runs the Game Boy clock, this operation should
          // include the advance of both the CPU, PPU, APU
          // and any other frequency based component of the system
          counter_cycles += self.system.clock() as u32;

          // in case a new frame is available from the emulator
          // then the frame must be pushed into SDL for display
          if self.system.ppu_frame() != last_frame {
            // obtains the frame buffer of the Game Boy PPU and uses it
            // to update the stream texture, that will latter be copied
            // to the canvas
            {
              let mut ppu_mut = self.system.ppu_mut();
              let frame_buffer = ppu_mut.frame_buffer();
              texture.update(None, frame_buffer, width * 3).unwrap();
            }
            // obtains the index of the current PPU frame, this value
            // is going to be used to detect for new frame presence
            last_frame = self.system.ppu_frame();
            frame_dirty = true;
          }
        }

        // in case there's at least one new frame that was drawn during
        // during the current tick, then we need to flush it to the canvas,
        // this separation between texture creation and canvas flush prevents
        // resources from being over-used in situations where multiple frames
        // are generated during the same tick cycle
        if frame_dirty {
          // clears the graphics canvas, making sure that no garbage
          // pixel data remaining in the pixel buffer, not doing this would
          // create visual glitches in OSs like Mac OS X
          self.sdl.as_mut().unwrap().canvas.clear();

          // copies the texture that was created for the frame (during
          // the loop part of the tick) to the canvas
          self
            .sdl
            .as_mut()
            .unwrap()
            .canvas
            .copy(&texture, None, None)
            .unwrap();

          // presents the canvas effectively updating the screen
          // information presented to the user
          self.sdl.as_mut().unwrap().canvas.present();
        }

        // calculates the number of ticks that have elapsed since the
        // last draw operation, this is critical to be able to properly
        // operate the clock of the CPU in frame drop situations, meaning
        // a situation where the system resources are no able to emulate
        // the system on time and frames must be skipped (ticks > 1)
        if self.next_tick_time == 0.0 {
          self.next_tick_time = current_time as f32;
        }
        let mut ticks = ((current_time as f32 - self.next_tick_time)
          / ((1.0 / self.visual_frequency) * 1000.0))
          .ceil() as u8;
        ticks = max(ticks, 1);

        // in case the limited (speed) mode is set then we must calculate
        // a new next tick time reference, this is required to prevent the
        // machine from running too fast (eg: 50x)
        // if self.limited() {
        // updates the next update time reference to the current
        // time so that it can be used from game loop control
        self.next_tick_time += (1000.0 / self.visual_frequency) * ticks as f32;
        self.next_tick_time_i = self.next_tick_time.ceil() as u32;
        // }
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

#[derive(Parser)]
struct Args {
  #[arg(default_value_t = String::from(DEFAULT_ROM_PATH), help = "Path to the ROM file to be loaded")]
  rom_path: String,
}

fn main() {
  set_var("RUST_LOG", "info");
  env_logger::init();

  let args = Args::parse();

  let path = Path::new(&args.rom_path);
  if args.rom_path == DEFAULT_ROM_PATH && !path.exists() {
    println!("No ROM file provided, please provide one using the [ROM_PATH] argument");
    return;
  }

  let mut game_boy = GameBoy::new();
  game_boy.load_dmg();

  let mut emulator = Emulator::new(game_boy);
  emulator.start(SCREEN_SCALE);
  emulator.load_cart(Some(&args.rom_path)).unwrap();
  emulator.toggle_palette();

  emulator.run();
}

fn key_to_pad(keycode: Keycode) -> Option<PadKey> {
  match keycode {
    Keycode::Up => Some(PadKey::Up),
    Keycode::Down => Some(PadKey::Down),
    Keycode::Left => Some(PadKey::Left),
    Keycode::Right => Some(PadKey::Right),
    Keycode::Return => Some(PadKey::Start),
    Keycode::Return2 => Some(PadKey::Start),
    Keycode::Space => Some(PadKey::Select),
    Keycode::A => Some(PadKey::A),
    Keycode::S => Some(PadKey::B),
    _ => None,
  }
}
