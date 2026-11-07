//! The bots, and the trait every bot shares.

/// Anything that can be parked in the garage.
pub trait Bot {
    /// The bot's name, like "Robbie".
    fn name(&self) -> &str;

    /// What kind of bot it is, like "drone". The same for every bot of a type.
    fn kind(&self) -> &'static str;

    /// Watts the bot draws while it's working.
    fn power(&self) -> u32;

    /// A one-line label, like "Zippy (drone, 60 W)". This one is written for you,
    /// and any bot can replace it with its own.
    fn label(&self) -> String {
        format!("{} ({}, {} W)", self.name(), self.kind(), self.power())
    }
}

/// Robbie himself. There is only one Robbie, so he has no name field.
#[derive(Debug, Clone, PartialEq)]
pub struct Robbie {
    /// How much of him is rust, 0 to 100.
    pub rust: u8,
}

/// A flying camera drone.
#[derive(Debug, Clone, PartialEq)]
pub struct Drone {
    pub name: String,
    pub rotors: u32,
}

/// The studio's robot vacuum.
#[derive(Debug, Clone, PartialEq)]
pub struct Vacuum {
    pub name: String,
    pub bin_full: bool,
}

/// Rules for Robbie:
/// - name is "Robbie", kind is "studio robot"
/// - power is 120 W, plus 1 W for every percent of rust (rusty joints work harder)
/// - label: the usual label, but when he is 50% rust or more, add " squeak" at the end,
///   like "Robbie (studio robot, 170 W) squeak"
impl Bot for Robbie {
    fn name(&self) -> &str {
        todo!()
    }
    fn kind(&self) -> &'static str {
        todo!()
    }
    fn power(&self) -> u32 {
        todo!("{}", self.rust)
    }
    fn label(&self) -> String {
        todo!("replace the default label for Robbie")
    }
}

/// Rules for a drone: kind is "drone", power is 15 W per rotor.
/// Don't write label here: the trait's default one is right for drones.
impl Bot for Drone {
    fn name(&self) -> &str {
        todo!("{}", self.name)
    }
    fn kind(&self) -> &'static str {
        todo!()
    }
    fn power(&self) -> u32 {
        todo!("{}", self.rotors)
    }
}

/// Rules for a vacuum: kind is "vacuum", power is 40 W, or 55 W when the bin is full.
impl Bot for Vacuum {
    fn name(&self) -> &str {
        todo!("{}", self.name)
    }
    fn kind(&self) -> &'static str {
        todo!()
    }
    fn power(&self) -> u32 {
        todo!("{}", self.bin_full)
    }
}
