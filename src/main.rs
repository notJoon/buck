use std::io::{self, Write};

use ansi_term::Color;
use buck::{engine::BuckDB, parser::parse::parse_query};

fn main() {
    let mut db = BuckDB::new();

    println!("Enter 'exit' to quit.");
    loop {
        print!("{} ", Color::Green.paint("buck>"));
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let input = input.trim();

        // Call Stack: input -> parse_query -> execute -> db -> Output
        match parse_query(input) {
            Ok(query) => match query.execute(input, &mut db) {
                Ok(log) => {
                    println!("{}", log);
                }
                Err(e) => {
                    eprintln!("[ERROR] {}", e);
                }
            },
            Err(e) => {
                eprintln!("[ERROR] Failed to parse query: {}", e)
            }
        };

        match input {
            "exit" => {
                std::process::exit(0);
            }
            "clear" => {
                print!("{}[2J", 27 as char);
            }
            _ => {}
        }
    }
}
