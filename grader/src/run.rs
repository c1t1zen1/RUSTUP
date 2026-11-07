//! Running cargo with a time limit and capturing what it says.

use std::io::Read;
use std::path::Path;
use std::process::{Command, Stdio};
use std::thread;
use std::time::{Duration, Instant};

/// What a finished command left behind.
#[derive(Debug, Clone, Default)]
pub struct Outcome {
    /// It exited with status 0.
    pub ok: bool,
    /// It ran past the time limit and was stopped.
    pub timed_out: bool,
    /// Standard output and standard error, one after the other.
    pub output: String,
}

/// Runs `cargo <args>` in `dir`, stopping it after `limit`.
pub fn cargo(dir: &Path, args: &[&str], limit: Duration) -> Outcome {
    let child = Command::new("cargo")
        .args(args)
        .current_dir(dir)
        .env("CARGO_TERM_COLOR", "never")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn();
    let mut child = match child {
        Ok(c) => c,
        Err(e) => {
            return Outcome {
                output: format!("couldn't start cargo: {e}"),
                ..Outcome::default()
            };
        }
    };
    let mut out_pipe = child.stdout.take();
    let mut err_pipe = child.stderr.take();
    let out_reader = thread::spawn(move || {
        let mut s = String::new();
        if let Some(p) = out_pipe.as_mut() {
            let _ = p.read_to_string(&mut s);
        }
        s
    });
    let err_reader = thread::spawn(move || {
        let mut s = String::new();
        if let Some(p) = err_pipe.as_mut() {
            let _ = p.read_to_string(&mut s);
        }
        s
    });

    let start = Instant::now();
    let mut timed_out = false;
    let status = loop {
        match child.try_wait() {
            Ok(Some(status)) => break Some(status),
            Ok(None) if start.elapsed() > limit => {
                let _ = child.kill();
                timed_out = true;
                break child.wait().ok();
            }
            Ok(None) => thread::sleep(Duration::from_millis(50)),
            Err(_) => break None,
        }
    };
    let mut output = out_reader.join().unwrap_or_default();
    output.push_str(&err_reader.join().unwrap_or_default());
    Outcome {
        ok: !timed_out && status.is_some_and(|s| s.success()),
        timed_out,
        output,
    }
}
