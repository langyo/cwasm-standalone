use anyhow::Result;
use std::io;

fn main() -> Result<()> {
    println!("Hello, world!");

    let mut buffer = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut buffer)?;

    println!("You said: {}", buffer.trim_end());

    Ok(())
}
