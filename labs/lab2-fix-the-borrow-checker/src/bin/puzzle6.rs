// Puzzle 6: The Dangling Antenna
// make_callsign() builds a String and tries to hand back a reference to it.
// Run: cargo run --bin puzzle6
// Goal: return something that outlives the function.

fn make_callsign(unit: u32) -> &String {
    let sign = format!("ROBBIE-{unit:03}");
    &sign
}

fn main() {
    let callsign = make_callsign(7);
    println!("callsign: {callsign}");
}
