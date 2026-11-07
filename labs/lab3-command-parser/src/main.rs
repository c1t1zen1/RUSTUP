//! Drive Robbie from the keyboard. This file is finished: your work is in src/lib.rs.
//! Try: forward 3, left, say hello, report, charge. Ctrl+D (or Ctrl+Z on Windows) to quit.
use command_parser::{Robot, parse};
use std::io::{self, Write};

fn main() {
    let mut robbie = Robot::new();
    println!("Robbie is listening. {}", robbie.status());
    loop {
        print!("> ");
        io::stdout().flush().ok();
        let mut line = String::new();
        if io::stdin().read_line(&mut line).unwrap_or(0) == 0 {
            break;
        }
        match parse(&line) {
            Some(cmd) => println!("{}", robbie.apply(cmd)),
            None => println!(
                "Robbie tilts his head. Try: forward 3, left, right, say hi, charge, report"
            ),
        }
    }
    println!("Robbie powers down at {}", robbie.status());
}
