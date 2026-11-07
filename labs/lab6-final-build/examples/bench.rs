//! Step 4: a home-made benchmark. Always run it in release mode:
//!
//!     cargo run --release --example bench
//!
//! black_box stops the optimizer from noticing that nobody uses the answer
//! and deleting the work you're trying to time.
use ship_it::{time_it, vowels_bytes, vowels_chars};
use std::hint::black_box;

fn main() {
    let text = "Robbie oils his elbows again. ".repeat(20_000);

    let chars = time_it(20, || {
        black_box(vowels_chars(black_box(&text)));
    });
    let bytes = time_it(20, || {
        black_box(vowels_bytes(black_box(&text)));
    });

    println!("{} letters of text, best of 20 runs each:", text.len());
    println!("  vowels_chars  {chars:>10.2?}");
    println!("  vowels_bytes  {bytes:>10.2?}");
    let ratio = chars.as_secs_f64() / bytes.as_secs_f64().max(1e-9);
    println!("  bytes is {ratio:.1}x the speed of chars");
}
