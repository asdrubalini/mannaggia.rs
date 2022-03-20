use std::{thread, time::Duration};

use clap::StructOpt;
use cli::Args;
use once_cell::sync::Lazy;
use rand::{thread_rng, Rng};

mod cli;

static SAINTS: Lazy<Vec<&'static str>> = Lazy::new(|| {
    include_str!("../saints_generator/saints.txt")
        .split_terminator('\n')
        .collect()
});

pub struct Curser {
    sleep_time: Duration,
    special_odds: f64,
}

impl Curser {
    pub fn new(args: Args) -> Self {
        let sleep_seconds = 1. / (args.spm as f32 / 60.);
        let sleep_ms = (sleep_seconds * 1_000.) as u64;

        Curser {
            sleep_time: Duration::from_millis(sleep_ms),
            special_odds: args.special_odds,
        }
    }

    pub fn wait(&self) {
        thread::sleep(self.sleep_time);
    }

    fn get_random_curse(&self) -> String {
        if thread_rng().gen_bool(self.special_odds) {
            "Mannaggia alla Madonna".to_string()
        } else {
            let random_saint = SAINTS[thread_rng().gen_range(0..SAINTS.len())];
            format!("Mannaggia a San {random_saint}")
        }
    }
}

impl Iterator for Curser {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.get_random_curse())
    }
}

fn main() {
    let args = Args::parse();
    let mut curser = Curser::new(args);

    while let Some(curse) = curser.next() {
        println!("{curse}");
        curser.wait();
    }
}
