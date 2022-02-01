use std::{thread, time::Duration};

use once_cell::sync::Lazy;
use rand::{thread_rng, Rng};

static SAINTS: Lazy<Vec<&'static str>> = Lazy::new(|| {
    include_str!("../saints_generator/saints.txt")
        .split_terminator('\n')
        .collect()
});

fn curse_special() {
    println!("Mannaggia alla Madonna");
}

fn curse_saint(saint: &str) {
    println!("Mannaggia San {saint}");
}

fn curse_random(saints: &[&str]) {
    if thread_rng().gen_bool(0.1) {
        curse_special();
    } else {
        let random_saint = saints[thread_rng().gen_range(0..saints.len())];
        curse_saint(random_saint);
    }
}

fn main() {
    loop {
        curse_random(&SAINTS);
        thread::sleep(Duration::from_secs(1));
    }
}
