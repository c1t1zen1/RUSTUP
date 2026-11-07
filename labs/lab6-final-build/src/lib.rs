//! Lab 6: the Final Build Lab.
//!
//! Five small jobs that get Robbie ready to ship: a macro, an unsafe helper
//! with a safe wrapper, two calls into the C library, a stopwatch for
//! benchmarks, and documentation good enough to publish.

use std::ffi::CStr;
use std::time::Duration;
// Step 4 needs std::time::Instant too.

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
    // Step 1: three arms. Replace each String::new() with the real thing.
    // Hint: vec![word; n] makes n copies, and .join(" ") glues them.
    () => {
        String::new()
    };
    ($n:expr) => {{
        let _ = $n;
        String::new()
    }};
    ($n:expr, $word:expr) => {{
        let _ = ($n, $word);
        String::new()
    }};
}

/// Step 1, part 2. The largest of any number of values, worked out at
/// compile time into a chain of `max` calls.
#[macro_export]
macro_rules! loudest {
    // Step 1, part 2: the first arm is done. The second one should be
    // std::cmp::max($x, loudest!(the rest)), calling itself with $crate::loudest!.
    ($x:expr) => {
        $x
    };
    ($x:expr, $($rest:expr),+ $(,)?) => {
        $x
    };
}

/// Step 2. Splits a slice into two halves you can change at the same time.
/// For an odd length, the extra item goes in the second half.
///
/// The borrow checker won't let you take two `&mut` from one slice, even
/// when they can't overlap, so this uses raw pointers inside `unsafe`.
pub fn split_halves(items: &mut [u32]) -> (&mut [u32], &mut [u32]) {
    todo!(
        "step 2: split {} items with as_mut_ptr and from_raw_parts_mut",
        items.len()
    )
}

// Step 3: declare C's abs and strlen here, in an `unsafe extern "C"` block.

/// Step 3. The C library's `abs`, behind a safe Rust function.
///
/// C's `abs(INT_MIN)` is undefined behaviour, so that one input gets `None`
/// instead of ever reaching C.
pub fn c_abs(x: i32) -> Option<i32> {
    todo!("step 3: call C's abs({x}), but never with i32::MIN")
}

/// Step 3, part 2. The C library's `strlen` on a C string.
pub fn c_strlen(s: &CStr) -> usize {
    todo!("step 3: call C's strlen on {s:?}")
}

/// Step 4. Runs `job` `runs` times and returns the fastest single run.
/// `runs` of 0 counts as 1. The fastest run is the one least disturbed by
/// whatever else the computer was doing.
pub fn time_it<F: FnMut()>(runs: u32, mut job: F) -> Duration {
    let _ = &mut job;
    todo!("step 4: run the job {runs} times and keep the fastest")
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
pub fn status() -> String {
    format!(
        "{} {}: {}",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_DESCRIPTION")
    )
}
