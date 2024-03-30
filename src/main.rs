use rustmate::parser::from_path;
use std::env;

fn main() {
    if let Some(filename) = env::args().nth(1) {
        match from_path(&filename) {
            Ok(board) => {
                println!("{}", board.finish_game());
            }
            Err(err) => {
                eprintln!("ERROR: [{}]", err);
                std::process::exit(1);
            }
        }
    } else {
        eprintln!("ERROR: [Please provide a filename as an argument]");
        std::process::exit(1);
    }
}
