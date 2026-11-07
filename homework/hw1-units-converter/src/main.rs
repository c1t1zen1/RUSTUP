//! Robbie's Units Converter: the menu. You shouldn't need to change this file.
use std::io::{self, Write};

use units_converter::{
    battery_bar, celsius_to_fahrenheit, fahrenheit_to_celsius, km_to_miles, miles_to_km,
};

fn read_line(prompt: &str) -> String {
    print!("{prompt}");
    io::stdout().flush().expect("could not flush stdout");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("could not read input");
    input.trim().to_string()
}

fn read_number(prompt: &str) -> Option<f64> {
    read_line(prompt).parse::<f64>().ok()
}

fn main() {
    println!("ROBBIE UNITS CONVERTER v1  (type q to quit)");
    loop {
        println!();
        println!("1) km -> miles   2) miles -> km");
        println!("3) C -> F        4) F -> C");
        println!("5) battery bar");
        let choice = read_line("> ");
        if choice == "q" {
            println!("Powering down. Beep.");
            break;
        }
        match choice.as_str() {
            "1" | "2" | "3" | "4" => {
                let Some(x) = read_number("value: ") else {
                    println!("That's not a number. Robbie is confused.");
                    continue;
                };
                let result = match choice.as_str() {
                    "1" => km_to_miles(x),
                    "2" => miles_to_km(x),
                    "3" => celsius_to_fahrenheit(x),
                    _ => fahrenheit_to_celsius(x),
                };
                println!("= {result:.2}");
            }
            "5" => match read_line("percent (0-100): ").parse::<u8>() {
                Ok(p) => println!("{}", battery_bar(p)),
                Err(_) => println!(
                    "Battery percent must be a whole number from 0 to 255 (and really 0 to 100)."
                ),
            },
            _ => println!("Unknown option. Try 1 to 5, or q."),
        }
    }
}
