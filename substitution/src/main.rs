use std::env;
use substitution::{encrypt, Config};

fn main() {
    let mut args = env::args();

    let config = Config::new(&mut args).unwrap();

    println!("Encrypted text: {}", encrypt(&config));
}
