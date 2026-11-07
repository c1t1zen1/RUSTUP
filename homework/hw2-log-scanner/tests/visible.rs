use log_scanner::*;

const LOG: &str = "[INFO] boot\n[WARN] squeak\n[ERROR] coffee in deck\n[INFO] ok";

#[test]
fn first_word_stops_at_space() {
    assert_eq!(first_word("[INFO] boot sequence"), "[INFO]");
}

#[test]
fn first_word_without_space_is_everything() {
    assert_eq!(first_word("standby"), "standby");
}

#[test]
fn counts_each_level() {
    assert_eq!(count_level(LOG, "INFO"), 2);
    assert_eq!(count_level(LOG, "ERROR"), 1);
    assert_eq!(count_level(LOG, "DEBUG"), 0);
}

#[test]
fn finds_longest_line() {
    assert_eq!(longest_line(LOG), "[ERROR] coffee in deck");
}

#[test]
fn redacts_secret() {
    assert_eq!(redact("pin 1234 ok", "1234"), "pin **** ok");
}

#[test]
fn redact_leaves_original_alone() {
    let original = String::from("key abc");
    let _ = redact(&original, "abc");
    assert_eq!(original, "key abc");
}

#[test]
fn shout_changes_in_place() {
    let mut s = String::from("rust on knee");
    shout(&mut s);
    assert_eq!(s, "RUST ON KNEE!");
}

#[test]
fn summary_counts_everything() {
    assert_eq!(summary(LOG), "4 lines: 1 ERROR, 1 WARN, 2 INFO");
}
