extern crate rustyline;

use rustyline::error::ReadlineError;
use rustyline::Editor;

pub fn read(message: &str) -> String {
    let mut rl = Editor::<()>::new();
    loop {
        let readline = rl.readline(message);
        match readline {
            Ok(line) => {
                return line;
            },
            Err(ReadlineError::Interrupted) => {
                println!("Ctrl-C");
                break
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
