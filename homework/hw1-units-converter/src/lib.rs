//! Robbie's Units Converter. Replace every `todo!()` so `cargo test` passes.
//!
//! `todo!()` compiles but panics when it runs: it's Rust's way of saying
//! "a human still owes me some code here".

/// Kilometres in one mile.
pub const KM_PER_MILE: f64 = 1.609_344;

/// Converts kilometres to miles.
pub fn km_to_miles(km: f64) -> f64 {
    todo!("divide by KM_PER_MILE; km is {km}")
}

/// Converts miles to kilometres.
pub fn miles_to_km(miles: f64) -> f64 {
    todo!("multiply by KM_PER_MILE; miles is {miles}")
}

/// Converts degrees Celsius to degrees Fahrenheit.
pub fn celsius_to_fahrenheit(c: f64) -> f64 {
    todo!("F = C * 9/5 + 32; c is {c}")
}

/// Converts degrees Fahrenheit to degrees Celsius.
pub fn fahrenheit_to_celsius(f: f64) -> f64 {
    todo!("C = (F - 32) * 5/9; f is {f}")
}

/// Draws a 10-cell battery bar. Values over 100 are treated as 100.
///
/// `battery_bar(42)` returns `"[####------] 42%"` (each `#` is 10%, rounded down).
pub fn battery_bar(percent: u8) -> String {
    todo!("build the bar for {percent}%")
}

/// Robbie's mood for a battery level: "sleepy" under 20, "ok" under 80, "zoomy" otherwise.
pub fn battery_mood(percent: u8) -> &'static str {
    todo!("pick a mood for {percent}%")
}
