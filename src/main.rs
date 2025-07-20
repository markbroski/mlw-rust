#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
// Declare the 'entities' module. This points to src/entities/mod.rs
mod entities;
mod mlw;
// Bring the structs and enums into scope from the re-exports in entities/mod.rs
use entities::{Stake, StakeId, StakesCollection};
use mlw::MLW; // StakeError is not used here directly

fn main() {
    let mlw_app = MLW::new();
    let mut areas = StakesCollection::new();
    // let projects = StakesCollection::new();
    // let mut tasks = StakesCollection::new();

    let mut finance = Stake::new(
        areas.generate_id(),
        "Financial Management".to_string(),
        None,
        None,
    );

    areas.add_stake(finance.clone());

    finance.mark_complete();
    let id = finance.stake_id.clone();
    let _ = areas.update_stake(finance);
    println!(
        "Debug representation of my_stake: {:?}",
        areas.get_by_id(&id)
    );
}
