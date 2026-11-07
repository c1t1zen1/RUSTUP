use std::{env, fs, process};
use swarm_sim::config::Config;
use swarm_sim::sim;

fn main() {
    let path = env::args()
        .nth(1)
        .unwrap_or_else(|| "swarm.json".to_string());
    let text = fs::read_to_string(&path).unwrap_or_else(|e| {
        eprintln!("can't open {path}: {e}");
        process::exit(1);
    });
    let config = Config::from_json(&text).unwrap_or_else(|e| {
        eprintln!("bad config in {path}: {e}");
        process::exit(1);
    });

    println!(
        "{} bots on a {}x{} floor.\n",
        config.bots.len(),
        config.width(),
        config.height()
    );
    let report = sim::run(&config);
    for b in &report.bots {
        println!(
            "  {:<8} ended at {:?}  {:>2} moves  {} bumps",
            b.name, b.end, b.moves, b.bumps
        );
    }
    println!(
        "\nCleaned {} units, {} left:",
        report.cleaned, report.dirt_left
    );
    for row in &report.floor {
        let tiles: Vec<String> = row.iter().map(|d| d.to_string()).collect();
        println!("  {}", tiles.join(" "));
    }
    fs::write("report.json", report.to_json()).expect("can't write report.json");
    println!("\nFull report written to report.json");
}
