use colored::Colorize;

use rustyline::error::ReadlineError;
use rustyline::Editor;

use crate::parser::parse_whole_term;

pub mod node;
pub mod parser;

#[cfg(windows)]
fn init_terminal() {
    use colored::control::set_virtual_terminal;

    set_virtual_terminal(true).unwrap();
}

#[cfg(not(windows))]
fn init_terminal() {}

fn main() {
    init_terminal();

    let mut rl = Editor::<()>::new();

    loop {
        match rl.readline(">> ") {
            Ok(input) => {
                rl.add_history_entry(&input);

                let term = parse_whole_term(&input);

                println!(
                    "Syntax       : {}",
                    term.as_ref()
                        .map_or_else(|| "INVALID".red(), |_| "OK".green())
                );
                println!(
                    "Formatted    : {}",
                    term.as_ref()
                        .map_or_else(|| "---".to_string(), |t| t.to_string())
                );

                let props = term.as_ref().map(|t| t.get_all_propositions());

                println!(
                    "Propositions : {}",
                    props.as_ref().map_or_else(
                        || "---".to_string(),
                        |props| if props.len() == 0 {
                            "---".to_string()
                        } else {
                            props.join(", ")
                        }
                    )
                );

                println!(
                    "Tautology    : {}",
                    term.as_ref().map_or_else(
                        || "---".normal(),
                        |t| match t.is_tautology(props.as_ref().unwrap()) {
                            Some(true) => "YES".green(),
                            Some(false) => "NO".red(),
                            None => "Too much propositions".yellow(),
                        }
                    )
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
