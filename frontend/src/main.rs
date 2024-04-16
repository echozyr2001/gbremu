use std::time::Duration;

use sdl2::{event::Event, keyboard::Keycode, pixels::Color};

fn main() -> Result<(), String> {
  let sdl_context = sdl2::init()?;
  let video_subsystem = sdl_context.video()?;

  let window = video_subsystem
    .window("rust-sdl2 demo", 800, 600)
    .position_centered()
    .build()
    .map_err(|e| e.to_string())?;

  let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
  canvas.set_draw_color(Color::RGB(0, 255, 255)); // set background color
  canvas.clear(); // clear canvas
  canvas.present(); // display canvas

  let mut event_pump = sdl_context.event_pump()?; // event_pump
  'running: loop {
    for event in event_pump.poll_iter() {
      match event {
        Event::Quit { .. }
        | Event::KeyDown {
          keycode: Some(Keycode::Escape),
          ..
        } => {
          break 'running; // press ESC to exit
        },
        Event::KeyDown {
          keycode: Some(key), ..
        } => {
          println!("Pressed {:?}", key);
        },
        Event::KeyUp {
          keycode: Some(key), ..
        } => {
          println!("Released {:?}", key);
        },
        _ => {},
      }
    }

    // update canvas
    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    std::thread::sleep(Duration::from_millis(100)); // wait for 100ms
  }
  Ok(())
}
