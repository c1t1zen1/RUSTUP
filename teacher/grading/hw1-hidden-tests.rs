// Hidden tests for Homework 1. Copy into the student's tests/ folder before grading.
use units_converter::*;
#[test]
fn bar_zero() {
    assert_eq!(battery_bar(0), "[----------] 0%");
}
#[test]
fn bar_full() {
    assert_eq!(battery_bar(100), battery_bar(255));
    assert!(battery_bar(255).ends_with("100%"));
    assert!(!battery_bar(255).contains('-'));
}
#[test]
fn bar_nine() {
    assert!(!battery_bar(9).contains('#'));
}
#[test]
fn moods() {
    assert_eq!(battery_mood(19), "sleepy");
    assert_eq!(battery_mood(79), "ok");
}
#[test]
fn zero_km() {
    assert_eq!(km_to_miles(0.0), 0.0);
}
#[test]
fn round_trip() {
    for c in [-273.15, 0.0, 37.0] {
        assert!((fahrenheit_to_celsius(celsius_to_fahrenheit(c)) - c).abs() < 1e-9);
    }
}
