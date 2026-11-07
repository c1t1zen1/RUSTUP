// Puzzle 6 solution: return the String itself, handing ownership to the caller.
fn make_callsign(unit: u32) -> String {
    format!("ROBBIE-{unit:03}")
}

fn main() {
    let callsign = make_callsign(7);
    println!("callsign: {callsign}");
}
