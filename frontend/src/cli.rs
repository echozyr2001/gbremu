use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
  // #[arg(short, long, default_value_t = String::from("auto"), help = "GB execution mode (ex: dmg, cgb, sgb) to be used")]
  // mode: String,

  // #[arg(short, long, default_value_t = String::from("printer"), help = "Serial device to be used")]
  // device: String,

  // #[arg(
  //   long,
  //   default_value_t = false,
  //   help = "If set no boot ROM will be loaded"
  // )]
  // no_boot: bool,

  // #[arg(
  //       long,
  //       default_value_t = String::from(""),
  //       help = "Path to Game Boy boot ROM file to be used in loading stage"
  //   )]
  // boot_rom_path: String,

  // #[arg(long, default_value_t = false, help = "If set no PPU will be used")]
  // no_ppu: bool,

  // #[arg(long, default_value_t = false, help = "If set no APU will be used")]
  // no_apu: bool,

  // #[arg(long, default_value_t = false, help = "If set no DMA will be used")]
  // no_dma: bool,

  // #[arg(long, default_value_t = false, help = "If set no timer will be used")]
  // no_timer: bool,

  // #[arg(
  //   long,
  //   default_value_t = false,
  //   help = "Run in benchmark mode, with no UI"
  // )]
  // benchmark: bool,

  // #[arg(
  //   long,
  //   default_value_t = 500000000,
  //   help = "The size of the benchmark in clock ticks"
  // )]
  // benchmark_count: usize,

  // #[arg(long, default_value_t = false, help = "Run benchmark only for the CPU")]
  // benchmark_cpu: bool,

  // #[arg(
  //   long,
  //   default_value_t = false,
  //   help = "Run in headless mode, with no UI"
  // )]
  // headless: bool,

  // #[arg(
  //   long,
  //   default_value_t = false,
  //   help = "If set no CPU speed limit will be imposed"
  // )]
  // unlimited: bool,

  // #[arg(
  //   long,
  //   default_value_t = 0,
  //   help = "Number of CPU cycles to run in headless mode"
  // )]
  // cycles: u64,

  // #[arg(
  //   long,
  //   help = "Cheat codes to be applied to the ROM, supports both Game Genie and GameShark"
  // )]
  // cheats: Vec<String>,

  // #[arg(default_value_t = String::from(DEFAULT_ROM_PATH), help = "Path to the ROM file to be loaded")]
  pub rom_path: Option<String>,
}
