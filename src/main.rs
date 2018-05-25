use std::io;
use std::process;

const SHELL_PROMPT: &str = "gabby@cosmonaut:~> ";

fn main() {
    println!("In the beginning there was darkness.");
    let mut command = String::new();
    loop {
        // read user input
        io::stdin().read_line(&mut command)
            .expect("issue reading command.");

        // determine when to exit
        // maybe 'match' for 'history' or 'more' or ! or !5 or & idk
        command.pop();
        if command == "exit" {
            process::exit(1);
        }

        // do some shit

        // clear the command
        command.clear();
    }
}
