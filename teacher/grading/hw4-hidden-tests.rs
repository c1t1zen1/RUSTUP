//! Hidden tests for Homework 4: Bot Garage.
//! Copy into the student's tests/ folder as tests/hidden.rs, then: cargo test --test hidden
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
fn squeak_starts_at_exactly_50() {
    assert_eq!(Robbie { rust: 49 }.label(), "Robbie (studio robot, 169 W)");
    assert_eq!(
        Robbie { rust: 50 }.label(),
        "Robbie (studio robot, 170 W) squeak"
    );
    assert_eq!(Robbie { rust: 100 }.power(), 220);
}

#[test]
fn full_bin_and_zero_rotors() {
    let v = Vacuum {
        name: "Dusty".to_string(),
        bin_full: true,
    };
    assert_eq!(v.label(), "Dusty (vacuum, 55 W)");
    let d = Drone {
        name: "Brick".to_string(),
        rotors: 0,
    };
    assert_eq!(d.power(), 0);
}

#[test]
fn duplicate_names_ignore_case_and_come_first() {
    let mut g = Garage::new(60);
    g.park(drone("Zippy", 4)).unwrap();
    // over budget too, but the name check comes first
    assert_eq!(
        g.park(drone("ZIPPY", 9)),
        Err(GarageError::Duplicate("ZIPPY".to_string()))
    );
    assert_eq!(
        g.park(Box::new(Robbie { rust: 0 })),
        Err(GarageError::OverBudget {
            needed: 120,
            available: 0
        })
    );
}

#[test]
fn exact_budget_fits() {
    let mut g = Garage::new(120);
    assert!(g.park(Box::new(Robbie { rust: 0 })).is_ok());
    assert_eq!(g.available(), 0);
    assert!(g.park(drone("Brick", 0)).is_ok());
}

#[test]
fn refused_bot_changes_nothing() {
    let mut g = Garage::new(100);
    g.park(drone("Bee", 2)).unwrap();
    let before = (g.len(), g.total_power(), g.names().join(","));
    let _ = g.park(drone("Hulk", 8));
    let _ = g.park(drone("bee", 1));
    assert_eq!(before, (g.len(), g.total_power(), g.names().join(",")));
}

#[test]
fn unpark_frees_power_and_keeps_order() {
    let mut g = Garage::new(500);
    g.park(drone("A", 1)).unwrap();
    g.park(drone("B", 2)).unwrap();
    g.park(drone("C", 3)).unwrap();
    let b = g.unpark("b").unwrap();
    assert_eq!((b.name(), b.power()), ("B", 30));
    assert_eq!(g.total_power(), 60);
    assert_eq!(g.power_hogs(0), ["A (drone, 15 W)", "C (drone, 45 W)"]);
    assert_eq!(
        g.unpark("B").err(),
        Some(GarageError::NotFound("B".to_string()))
    );
}

#[test]
fn empty_garage() {
    let g = Garage::new(50);
    assert!(g.is_empty());
    assert_eq!(g.total_power(), 0);
    assert_eq!(g.available(), 50);
    assert!(g.names().is_empty());
    assert!(g.count_by_kind().is_empty());
    assert!(g.power_hogs(0).is_empty());
}

#[test]
fn largest_edge_cases() {
    let none: [u8; 0] = [];
    assert_eq!(largest(&none), None);
    assert_eq!(largest(&[-5, -2, -9]), Some(-2));
    assert_eq!(largest(&[1.5, 2.25, 0.5]), Some(2.25));
    assert_eq!(largest(&[7]), Some(7));
}

#[test]
fn hungriest_ties_go_to_the_first() {
    let vacs = [
        Vacuum {
            name: "First".to_string(),
            bin_full: true,
        },
        Vacuum {
            name: "Second".to_string(),
            bin_full: true,
        },
    ];
    assert_eq!(hungriest(&vacs).map(|v| v.name()), Some("First"));
    let none: [Robbie; 0] = [];
    assert!(hungriest(&none).is_none());
}
