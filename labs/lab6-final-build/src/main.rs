use ship_it::{beep, c_abs, c_strlen, loudest, split_halves, status};

fn main() {
    println!("{}\n", status());
    println!("Robbie says: {}", beep!(3));
    println!("Loudest squeak: {} decibels", loudest!(42, 87, 63));

    let mut joints = [5, 9, 2, 7];
    let (left, right) = split_halves(&mut joints);
    left[0] = 0;
    right[0] = 0;
    println!("Oiled the first joint on each side: {joints:?}");

    println!("C says |-12| is {:?}", c_abs(-12));
    println!("C says \"rustup\" is {} bytes long", c_strlen(c"rustup"));
    println!("\nRun the benchmark with: cargo run --release --example bench");
}
