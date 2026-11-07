use bot_garage::bots::{Bot, Drone, Robbie, Vacuum};
use bot_garage::garage::Garage;
use bot_garage::{hungriest, largest};

fn main() {
    let mut garage = Garage::new(400);
    let arrivals: Vec<Box<dyn Bot>> = vec![
        Box::new(Robbie { rust: 50 }),
        Box::new(Drone {
            name: "Zippy".to_string(),
            rotors: 4,
        }),
        Box::new(Vacuum {
            name: "Dusty".to_string(),
            bin_full: true,
        }),
        Box::new(Drone {
            name: "zippy".to_string(),
            rotors: 6,
        }),
        Box::new(Drone {
            name: "Hulk".to_string(),
            rotors: 8,
        }),
    ];
    for bot in arrivals {
        let label = bot.label();
        match garage.park(bot) {
            Ok(()) => println!("parked  {label}"),
            Err(e) => println!("refused {label}: {e}"),
        }
    }
    println!(
        "\n{} bots, {} W used, {} W free",
        garage.len(),
        garage.total_power(),
        garage.available()
    );
    println!("names: {:?}", garage.names());
    println!("power hogs (100 W+): {:?}", garage.power_hogs(100));

    let drones = [
        Drone {
            name: "Bee".to_string(),
            rotors: 2,
        },
        Drone {
            name: "Hornet".to_string(),
            rotors: 6,
        },
    ];
    if let Some(d) = hungriest(&drones) {
        println!("hungriest drone: {}", d.label());
    }
    println!("largest battery reading: {:?}", largest(&[87, 42, 99, 13]));
}
