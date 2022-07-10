use rustyline::error::ReadlineError;
use rustyline::Editor;

use crate::parser::parse_whole_term;

pub mod node;
pub mod parser;

fn main() {
    let mut rl = Editor::<()>::new();

    loop {
        match rl.readline(">> ") {
            Ok(input) => {
                rl.add_history_entry(&input);
                println!(
                    "{}",
                    parse_whole_term(&input)
                        .map_or_else(|| "Error: Invalid syntax".to_string(), |t| t.to_string())
                );
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
}
