extern crate rustyline;
extern crate colored;

use rustyline::error::ReadlineError;
use rustyline::Editor;
use colored::*;
use std::process::exit;

pub fn read(message: &str) -> String {
    let mut rl = Editor::<()>::new();
    let colored_message = "[<] ".bright_cyan().bold().to_string() + message;
    loop {
        let readline = rl.readline(&colored_message);
        match readline {
            Ok(line) => {
                return line;
            },
            Err(ReadlineError::Interrupted) => {
                println!("Ctrl-C");
                exit(0)
            },
            Err(ReadlineError::Eof) => {
                println!("Ctrl-D");
                break
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break
            },
        }
    }
    return String::new();
}
