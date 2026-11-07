//! Lab 6: the Final Build Lab.
//!
//! Five small jobs that get Robbie ready to ship: a macro, an unsafe helper
//! with a safe wrapper, two calls into the C library, a stopwatch for
//! benchmarks, and documentation good enough to publish.

use std::ffi::CStr;
use std::time::{Duration, Instant};

/// Step 1. Robbie's beep, as a macro.
///
/// `beep!()` is one beep. `beep!(3)` is three, with spaces between.
/// `beep!(2, "boop")` repeats any word.
///
/// ```
/// use ship_it::beep;
/// assert_eq!(beep!(), "beep");
/// assert_eq!(beep!(3), "beep beep beep");
/// assert_eq!(beep!(2, "boop"), "boop boop");
/// ```
#[macro_export]
macro_rules! beep {
    () => {
        String::from("beep")
    };
    ($n:expr) => {
        $crate::beep!($n, "beep")
    };
    ($n:expr, $word:expr) => {
        vec![$word; $n].join(" ")
    };
}

/// Step 1, part 2. The largest of any number of values, worked out at
/// compile time into a chain of `max` calls.
///
/// ```
/// use ship_it::loudest;
/// assert_eq!(loudest!(3), 3);
/// assert_eq!(loudest!(3, 9, 4), 9);
/// ```
#[macro_export]
macro_rules! loudest {
    ($x:expr) => {
        $x
    };
    ($x:expr, $($rest:expr),+ $(,)?) => {
        std::cmp::max($x, $crate::loudest!($($rest),+))
    };
}

/// Step 2. Splits a slice into two halves you can change at the same time.
/// For an odd length, the extra item goes in the second half.
///
/// The borrow checker won't let you take two `&mut` from one slice, even
/// when they can't overlap, so this uses raw pointers inside `unsafe`.
///
/// ```
/// let mut readings = [1, 2, 3, 4, 5];
/// let (front, back) = ship_it::split_halves(&mut readings);
/// front[0] = 10;
/// back[0] = 30;
/// assert_eq!(readings, [10, 2, 30, 4, 5]);
/// ```
pub fn split_halves(items: &mut [u32]) -> (&mut [u32], &mut [u32]) {
    let len = items.len();
    let mid = len / 2;
    let ptr = items.as_mut_ptr();
    // SAFETY: `ptr` is valid for `len` items because it came from `items`.
    // The first slice covers 0..mid and the second mid..len, so they never
    // overlap, and both live no longer than the `items` borrow they came from.
    unsafe {
        (
            std::slice::from_raw_parts_mut(ptr, mid),
            std::slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

unsafe extern "C" {
    /// `abs` from the C standard library.
    fn abs(x: i32) -> i32;
    /// `strlen` from the C standard library.
    fn strlen(s: *const std::ffi::c_char) -> usize;
}

/// Step 3. The C library's `abs`, behind a safe Rust function.
///
/// C's `abs(INT_MIN)` is undefined behaviour, so that one input gets `None`
/// instead of ever reaching C.
///
/// ```
/// assert_eq!(ship_it::c_abs(-7), Some(7));
/// assert_eq!(ship_it::c_abs(i32::MIN), None);
/// ```
pub fn c_abs(x: i32) -> Option<i32> {
    if x == i32::MIN {
        return None;
    }
    // SAFETY: abs has no preconditions apart from INT_MIN, ruled out above.
    Some(unsafe { abs(x) })
}

/// Step 3, part 2. The C library's `strlen` on a C string.
///
/// ```
/// assert_eq!(ship_it::c_strlen(c"robbie"), 6);
/// ```
pub fn c_strlen(s: &CStr) -> usize {
    // SAFETY: a &CStr always points at valid memory that ends in a 0 byte,
    // which is exactly what strlen needs.
    unsafe { strlen(s.as_ptr()) }
}

/// Step 4. Runs `job` `runs` times and returns the fastest single run.
/// `runs` of 0 counts as 1. The fastest run is the one least disturbed by
/// whatever else the computer was doing.
///
/// ```
/// let best = ship_it::time_it(3, || {
///     std::hint::black_box(2 + 2);
/// });
/// assert!(best < std::time::Duration::from_secs(1));
/// ```
pub fn time_it<F: FnMut()>(runs: u32, mut job: F) -> Duration {
    let mut best = Duration::MAX;
    for _ in 0..runs.max(1) {
        let start = Instant::now();
        job();
        best = best.min(start.elapsed());
    }
    best
}

/// Counts vowels by walking the `char`s. Given to you for the benchmark.
pub fn vowels_chars(text: &str) -> usize {
    text.chars()
        .filter(|c| matches!(c.to_ascii_lowercase(), 'a' | 'e' | 'i' | 'o' | 'u'))
        .count()
}

/// Counts vowels by walking the bytes. Given to you for the benchmark.
pub fn vowels_bytes(text: &str) -> usize {
    text.bytes()
        .filter(|b| matches!(b.to_ascii_lowercase(), b'a' | b'e' | b'i' | b'o' | b'u'))
        .count()
}

/// Step 5. A one-line status for the crate, read from `Cargo.toml` when it
/// was built: `"ship_it 0.1.0: <description>"`.
///
/// ```
/// assert!(ship_it::status().starts_with("ship_it 0.1.0: "));
/// ```
pub fn status() -> String {
    format!(
        "{} {}: {}",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_DESCRIPTION")
    )
}
