use std::io::{self, Write};
use std::process;
use std::env;

extern crate colored;

use colored::*;

const HOSTNAME: &str = "gabby@cosmonaut";

fn main() {
    println!("In the beginning there was darkness.");
    loop {
        print!("{}:~{}{} ", HOSTNAME.magenta().bold(), env::current_dir().unwrap().to_string_lossy().cyan().bold(), String::from(">").cyan().bold());
        io::stdout().flush().unwrap();
        let mut command = read_line().expect("issue reading from stdin");

        command.pop();
        if command.is_empty() {
            continue
        }

        let tokens: Vec<_> = command.split_whitespace().map(String::from).collect();

        for token in &tokens {
            if *token == "exit" {
                process::exit(0);
            }
        }

        process::Command::new(&tokens[0])
            .args(&tokens[1..])
            .spawn()
            .expect("oh no oh nononono");
    }
}

// thanks elias
fn read_line() -> Result<String, io::Error> {
    let mut line = String::new();
    io::stdin().read_line(&mut line)?;
    Ok(line)
}
