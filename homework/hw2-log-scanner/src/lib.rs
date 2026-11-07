//! Homework 2: Log Scanner.
//!
//! Every function here borrows its input. Nothing takes ownership of the log,
//! so main can keep using it after each call. Replace each `todo!()` with a real body.
//!
//! A log line looks like `[LEVEL] message`, for example `[WARN] rust spotted on right knee`.

/// Returns the first word of `text`: everything before the first space.
/// If there is no space, the whole text is one word. Return a slice of `text`,
/// don't build a new String.
///
/// `first_word("[INFO] boot")` is `"[INFO]"`.
pub fn first_word(text: &str) -> &str {
    todo!("find the first space in {text:?}")
}

/// Counts the lines in `log` whose level is `level`.
/// A line has level `ERROR` when it starts with `[ERROR]`.
///
/// `count_level("[ERROR] a\n[INFO] b\n[ERROR] c", "ERROR")` is `2`.
pub fn count_level(log: &str, level: &str) -> usize {
    todo!("count lines of {log:?} starting with [{level}]")
}

/// Returns the longest line in `log`, as a slice of `log`.
/// If two lines tie, return the first one. An empty log gives `""`.
pub fn longest_line(log: &str) -> &str {
    todo!("find the longest line of {log:?}")
}

/// Returns a copy of `line` where every occurrence of `secret` is replaced
/// by the same number of `*` characters. The original is untouched.
///
/// `redact("pin 1234 ok", "1234")` is `"pin **** ok"`.
pub fn redact(line: &str, secret: &str) -> String {
    todo!("hide {secret:?} in {line:?}")
}

/// Changes `line` in place: make it uppercase and add `!` at the end.
/// This one borrows mutably, so it returns nothing.
///
/// `"rust on knee"` becomes `"RUST ON KNEE!"`.
pub fn shout(line: &mut String) {
    line.push_str(""); // keeps clippy calm until you write the real body
    todo!("uppercase {line:?} and add !")
}

/// One-line report of the log: the number of lines, then the count of each level.
///
/// For a log with 7 lines, 1 ERROR, 2 WARN and 4 INFO it returns
/// `"7 lines: 1 ERROR, 2 WARN, 4 INFO"`. An empty log is `"0 lines: 0 ERROR, 0 WARN, 0 INFO"`.
pub fn summary(log: &str) -> String {
    todo!("summarise {log:?}")
}
