use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::open("/guest/Cargo.toml").expect("Failed to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read file");
    println!("{}", contents);
}
