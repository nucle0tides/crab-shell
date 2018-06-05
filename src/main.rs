use std::io;
use std::process;

const SHELL_PROMPT: &str = "gabby@cosmonaut:~> ";

fn main() {
    println!("In the beginning there was darkness.");
    let mut command = String::new();
    loop {
        io::stdin().read_line(&mut command)
            .expect("issue reading command.");

        command.pop();
        // history, !!, !n, etc
        // okay but what about | and && and what not
        match command.as_ref() {
            "exit" => process::exit(1),
            "history" => println!("list command history"),
            "!!" => println!("execute last command"),
            _ => tokenize_command(&command),
        }

        command.clear();
    }
}

fn tokenize_command(command: &str) {
    let tokens: Vec<&str> = command.split(" ").collect();
    //println!("{:?}", tokens);
}
