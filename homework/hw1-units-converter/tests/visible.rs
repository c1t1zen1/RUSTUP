use units_converter::*;

fn close(a: f64, b: f64) -> bool {
    (a - b).abs() < 1e-6
}

#[test]
fn marathon_in_miles() {
    assert!(close(km_to_miles(42.195), 26.218_757_456_454_63));
}

#[test]
fn miles_round_trip() {
    assert!(close(km_to_miles(miles_to_km(3.0)), 3.0));
}

#[test]
fn water_boils() {
    assert!(close(celsius_to_fahrenheit(100.0), 212.0));
}

#[test]
fn minus_forty_is_minus_forty() {
    assert!(close(fahrenheit_to_celsius(-40.0), -40.0));
}

#[test]
fn battery_bar_half() {
    assert_eq!(battery_bar(50), "[#####-----] 50%");
}

#[test]
fn battery_bar_rounds_down() {
    assert_eq!(battery_bar(42), "[####------] 42%");
}

#[test]
fn battery_bar_caps_at_100() {
    assert_eq!(battery_bar(250), "[##########] 100%");
}

#[test]
fn moods() {
    assert_eq!(battery_mood(5), "sleepy");
    assert_eq!(battery_mood(20), "ok");
    assert_eq!(battery_mood(80), "zoomy");
}
