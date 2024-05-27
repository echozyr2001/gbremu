use sdl2::{
  render::Canvas, ttf::Sdl2TtfContext, video::Window, AudioSubsystem, EventPump, Sdl,
  TimerSubsystem, VideoSubsystem,
};

pub struct SdlSystem {
  pub canvas: Canvas<Window>,
  pub video_subsystem: VideoSubsystem,
  pub timer_subsystem: TimerSubsystem,
  pub audio_subsystem: AudioSubsystem,
  pub event_pump: EventPump,
  pub ttf_context: Sdl2TtfContext,
}

impl SdlSystem {
  pub fn new(sdl: &Sdl, title: &str, width: u32, height: u32, scale: f32) -> Self {
    let video_subsystem = sdl.video().unwrap();
    let timer_subsystem = sdl.timer().unwrap();
    let audio_subsystem = sdl.audio().unwrap();
    let event_pump = sdl.event_pump().unwrap();

    let ttf_context = sdl2::ttf::init().unwrap();

    let window = video_subsystem
      .window(
        title,
        (scale * width as f32) as u32,
        (scale * height as f32) as u32,
      )
      .resizable()
      .position_centered()
      .opengl()
      .build()
      .unwrap();

    let mut canvas_builder = window.into_canvas();
    canvas_builder = canvas_builder.present_vsync();
    let mut canvas = canvas_builder.build().unwrap();
    canvas.set_logical_size(width, height).unwrap();
    canvas.clear();

    Self {
      canvas,
      video_subsystem,
      timer_subsystem,
      audio_subsystem,
      event_pump,
      ttf_context,
    }
  }

  pub fn window(&self) -> &Window {
    self.canvas.window()
  }

  pub fn window_mut(&mut self) -> &mut Window {
    self.canvas.window_mut()
  }
}
