pub use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(name = "mannaggia.rs", author, version, about, long_about = None)]
pub struct Args {
    /// Saints per minute
    #[clap(long, default_value_t = 60)]
    pub spm: u32,

    #[clap(long, short, default_value_t = 0.1)]
    pub special_odds: f64,
}
