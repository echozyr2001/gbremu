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

use anyhow::Result;
use std::{
  env::set_var,
  path::Path,
  process::{ExitCode, Termination},
};

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
  set_var("RUST_LOG", "debug");
  env_logger::init();

  match run() {
    Ok(_) => Exit::Success,
    Err(e) => {
      println!("{:?}", e);
      Exit::Failure
    },
  }
}

fn run() -> Result<()> {
  let boot = helper::boot(Path::new("/Users/echo/dev/gbremu/roms/boot/dmg_boot.bin"))?;
  let cart = helper::cart(Path::new("/Users/echo/dev/gbremu/roms/DrMario.gb"))?;

  // let emu = GameBo
  // let emu = helper::b?;
  Ok(())
}

mod helper {
  use std::{fs::File, io::Read, path::Path};

  use anyhow::Context;
  use libemu::Emulator;
  use libemu::{generic::memory::rom::ROM, hardware::cartridge::Cartridge};
  use log::{debug, info};

  pub fn emu() -> anyhow::Result<Emulator> {
    Ok(Emulator::default())
  }

  type Boot = ROM<u8, 0x100>;
  pub fn boot(path: &Path) -> anyhow::Result<Boot> {
    // Read boot rom
    let rom = {
      // Open boot rom file
      let mut file =
        File::open(path).with_context(|| format!("Filed to open: {}", path.display()))?;
      // Read boot rom into a buffer (must be exactly 0x100 bytes)
      let mut buf = [0u8; 0x100];
      file
        // Read exactly bytes form file
        .read_exact(&mut buf)
        .with_context(|| format!("Filed to read:{}", path.display()))?;
      let len = buf.len();
      debug!("read {} bytes from {}", len, path.display());
      buf
    };

    // Create boot rom
    let boot = Boot::from(&rom);
    info!("Loaded boot ROM");

    Ok(boot)
  }

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
