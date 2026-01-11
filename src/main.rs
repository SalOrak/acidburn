#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    println!("$ ");
    io::stdout().flush().unwrap();
}
