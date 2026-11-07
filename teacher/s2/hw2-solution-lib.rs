//! Homework 2: Log Scanner. Reference solution.

/// Returns the first word of `text`: everything before the first space.
pub fn first_word(text: &str) -> &str {
    for (i, b) in text.bytes().enumerate() {
        if b == b' ' {
            return &text[..i];
        }
    }
    text
}

/// Counts the lines in `log` whose level is `level`.
pub fn count_level(log: &str, level: &str) -> usize {
    let tag = format!("[{level}]");
    let mut count = 0;
    for line in log.lines() {
        if line.starts_with(&tag) {
            count += 1;
        }
    }
    count
}

/// Returns the longest line in `log`; the first one wins a tie.
pub fn longest_line(log: &str) -> &str {
    let mut best = "";
    for line in log.lines() {
        if line.len() > best.len() {
            best = line;
        }
    }
    best
}

/// Replaces every occurrence of `secret` with the same number of stars.
pub fn redact(line: &str, secret: &str) -> String {
    line.replace(secret, &"*".repeat(secret.len()))
}

/// Uppercases `line` in place and adds `!`.
pub fn shout(line: &mut String) {
    *line = line.to_uppercase();
    line.push('!');
}

/// One-line report: line count, then each level's count.
pub fn summary(log: &str) -> String {
    format!(
        "{} lines: {} ERROR, {} WARN, {} INFO",
        log.lines().count(),
        count_level(log, "ERROR"),
        count_level(log, "WARN"),
        count_level(log, "INFO")
    )
}
