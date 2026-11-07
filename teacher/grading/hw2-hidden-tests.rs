// Hidden tests for Homework 2. Copy into the student's tests/ folder before grading.
use log_scanner::*;

#[test]
fn empty_log_summary() {
    assert_eq!(summary(""), "0 lines: 0 ERROR, 0 WARN, 0 INFO");
}

#[test]
fn empty_log_longest_line() {
    assert_eq!(longest_line(""), "");
}

#[test]
fn tie_goes_to_first_line() {
    assert_eq!(longest_line("[INFO] aaa\n[WARN] bbb"), "[INFO] aaa");
}

#[test]
fn level_named_mid_line_is_not_counted() {
    assert_eq!(
        count_level("[INFO] no ERROR here\n[ERROR] real", "ERROR"),
        1
    );
}

#[test]
fn redact_repeated_and_missing_secrets() {
    assert_eq!(redact("ab ab", "ab"), "** **");
    assert_eq!(redact("nothing to hide", "zzz"), "nothing to hide");
}

#[test]
fn first_word_of_empty_and_leading_space() {
    assert_eq!(first_word(""), "");
    assert_eq!(first_word(" lead"), "");
}

#[test]
fn shout_empty_string() {
    let mut s = String::new();
    shout(&mut s);
    assert_eq!(s, "!");
}
