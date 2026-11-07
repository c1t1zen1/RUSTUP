//! Reading cargo's output and the student's source.

/// Adds up every `test result:` line: `(passed, failed)`.
pub fn test_counts(output: &str) -> (u32, u32) {
    let mut passed = 0;
    let mut failed = 0;
    for line in output.lines().filter(|l| l.starts_with("test result:")) {
        for part in line.split(';') {
            let words: Vec<&str> = part.split_whitespace().collect();
            if let [.., n, kind] = words.as_slice() {
                let n: u32 = n.trim_end_matches('.').parse().unwrap_or(0);
                match *kind {
                    "passed" => passed += n,
                    "failed" => failed += n,
                    _ => {}
                }
            }
        }
    }
    (passed, failed)
}

/// How many `#[test]` functions a test file holds.
pub fn tests_in(source: &str) -> u32 {
    source.lines().filter(|l| l.trim() == "#[test]").count() as u32
}

/// The distinct warnings in `cargo clippy --message-format=short` output.
pub fn clippy_warnings(output: &str) -> Vec<String> {
    let mut seen: Vec<String> = Vec::new();
    for line in output.lines() {
        if line.contains(": warning: ") && !line.starts_with("warning: ") {
            let line = line.to_string();
            if !seen.contains(&line) {
                seen.push(line);
            }
        }
    }
    seen
}

/// Public items in one source file: `(documented, total)`.
///
/// An item is a line starting with `pub fn`, `pub struct`, `pub enum`,
/// `pub trait`, `pub type`, `pub const`, `pub static` or `pub mod`. It counts
/// as documented when the nearest line above it that isn't an attribute is a
/// `///` comment.
pub fn doc_coverage(source: &str) -> (u32, u32) {
    const KINDS: [&str; 8] = [
        "pub fn ",
        "pub struct ",
        "pub enum ",
        "pub trait ",
        "pub type ",
        "pub const ",
        "pub static ",
        "pub mod ",
    ];
    let lines: Vec<&str> = source.lines().map(str::trim).collect();
    let mut documented = 0;
    let mut total = 0;
    for (i, line) in lines.iter().enumerate() {
        if !KINDS.iter().any(|k| line.starts_with(k)) {
            continue;
        }
        total += 1;
        let above = lines[..i]
            .iter()
            .rev()
            .find(|l| !l.starts_with("#[") && !l.starts_with("#!["));
        if above.is_some_and(|l| l.starts_with("///")) {
            documented += 1;
        }
    }
    (documented, total)
}

/// Things a reviewer should look at in library code: `(line number, what)`.
/// Scanning stops at `#[cfg(test)]`, so unit tests can unwrap freely.
pub fn review_flags(source: &str) -> Vec<(usize, &'static str)> {
    const LOOK_FOR: [(&str, &str); 6] = [
        (".unwrap()", "unwrap"),
        (".expect(", "expect"),
        ("panic!(", "panic!"),
        ("todo!(", "todo!"),
        ("unimplemented!(", "unimplemented!"),
        ("unsafe ", "unsafe"),
    ];
    let mut flags = Vec::new();
    for (i, line) in source.lines().enumerate() {
        let code = line.trim_start();
        if code.starts_with("#[cfg(test)]") {
            break;
        }
        if code.starts_with("//") {
            continue;
        }
        for (needle, what) in LOOK_FOR {
            if code.contains(needle) {
                flags.push((i + 1, what));
            }
        }
    }
    flags
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_every_result_line() {
        let out = "test result: ok. 12 passed; 0 failed; 0 ignored\n\
                   test result: FAILED. 3 passed; 2 failed; 1 ignored; 0 measured";
        assert_eq!(test_counts(out), (15, 2));
        assert_eq!(test_counts("error[E0599]: nope"), (0, 0));
    }

    #[test]
    fn counts_test_attributes() {
        assert_eq!(
            tests_in("#[test]\nfn a() {}\n  #[test]\nfn b() {}\n// #[test]"),
            2
        );
    }

    #[test]
    fn clippy_warnings_are_deduplicated() {
        let out = "src/lib.rs:3:5: warning: unused variable\n\
                   src/lib.rs:3:5: warning: unused variable\n\
                   warning: `x` (lib) generated 1 warning\n\
                   src/bot.rs:9:1: warning: very complex type";
        assert_eq!(clippy_warnings(out).len(), 2);
    }

    #[test]
    fn doc_coverage_skips_attributes() {
        let src = "/// A bot.\n#[derive(Debug)]\npub struct Bot;\npub fn naked() {}\npub use x::y;";
        assert_eq!(doc_coverage(src), (1, 2));
    }

    #[test]
    fn review_flags_stop_at_tests() {
        let src = "let a = b.unwrap();\n// x.unwrap() in a comment\nunsafe { go() }\n#[cfg(test)]\nlet c = d.unwrap();";
        assert_eq!(review_flags(src), vec![(1, "unwrap"), (3, "unsafe")]);
    }
}
