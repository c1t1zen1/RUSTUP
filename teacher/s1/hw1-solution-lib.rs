//! Robbie's Units Converter. Fill in every `todo!()` so `cargo test` passes.

/// Kilometres in one mile.
pub const KM_PER_MILE: f64 = 1.609_344;

/// Converts kilometres to miles.
pub fn km_to_miles(km: f64) -> f64 {
    km / KM_PER_MILE
}

/// Converts miles to kilometres.
pub fn miles_to_km(miles: f64) -> f64 {
    miles * KM_PER_MILE
}

/// Converts degrees Celsius to degrees Fahrenheit.
pub fn celsius_to_fahrenheit(c: f64) -> f64 {
    c * 9.0 / 5.0 + 32.0
}

/// Converts degrees Fahrenheit to degrees Celsius.
pub fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

/// Draws a 10-cell battery bar. Values over 100 are treated as 100.
///
/// `battery_bar(42)` returns `"[####------] 42%"` (each `#` is 10%, rounded down).
pub fn battery_bar(percent: u8) -> String {
    let p = percent.min(100);
    let filled = (p / 10) as usize;
    let mut bar = String::from("[");
    for i in 0..10 {
        if i < filled {
            bar.push('#');
        } else {
            bar.push('-');
        }
    }
    bar.push_str("] ");
    format!("{bar}{p}%")
}

/// Robbie's mood for a battery level: "sleepy" under 20, "ok" under 80, "zoomy" otherwise.
pub fn battery_mood(percent: u8) -> &'static str {
    if percent < 20 {
        "sleepy"
    } else if percent < 80 {
        "ok"
    } else {
        "zoomy"
    }
}
