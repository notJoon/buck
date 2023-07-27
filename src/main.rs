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

        match parse_query(input) {
            Ok(query) => {
                let log = query.execute(input, &mut db).unwrap();
                println!("{}", log);
            }
            Err(e) => {
                println!("{}", e);
            }
        };

        match input {
            "exit" => break,
            "clear" => {
                print!("{}[2J", 27 as char);
            }
            _ => {}
        }
    }
}
