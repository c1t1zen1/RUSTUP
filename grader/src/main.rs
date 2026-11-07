//! robbie-grader: scores a RUSTUP final project the way the course does.
//!
//! ```text
//! robbie-grader PROJECT [--hidden FILE]... [--review N] [--stretch N]
//! ```
//!
//! It builds the project, runs hidden tests (copied in for the run and
//! removed after), counts the student's own tests, runs clippy, fmt and
//! cargo doc, flags lines for the code review, prints a scorecard and writes
//! `grade.json` into the project folder for the Review Lab.

mod parse;
mod run;

use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::time::Duration;

const BUILD_LIMIT: Duration = Duration::from_secs(600);
const TEST_LIMIT: Duration = Duration::from_secs(300);

/// One line of the scorecard.
struct Line {
    name: &'static str,
    got: f64,
    max: f64,
    note: String,
}

/// How many points each check is worth for one kind of project.
#[derive(Debug, Clone, PartialEq)]
struct Rubric {
    kind: &'static str,
    build: f64,
    visible: f64,
    hidden: f64,
    mine: f64,
    /// Passing tests in tests/mine.rs needed for full marks.
    mine_needed: u32,
    clippy: f64,
    /// True: any warning loses every clippy point. False: minus 1 each.
    clippy_strict: bool,
    fmt: f64,
    docs: f64,
    review: f64,
}

impl Rubric {
    /// The rubric for a project folder, from its name: `hw1` to `hw4`,
    /// `hw5`, or anything else, which is graded as the final project.
    fn for_project(dir: &Path) -> Rubric {
        let name = dir
            .file_name()
            .map(|n| n.to_string_lossy().to_lowercase())
            .unwrap_or_default();
        let homework = Rubric {
            kind: "homework",
            build: 2.0,
            visible: 3.0,
            hidden: 3.0,
            mine: 0.0,
            mine_needed: 0,
            clippy: 1.0,
            clippy_strict: true,
            fmt: 1.0,
            docs: 0.0,
            review: 0.0,
        };
        if name.starts_with("hw5") {
            Rubric {
                visible: 2.0,
                mine: 1.0,
                mine_needed: 3,
                ..homework
            }
        } else if name.starts_with("hw") {
            homework
        } else {
            Rubric {
                kind: "final project",
                build: 15.0,
                visible: 0.0,
                hidden: 40.0,
                mine: 10.0,
                mine_needed: 10,
                clippy: 10.0,
                clippy_strict: false,
                fmt: 5.0,
                docs: 5.0,
                review: 15.0,
            }
        }
    }

    fn total(&self) -> f64 {
        self.build
            + self.visible
            + self.hidden
            + self.mine
            + self.clippy
            + self.fmt
            + self.docs
            + self.review
    }
}

struct Options {
    project: PathBuf,
    hidden: Vec<PathBuf>,
    review: Option<f64>,
    stretch: f64,
}

const USAGE: &str =
    "usage: robbie-grader PROJECT [--hidden FILE]... [--review 0-15] [--stretch 0-5]
       robbie-grader check hw3     (or check final, from the course folder)";

/// Finds `homework/hw3-...` or `final/robbie-mc` from a short name.
fn find_project(name: &str) -> Result<PathBuf, String> {
    if name == "final" {
        return Ok(PathBuf::from("final/robbie-mc"));
    }
    let entries = fs::read_dir("homework")
        .map_err(|_| "run check from the course folder, next to homework/".to_string())?;
    let prefix = format!("{name}-");
    let mut found: Vec<PathBuf> = entries
        .flatten()
        .map(|e| e.path())
        .filter(|p| {
            p.file_name()
                .is_some_and(|f| f.to_string_lossy().starts_with(&prefix))
        })
        .collect();
    found.sort();
    found
        .into_iter()
        .next()
        .ok_or(format!("no homework folder starts with {prefix}"))
}

fn options(args: &[String]) -> Result<Options, String> {
    let mut project = None;
    let mut hidden = Vec::new();
    let mut review = None;
    let mut stretch = 0.0;
    let mut it = args.iter();
    while let Some(a) = it.next() {
        let mut value = |flag: &str| it.next().cloned().ok_or(format!("{flag} needs a value"));
        match a.as_str() {
            "--hidden" => hidden.push(PathBuf::from(value("--hidden")?)),
            "--review" => {
                let n: f64 = value("--review")?
                    .parse()
                    .map_err(|_| "--review takes a number")?;
                review = Some(n.clamp(0.0, 15.0));
            }
            "--stretch" => {
                let n: f64 = value("--stretch")?
                    .parse()
                    .map_err(|_| "--stretch takes a number")?;
                stretch = n.clamp(0.0, 5.0);
            }
            flag if flag.starts_with("--") => return Err(format!("unknown option {flag}")),
            "check" if project.is_none() => {
                let name = value("check")?;
                project = Some(find_project(&name)?);
            }
            path if project.is_none() => project = Some(PathBuf::from(path)),
            extra => return Err(format!("one project at a time, not also {extra}")),
        }
    }
    let project = project.ok_or(USAGE.to_string())?;
    if !project.join("Cargo.toml").is_file() {
        return Err(format!("{} has no Cargo.toml", project.display()));
    }
    Ok(Options {
        project,
        hidden,
        review,
        stretch,
    })
}

/// Copies a hidden test file into the project for one run and removes it
/// again when dropped, even if grading stops early.
struct Borrowed(PathBuf);

impl Drop for Borrowed {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.0);
    }
}

fn round1(x: f64) -> f64 {
    (x * 10.0).round() / 10.0
}

/// Runs one test target and returns `(passed, total)`. A file that doesn't
/// compile counts every test in it as failed.
fn run_tests(dir: &Path, target: &str, expected: u32) -> (u32, u32, String) {
    let out = run::cargo(dir, &["test", "--test", target], TEST_LIMIT);
    let (passed, failed) = parse::test_counts(&out.output);
    let total = (passed + failed).max(expected);
    let note = if out.timed_out {
        "stopped: ran too long".to_string()
    } else if passed + failed == 0 && !out.ok {
        "didn't compile".to_string()
    } else {
        format!("{passed} of {total} passed")
    };
    (passed, total, note)
}

fn sources(dir: &Path) -> Vec<PathBuf> {
    let mut found = Vec::new();
    let mut stack = vec![dir.join("src")];
    while let Some(d) = stack.pop() {
        let Ok(entries) = fs::read_dir(&d) else {
            continue;
        };
        for e in entries.flatten() {
            let p = e.path();
            if p.is_dir() {
                stack.push(p);
            } else if p.extension().is_some_and(|x| x == "rs") {
                found.push(p);
            }
        }
    }
    found.sort();
    found
}

fn json_str(s: &str) -> String {
    let mut out = String::from("\"");
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

/// Copies each hidden file in for one run and adds up the results.
fn hidden_tests(dir: &Path, files: &[PathBuf]) -> (u32, u32, Vec<String>) {
    let (mut passed, mut total, mut notes) = (0, 0, Vec::new());
    for (i, file) in files.iter().enumerate() {
        let src = match fs::read_to_string(file) {
            Ok(s) => s,
            Err(e) => {
                notes.push(format!("can't read {}: {e}", file.display()));
                continue;
            }
        };
        let target = format!("zz_grader_hidden_{i}");
        let path = dir.join("tests").join(format!("{target}.rs"));
        if fs::create_dir_all(dir.join("tests")).is_err() || fs::write(&path, &src).is_err() {
            notes.push(format!("couldn't copy {} in", file.display()));
            continue;
        }
        let _guard = Borrowed(path);
        let (p, t, n) = run_tests(dir, &target, parse::tests_in(&src));
        passed += p;
        total += t;
        let name = file
            .file_name()
            .map(|f| f.to_string_lossy().into_owned())
            .unwrap_or_default();
        notes.push(format!("{name}: {n}"));
    }
    (passed, total, notes)
}

fn share(passed: u32, total: u32) -> f64 {
    if total == 0 {
        0.0
    } else {
        f64::from(passed) / f64::from(total)
    }
}

fn grade(opts: &Options, rubric: &Rubric) -> (Vec<Line>, Vec<String>, bool) {
    let dir = &opts.project;
    let mut lines = Vec::new();
    let mut flags = Vec::new();

    println!("Grading {} as {} ...", dir.display(), rubric.kind);
    let build = run::cargo(dir, &["build", "--release"], BUILD_LIMIT);
    lines.push(Line {
        name: "Build",
        got: if build.ok { rubric.build } else { 0.0 },
        max: rubric.build,
        note: if build.ok {
            "builds".into()
        } else {
            "doesn't build, grading stops".into()
        },
    });
    if !build.ok {
        let errors = build
            .output
            .lines()
            .filter(|l| l.starts_with("error"))
            .take(5);
        flags.extend(errors.map(|l| l.to_string()));
        return (lines, flags, false);
    }

    let visible_count = fs::read_to_string(dir.join("tests/visible.rs"))
        .map(|s| parse::tests_in(&s))
        .unwrap_or(0);
    if rubric.visible > 0.0 {
        let (p, t, n) = run_tests(dir, "visible", visible_count);
        lines.push(Line {
            name: "Visible tests",
            got: round1(rubric.visible * share(p, t)),
            max: rubric.visible,
            note: n,
        });
    }

    let hidden_line = if !opts.hidden.is_empty() {
        let (p, t, notes) = hidden_tests(dir, &opts.hidden);
        Line {
            name: "Hidden tests",
            got: round1(rubric.hidden * share(p, t)),
            max: rubric.hidden,
            note: notes.join(", "),
        }
    } else if rubric.visible == 0.0 {
        // The final project has no separate visible line, so the visible
        // tests stand in until the teacher adds the hidden ones.
        let (p, t, n) = run_tests(dir, "visible", visible_count);
        Line {
            name: "Hidden tests",
            got: round1(rubric.hidden * share(p, t)),
            max: rubric.hidden,
            note: format!("visible tests standing in: {n}"),
        }
    } else {
        Line {
            name: "Hidden tests",
            got: 0.0,
            max: rubric.hidden,
            note: "not run: your teacher adds these".into(),
        }
    };
    let hidden_ran = !opts.hidden.is_empty() || rubric.visible == 0.0;
    lines.push(hidden_line);

    if rubric.mine > 0.0 {
        let mine = fs::read_to_string(dir.join("tests/mine.rs"))
            .map(|s| parse::tests_in(&s))
            .unwrap_or(0);
        let (p, n) = if mine == 0 {
            (0, "no tests/mine.rs".to_string())
        } else {
            let (p, _, n) = run_tests(dir, "mine", mine);
            (p, n)
        };
        let got = if rubric.mine_needed >= 10 {
            f64::from(p.min(rubric.mine_needed)) * rubric.mine / f64::from(rubric.mine_needed)
        } else if p >= rubric.mine_needed {
            rubric.mine
        } else {
            0.0
        };
        lines.push(Line {
            name: "Your tests",
            got: round1(got),
            max: rubric.mine,
            note: format!("{n} ({} needed)", rubric.mine_needed),
        });
    }

    let clippy = run::cargo(
        dir,
        &["clippy", "--all-targets", "--message-format=short"],
        BUILD_LIMIT,
    );
    let warnings = parse::clippy_warnings(&clippy.output);
    let clippy_points = match (clippy.ok, rubric.clippy_strict) {
        (false, _) => 0.0,
        (true, true) if !warnings.is_empty() => 0.0,
        (true, _) => (rubric.clippy - warnings.len() as f64).max(0.0),
    };
    lines.push(Line {
        name: "Clippy",
        got: clippy_points,
        max: rubric.clippy,
        note: if !clippy.ok {
            "clippy failed".into()
        } else {
            match warnings.len() {
                1 => "1 warning".to_string(),
                n => format!("{n} warnings"),
            }
        },
    });
    flags.extend(warnings.iter().map(|w| format!("clippy: {w}")));

    let fmt = run::cargo(dir, &["fmt", "--check"], TEST_LIMIT);
    lines.push(Line {
        name: "Format",
        got: if fmt.ok { rubric.fmt } else { 0.0 },
        max: rubric.fmt,
        note: if fmt.ok {
            "cargo fmt --check is clean".into()
        } else {
            "run cargo fmt".into()
        },
    });

    // Review flags for every kind of project; docs points for the final.
    let (mut documented, mut items) = (0, 0);
    for file in sources(dir) {
        let Ok(src) = fs::read_to_string(&file) else {
            continue;
        };
        let (d, t) = parse::doc_coverage(&src);
        documented += d;
        items += t;
        if !file.ends_with("main.rs") {
            let shown = file
                .strip_prefix(dir)
                .unwrap_or(&file)
                .display()
                .to_string();
            for (line, what) in parse::review_flags(&src) {
                flags.push(format!("review: {shown}:{line} uses {what}"));
            }
        }
    }
    if rubric.docs > 0.0 {
        let doc = run::cargo(dir, &["doc", "--no-deps"], BUILD_LIMIT);
        let readme = fs::metadata(dir.join("README.md")).is_ok_and(|m| m.len() > 200);
        let points = rubric.docs
            * (0.4 * f64::from(u8::from(doc.ok))
                + 0.4 * share(documented, items)
                + 0.2 * f64::from(u8::from(readme)));
        lines.push(Line {
            name: "Docs",
            got: round1(points),
            max: rubric.docs,
            note: format!(
                "cargo doc {}, {documented} of {items} public items documented, README {}",
                if doc.ok { "builds" } else { "fails" },
                if readme { "present" } else { "missing" }
            ),
        });
    }

    if rubric.review > 0.0 {
        lines.push(Line {
            name: "Code review",
            got: opts.review.unwrap_or(0.0).min(rubric.review),
            max: rubric.review,
            note: if opts.review.is_some() {
                "scored by your teacher".into()
            } else {
                "pending: see the flags below".into()
            },
        });
        lines.push(Line {
            name: "Stretch",
            got: opts.stretch,
            max: 0.0,
            note: "up to +5, scored by your teacher".into(),
        });
    }
    (lines, flags, hidden_ran)
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let opts = match options(&args) {
        Ok(o) => o,
        Err(e) => {
            eprintln!("{e}\n{USAGE}");
            return ExitCode::from(2);
        }
    };
    let rubric = Rubric::for_project(&opts.project);
    let (lines, flags, hidden_ran) = grade(&opts, &rubric);
    let built = lines.first().is_some_and(|l| l.got > 0.0);
    let total = round1(lines.iter().map(|l| l.got).sum());
    let mut pending = Vec::new();
    if built && !hidden_ran {
        pending.push("hidden tests");
    }
    if built && rubric.review > 0.0 && opts.review.is_none() {
        pending.push("code review");
    }

    println!("\nRobbie's scorecard");
    for l in &lines {
        let max = if l.max > 0.0 {
            format!("{:>4}", l.max)
        } else {
            "  +5".to_string()
        };
        println!("  {:<13} {:>5} / {max}   {}", l.name, l.got, l.note);
    }
    let out_of = rubric.total();
    if pending.is_empty() {
        println!("  {:<13} {:>5} / {out_of}", "Total", total);
    } else {
        println!(
            "  {:<13} {:>5} / {out_of}, still to come: {}",
            "Total",
            total,
            pending.join(" and ")
        );
    }
    if !flags.is_empty() {
        println!("\nFor the code review:");
        for f in flags.iter().take(40) {
            println!("  {f}");
        }
        if flags.len() > 40 {
            println!("  ...and {} more in grade.json", flags.len() - 40);
        }
    }

    let json = format!(
        "{{\n  \"robbie_grader\": 1,\n  \"project\": {},\n  \"kind\": {},\n  \"total\": {},\n  \"out_of\": {},\n  \"pending\": [{}],\n  \"lines\": [\n{}\n  ],\n  \"flags\": [{}]\n}}\n",
        json_str(&opts.project.display().to_string()),
        json_str(rubric.kind),
        total,
        out_of,
        pending
            .iter()
            .map(|p| json_str(p))
            .collect::<Vec<_>>()
            .join(", "),
        lines
            .iter()
            .map(|l| format!(
                "    {{\"name\": {}, \"got\": {}, \"max\": {}, \"note\": {}}}",
                json_str(l.name),
                l.got,
                l.max,
                json_str(&l.note)
            ))
            .collect::<Vec<_>>()
            .join(",\n"),
        flags
            .iter()
            .map(|f| json_str(f))
            .collect::<Vec<_>>()
            .join(",\n    ")
    );
    let out = opts.project.join("grade.json");
    match fs::write(&out, json) {
        Ok(()) => println!(
            "\nWrote {}. Drop it on the Review Lab page to go through it.",
            out.display()
        ),
        Err(e) => eprintln!("couldn't write {}: {e}", out.display()),
    }
    if built {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rubrics_add_up() {
        assert_eq!(
            Rubric::for_project(Path::new("final/robbie-mc")).total(),
            100.0
        );
        assert_eq!(
            Rubric::for_project(Path::new("homework/hw3-command-language")).total(),
            10.0
        );
        assert_eq!(
            Rubric::for_project(Path::new("homework/hw5-swarm-sim")).total(),
            10.0
        );
    }

    #[test]
    fn json_strings_are_escaped() {
        assert_eq!(json_str("a \"b\"\n\\"), r#""a \"b\"\n\\""#);
    }
}
