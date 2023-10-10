use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::io;

#[derive(Debug, Serialize, Deserialize)]
pub struct Msg {
    pub id: u32,
    pub data: String,
}

fn main() -> Result<()> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut buffer)?;

    let msg: Msg = ron::from_str(&buffer)?;

    let ret = Msg {
        id: msg.id + 1,
        data: msg.data + " hahaha",
    };
    let ret = ron::to_string(&ret)?;

    println!("{}", ret);

    Ok(())
}
