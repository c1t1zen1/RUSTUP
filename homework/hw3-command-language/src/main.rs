//! Runs Robbie's studio tour script, then a broken one. This file is finished.
use command_language::run_script;

const TOUR: &str = "\
# Robbie's studio tour
say Welcome to the studio!
forward 4
right
forward 3
say This is the LED wall. It's very bright. I'm very orange.
report
back 2
left
charge
report
";

const OOPS: &str = "\
forward 2
moonwalk 3
";

fn main() {
    for (name, script) in [("tour", TOUR), ("oops", OOPS)] {
        println!("== {name} ==");
        match run_script(script) {
            Ok((robbie, said)) => {
                for line in said {
                    println!("{line}");
                }
                println!("Finished at {}", robbie.status());
            }
            Err(e) => println!("Script stopped. {e}"),
        }
    }
}
