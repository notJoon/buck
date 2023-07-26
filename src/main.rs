use std::io::{self, Write};

use ansi_term::Color;
use buck::engine::BuckDB;

fn main() {
    let mut db = BuckDB::new();

    loop {
        println!("Enter 'exit' to quit.");
        print!("{} ", Color::Green.paint("buck>"));
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let input = input.trim();

        if input == "exit" {
            break;
        }

        println!("Unrecognized command '{}'.\n", input)
    }
}
