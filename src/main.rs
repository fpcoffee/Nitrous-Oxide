extern crate nitro;
extern crate rustyline;

use std::io::Write;
use std::io;
use std::process::{Command, Stdio};

use nitro::parser::expr;

use rustyline::error::ReadlineError;
use rustyline::Editor;

fn parse_line(line: &str) {
    if line.starts_with(":h") { 
        println!("Type Nitro code and press ENTER to run it, or type one of these commands:

        :h Show this help message
        :t Typecheck a Nitro expression
        ");
    } else if line.starts_with(":t") { 
        println!("typechecking {}", &line[2..]);
    } else {
        let res = expr::parse_TopLevelExpr(&line[..]);
        println!("{:#?}\n", res);
    }
}

fn main() {
    // Create a readline editor using rustyline
    // By default, it ignores duplicate entries in the history
    let mut rl = Editor::<()>::new();
    loop {
        let readline = rl.readline("nitro>>> ");
        match readline {
            Ok(line) => {
                // Add to history
                rl.add_history_entry(&line);

                parse_line(&line);

                let _ = io::stdout().flush();
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break
            },
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
    }
    println!("Exiting N20");
}
