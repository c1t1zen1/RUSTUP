//! Lab 1: Robbie's Greeting. A boot log, a countdown and a hello.
use std::thread;
use std::time::Duration;

const MODEL: &str = "Robbie v11";
const PARTS: [&str; 5] = ["optics", "cassette deck", "shoulders", "grippers", "tracks"];

/// One line of the boot log, padded so the OKs line up.
fn boot_line(step: usize, total: usize, part: &str) -> String {
    format!("[{step}/{total}] checking {part:<14} OK")
}

/// Battery drops 7% per part checked, but never below zero.
fn battery_after(checks: u32) -> u32 {
    100_u32.saturating_sub(checks * 7)
}

fn main() {
    println!("== {MODEL} boot sequence ==");
    let total = PARTS.len();
    for (i, part) in PARTS.iter().enumerate() {
        println!("{}", boot_line(i + 1, total, part));
        thread::sleep(Duration::from_millis(150));
    }

    let battery = battery_after(total as u32);
    let mood = if battery > 50 { "chipper" } else { "sleepy" };
    println!("battery {battery}% and feeling {mood}");

    for n in (1..=3).rev() {
        println!("{n}...");
        thread::sleep(Duration::from_millis(300));
    }
    println!("Hello, world! I am {MODEL}. Let's learn some Rust.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn boot_line_pads_parts() {
        assert_eq!(
            boot_line(1, 5, "optics"),
            "[1/5] checking optics         OK"
        );
    }

    #[test]
    fn battery_never_goes_negative() {
        assert_eq!(battery_after(5), 65);
        assert_eq!(battery_after(100), 0);
    }
}
