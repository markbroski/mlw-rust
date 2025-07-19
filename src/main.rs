#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
// Declare the 'entities' module. This points to src/entities/mod.rs
mod entities;

// Bring the structs and enums into scope from the re-exports in entities/mod.rs
use entities::{Stake, StakeId, StakesCollection}; // StakeError is not used here directly

fn main() {
    let mut areas = StakesCollection::new();
    let mut projects = StakesCollection::new();
    let mut tasks = StakesCollection::new();

    let finance = Stake::new(
        areas.generate_id(),
        "Financial Management".to_string(),
        None,
        None,
    );

    areas.add_stake(finance);
}
