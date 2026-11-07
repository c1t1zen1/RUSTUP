// Homework 2: Log Scanner. This file is done; your work is in src/lib.rs.
use log_scanner::{count_level, first_word, longest_line, redact, shout, summary};

const ROBBIE_LOG: &str = "\
[INFO] boot sequence started
[INFO] treads calibrated
[WARN] left elbow squeaks at 40 degrees
[ERROR] coffee detected in cassette deck
[INFO] wifi password is hunter2
[WARN] rust spotted on right knee
[INFO] standing by";

fn main() {
    println!("== Robbie's log ==\n{ROBBIE_LOG}\n");
    println!("{}", summary(ROBBIE_LOG));
    println!("errors: {}", count_level(ROBBIE_LOG, "ERROR"));
    println!("longest line: {}", longest_line(ROBBIE_LOG));
    println!("first word of the log: {}", first_word(ROBBIE_LOG));

    let leak = "[INFO] wifi password is hunter2";
    println!("redacted: {}", redact(leak, "hunter2"));

    let mut alarm = String::from("rust spotted on right knee");
    shout(&mut alarm);
    println!("shouted: {alarm}");
}
