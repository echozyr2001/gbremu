// use std::{
//   fmt::write,
//   process::{ExitCode, Termination},
//   time::Duration,
// };

// use sdl2::{event::Event, keyboard::Keycode, pixels::Color};

// fn main() -> Result<(), String> {
//   let sdl_context = sdl2::init()?;
//   let video_subsystem = sdl_context.video()?;

//   let window = video_subsystem
//     .window("rust-sdl2 demo", 800, 600)
//     .position_centered()
//     .build()
//     .map_err(|e| e.to_string())?;

//   let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
//   canvas.set_draw_color(Color::RGB(0, 255, 255)); // set background color
//   canvas.clear(); // clear canvas
//   canvas.present(); // display canvas

//   let mut event_pump = sdl_context.event_pump()?; // event_pump
//   'running: loop {
//     for event in event_pump.poll_iter() {
//       match event {
//         Event::Quit { .. }
//         | Event::KeyDown {
//           keycode: Some(Keycode::Escape),
//           ..
//         } => {
//           break 'running; // press ESC to exit
//         },
//         Event::KeyDown {
//           keycode: Some(key), ..
//         } => {
//           println!("Pressed {:?}", key);
//         },
//         Event::KeyUp {
//           keycode: Some(key), ..
//         } => {
//           println!("Released {:?}", key);
//         },
//         _ => {},
//       }
//     }

//     // update canvas
//     canvas.set_draw_color(Color::RGB(0, 255, 255));
//     canvas.clear();
//     canvas.present();
//     std::thread::sleep(Duration::from_millis(100)); // wait for 100ms
//   }
//   Ok(())
// }

mod cli;
mod sdl;

use anyhow::Result;
use clap::Parser;
use cli::Args;
use libemu::GameBoy;
use log::info;
use sdl::SdlSystem;
use sdl2::{event::Event, keyboard::Keycode};
use std::{
  env::set_var,
  path::Path,
  process::{ExitCode, Termination},
};

const DEFAULT_ROM_PATH: &str = "/Users/echo/dev/gbremu/roms/pocket.gb";
enum Exit {
  Success,
  Failure,
}

impl Termination for Exit {
  fn report(self) -> ExitCode {
    match self {
      Self::Success => ExitCode::SUCCESS,
      Self::Failure => ExitCode::FAILURE,
    }
  }
}

fn main() -> Exit {
  // init logger system
  set_var("RUST_LOG", "trace");
  env_logger::init();

  // parse command line arguments
  let args = Args::parse();

  let rom_path = if let Some(ref rom_path) = args.rom_path {
    let rom_path = Path::new(rom_path);
    if !rom_path.exists() {
      info!("File not found: {}", rom_path.display());
      return Exit::Failure;
    }
    rom_path
  } else {
    info!("Using default ROM path: {}", DEFAULT_ROM_PATH);
    Path::new(DEFAULT_ROM_PATH)
  };

  let cart = helper::cart(rom_path).unwrap();
  // mode is always dmg

  let mut gb = GameBoy::new();
  gb.load_cart(cart);

  loop {
    gb.cycle();
  }

  // match run() {
  //   Ok(_) => Exit::Success,
  //   Err(e) => {
  //     println!("{:?}", e);
  //     Exit::Failure
  //   },
  // }
}

fn run() -> Result<()> {
  // let boot = helper::boot(Path::new("/Users/echo/dev/gbremu/roms/boot/dmg_boot.bin"))?;

  // let mut emu = GameBoy::with(boot);
  // emu.load_cart(cart);
  // let emu = helper::b?;

  // loop {
  //   emu.cycle();
  // }

  let width = 160;
  let height = 144;

  let sdl = sdl2::init().unwrap();

  let mut sdl = SdlSystem::new(&sdl, "test", width, height, 3.0, true, false);

  // let texture_creater = sdl.canvas.texture_creator();

  // let texture = texture_creater
  //   .create_texture_streaming(PixelFormatEnum::RGB24, width as u32, height as u32)
  //   .unwrap();

  'main: loop {
    for event in sdl.event_pump.poll_iter() {
      match event {
        Event::Quit { .. } => break 'main,
        Event::KeyDown {
          keycode: Some(Keycode::Escape),
          ..
        } => break 'main,
        // Event::KeyDown {
        //   keycode: Some(Keycode::R),
        //   ..
        // } => self.reset().unwrap(),
        // Event::KeyDown {
        //   keycode: Some(Keycode::B),
        //   ..
        // } => self.benchmark(&Benchmark::default()),
        // Event::KeyDown {
        //   keycode: Some(Keycode::I),
        //   ..
        // } => self.save_image(&self.image_name(Some("png"), Some(&self.dir_path))),
        // Event::KeyDown {
        //   keycode: Some(Keycode::T),
        //   ..
        // } => self.toggle_audio(),
        // Event::KeyDown {
        //   keycode: Some(Keycode::P),
        //   ..
        // } => self.toggle_palette(),
        // Event::KeyDown {
        //   keycode: Some(Keycode::E),
        //   keymod,
        //   ..
        // } => {
        //   if !self.fast && (keymod & (Mod::LCTRLMOD | Mod::RCTRLMOD)) != Mod::NOMOD {
        //     self.fast = true;
        //     self.logic_frequency *= 8;
        //   }
        // },
        // Event::KeyUp {
        //   keycode: Some(Keycode::E),
        //   ..
        // } => {
        //   if self.fast {
        //     self.fast = false;
        //     self.logic_frequency /= 8;
        //   }
        // },
        // Event::KeyUp {
        //   keycode: Some(Keycode::LCtrl) | Some(Keycode::RCtrl),
        //   ..
        // } => {
        //   if self.fast {
        //     self.fast = false;
        //     self.logic_frequency /= 8;
        //   }
        // },
        // Event::KeyDown {
        //   keycode: Some(Keycode::F),
        //   keymod,
        //   ..
        // } => {
        //   if (keymod & (Mod::LCTRLMOD | Mod::RCTRLMOD)) != Mod::NOMOD {
        //     self.toggle_fullscreen()
        //   }
        // },
        // Event::KeyDown {
        //   keycode: Some(Keycode::Plus),
        //   ..
        // } => self.logic_frequency = self.logic_frequency.saturating_add(400000),
        // Event::KeyDown {
        //   keycode: Some(Keycode::Minus),
        //   ..
        // } => self.logic_frequency = self.logic_frequency.saturating_sub(400000),
        // Event::KeyDown {
        //   keycode: Some(keycode),
        //   keymod,
        //   ..
        // } => {
        //   match keycode {
        //     Keycode::Num0
        //     | Keycode::Num1
        //     | Keycode::Num2
        //     | Keycode::Num3
        //     | Keycode::Num4
        //     | Keycode::Num5
        //     | Keycode::Num6
        //     | Keycode::Num7
        //     | Keycode::Num8
        //     | Keycode::Num9 => {
        //       let file_path = self.save_name(
        //         keycode as u8 - Keycode::Num0 as u8,
        //         None,
        //         Some(&self.dir_path),
        //       );
        //       if (keymod & (Mod::LCTRLMOD | Mod::RCTRLMOD)) != Mod::NOMOD {
        //         self.save_state(&file_path);
        //       } else {
        //         self.load_state(&file_path);
        //       }
        //     },
        //     _ => {},
        //   }
        //   if let Some(key) = key_to_pad(keycode) {
        //     self.system.key_press(key)
        //   }
        // },
        // Event::KeyUp {
        //   keycode: Some(keycode),
        //   ..
        // } => {
        //   if let Some(key) = key_to_pad(keycode) {
        //     self.system.key_lift(key)
        //   }
        // },
        // Event::DropFile { filename, .. } => {
        //   if self.auto_mode {
        //     let mode = Cartridge::from_file(&filename).unwrap().gb_mode();
        //     self.system.set_mode(mode);
        //   }
        //   self.system.reset();
        //   self.system.load(true);
        //   self.load_rom(Some(&filename)).unwrap();
        // },
        _ => (),
      }
    }
  }

  Ok(())
}

mod helper {
  use std::{fs::File, io::Read, path::Path};

  use anyhow::Context;
  use libemu::hardware::cartridge::Cartridge;
  use log::{debug, info};

  // // pub fn emu() -> anyhow::Result<Emulator> {
  // //   Ok(Emulator::default())
  // // }

  // pub fn boot(path: &Path) -> anyhow::Result<Boot> {
  //   // Read boot rom
  //   let rom = {
  //     // Open boot rom file
  //     let mut file =
  //       File::open(path).with_context(|| format!("Filed to open: {}", path.display()))?;
  //     // Read boot rom into a buffer (must be exactly 0x100 bytes)
  //     let mut buf = [0u8; 0x100];
  //     file
  //       // Read exactly bytes form file
  //       .read_exact(&mut buf)
  //       .with_context(|| format!("Filed to read:{}", path.display()))?;
  //     let len = buf.len();
  //     debug!("read {} bytes from {}", len, path.display());
  //     buf
  //   };

  //   // Create boot rom
  //   let boot = Boot::from(&rom);
  //   info!("Loaded boot ROM");

  //   Ok(boot)
  // }

  pub fn cart(path: &Path) -> anyhow::Result<Cartridge> {
    let rom = {
      let file = File::open(path).with_context(|| format!("Filed to open: {}", path.display()))?;
      let mut buf = Vec::new();

      let len = file
        .take(0x0080_0000)
        .read_to_end(&mut buf)
        .with_context(|| format!("Filed to read:{}", path.display()))?;
      debug!("read {} bytes from {}", len, path.display());
      buf
    };

    let cart =
      Cartridge::new(rom).with_context(|| format!("Failed to load: {}", path.display()))?;
    info!("Loaded cartridge:\n{}", cart.header());

    Ok(cart)
  }
}

#[cfg(test)]
mod test {
  use log::debug;

  use super::*;

  #[test]
  fn test_main() {
    set_var("RUST_LOG", "debug");
    env_logger::init();

    let mut gb = GameBoy::new();
    let cart = helper::cart(Path::new("/Users/echo/dev/gbremu/roms/pocket.gb")).unwrap();

    gb.load_cart(cart);
    // gb.soc.cpu.read(0x0000);
    // gb.soc.cpu.read(0x3000);
    gb.soc.cpu.read(0x4000);
  }
}
