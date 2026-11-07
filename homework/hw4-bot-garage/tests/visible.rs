//! Visible tests for Homework 4. The grader also runs hidden tests that check
//! every rule in the doc comments, so read those closely.
use bot_garage::bots::{Bot, Drone, Robbie, Vacuum};
use bot_garage::garage::{Garage, GarageError};
use bot_garage::{hungriest, largest};

fn drone(name: &str, rotors: u32) -> Box<dyn Bot> {
    Box::new(Drone {
        name: name.to_string(),
        rotors,
    })
}

#[test]
fn robbie_is_a_bot() {
    let r = Robbie { rust: 20 };
    assert_eq!(r.name(), "Robbie");
    assert_eq!(r.kind(), "studio robot");
    assert_eq!(r.power(), 140);
    assert_eq!(r.label(), "Robbie (studio robot, 140 W)");
}

#[test]
fn drones_and_vacuums() {
    let d = Drone {
        name: "Zippy".to_string(),
        rotors: 4,
    };
    assert_eq!(d.power(), 60);
    assert_eq!(d.label(), "Zippy (drone, 60 W)");
    let v = Vacuum {
        name: "Dusty".to_string(),
        bin_full: false,
    };
    assert_eq!((v.kind(), v.power()), ("vacuum", 40));
}

#[test]
fn rusty_robbie_squeaks() {
    assert_eq!(
        Robbie { rust: 72 }.label(),
        "Robbie (studio robot, 192 W) squeak"
    );
}

#[test]
fn park_and_count_power() {
    let mut g = Garage::new(300);
    g.park(Box::new(Robbie { rust: 0 })).unwrap();
    g.park(drone("Zippy", 4)).unwrap();
    assert_eq!(g.len(), 2);
    assert_eq!(g.total_power(), 180);
    assert_eq!(g.available(), 120);
}

#[test]
fn refuses_duplicates_and_overload() {
    let mut g = Garage::new(100);
    g.park(drone("Zippy", 4)).unwrap();
    assert_eq!(
        g.park(drone("Zippy", 1)),
        Err(GarageError::Duplicate("Zippy".to_string()))
    );
    assert_eq!(
        g.park(drone("Hulk", 8)),
        Err(GarageError::OverBudget {
            needed: 120,
            available: 40
        })
    );
    assert_eq!(g.len(), 1);
}

#[test]
fn error_messages() {
    assert_eq!(
        GarageError::Duplicate("Zippy".to_string()).to_string(),
        "Zippy is already parked"
    );
    assert_eq!(
        GarageError::OverBudget {
            needed: 60,
            available: 20
        }
        .to_string(),
        "needs 60 W, only 20 W left"
    );
    assert_eq!(
        GarageError::NotFound("Zippy".to_string()).to_string(),
        "no bot called Zippy"
    );
}

#[test]
fn names_kinds_and_hogs() {
    let mut g = Garage::new(1000);
    g.park(drone("Zippy", 4)).unwrap();
    g.park(Box::new(Robbie { rust: 10 })).unwrap();
    g.park(drone("Bee", 2)).unwrap();
    assert_eq!(g.names(), ["Bee", "Robbie", "Zippy"]);
    let kinds = g.count_by_kind();
    assert_eq!(kinds["drone"], 2);
    assert_eq!(kinds["studio robot"], 1);
    assert_eq!(
        g.power_hogs(60),
        ["Zippy (drone, 60 W)", "Robbie (studio robot, 130 W)"]
    );
}

#[test]
fn generic_helpers() {
    assert_eq!(largest(&[3, 9, 2]), Some(9));
    assert_eq!(largest(&['r', 'u', 's', 't']), Some('u'));
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
    assert_eq!(hungriest(&drones).map(|d| d.name()), Some("Hornet"));
}
