//! Lab 6 checks, one group per step. Run one step with: cargo test step3
//! The examples in the doc comments are tests too: cargo test --doc
use ship_it::{beep, c_abs, c_strlen, loudest, split_halves, time_it, vowels_bytes, vowels_chars};

#[test]
fn step1_beep_has_three_arms() {
    assert_eq!(beep!(), "beep");
    assert_eq!(beep!(1), "beep");
    assert_eq!(beep!(3), "beep beep beep");
    assert_eq!(beep!(2, "squeak"), "squeak squeak");
    assert_eq!(beep!(0), "");
}

#[test]
fn step1_loudest_takes_any_number() {
    assert_eq!(loudest!(4), 4);
    assert_eq!(loudest!(4, 11), 11);
    assert_eq!(loudest!(2, 8, 5, 8, 1), 8);
    assert_eq!(loudest!("bolt", "nut", "washer"), "washer");
}

#[test]
fn step2_halves_change_together() {
    let mut readings = [1, 2, 3, 4, 5, 6];
    let (front, back) = split_halves(&mut readings);
    assert_eq!((front.len(), back.len()), (3, 3));
    front[2] += 100;
    back[0] += 1000;
    assert_eq!(readings, [1, 2, 103, 1004, 5, 6]);
}

#[test]
fn step2_odd_and_empty() {
    let mut odd = [7, 8, 9];
    let (front, back) = split_halves(&mut odd);
    assert_eq!(front, &mut [7]);
    assert_eq!(back, &mut [8, 9]);

    let mut none: [u32; 0] = [];
    let (front, back) = split_halves(&mut none);
    assert!(front.is_empty() && back.is_empty());
}

#[test]
fn step3_c_abs_is_safe() {
    assert_eq!(c_abs(-42), Some(42));
    assert_eq!(c_abs(0), Some(0));
    assert_eq!(c_abs(i32::MAX), Some(i32::MAX));
    assert_eq!(c_abs(i32::MIN), None);
}

#[test]
fn step3_c_strlen_counts_bytes() {
    assert_eq!(c_strlen(c"robbie"), 6);
    assert_eq!(c_strlen(c""), 0);
    assert_eq!(c_strlen(c"r\u{f6}bbie"), 7); // the ö is two bytes
}

#[test]
fn step4_time_it_runs_the_job() {
    let mut calls = 0;
    let best = time_it(5, || calls += 1);
    assert_eq!(calls, 5);
    assert!(best < std::time::Duration::from_secs(1));

    let mut once = 0;
    time_it(0, || once += 1);
    assert_eq!(once, 1);
}

#[test]
fn step4_both_counters_agree() {
    let text = "Robbie Oils His Elbows Again";
    assert_eq!(vowels_chars(text), 11);
    assert_eq!(vowels_bytes(text), 11);
}

#[test]
fn step5_ready_to_publish() {
    assert!(
        !env!("CARGO_PKG_DESCRIPTION").is_empty(),
        "add a description to Cargo.toml"
    );
    assert!(
        !env!("CARGO_PKG_LICENSE").is_empty(),
        "add a license to Cargo.toml"
    );
    assert!(
        !env!("CARGO_PKG_REPOSITORY").is_empty(),
        "add a repository to Cargo.toml"
    );
    assert!(ship_it::status().ends_with(env!("CARGO_PKG_DESCRIPTION")));
}
