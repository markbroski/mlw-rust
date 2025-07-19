// Declare the 'entities' module. This points to src/entities/mod.rs
mod entities;

// Bring the structs and enums into scope from the re-exports in entities/mod.rs
use entities::{Stake, StakeId, StakesCollection}; // StakeError is not used here directly

fn main() {
    // --- Demonstrate Stake usage ---
    let stake_id_1 = StakeId(1);
    let parent_id_for_stake1 = Some(StakeId(99)); // Example parent ID

    let mut my_stake = Stake::new(
        stake_id_1,
        "Website Redesign Stake".to_string(),
        parent_id_for_stake1, // <--- Pass parent_id
        Some("Initial setup notes".to_string()),
    );

    println!("Initial Stake: {:?}", my_stake);
    println!("Is active: {}", my_stake.is_active());

    my_stake.mark_complete();
    println!("Stake after completion: {:?}", my_stake);
    println!("Is active: {}", my_stake.is_active());

    let stake_id_2 = StakeId(2);
    let parent_id_for_stake2 = None; // Example: no parent
    let mut another_stake = Stake::new(
        stake_id_2,
        "Mobile App Development Stake".to_string(),
        parent_id_for_stake2, // <--- Pass parent_id
        None,                 // No note for this one
    );
    println!("Another Stake: {:?}", another_stake);
    println!("Is active: {}", another_stake.is_active());

    another_stake.mark_dropped();
    println!("Another Stake after being dropped: {:?}", another_stake);
    println!("Is active: {}", another_stake.is_active());

    println!("\n--- StakesCollection Demo ---");

    let mut collection = StakesCollection::new();

    // Now creating stakes for the collection also requires parent_id
    let stake_3 = Stake::new(
        StakeId(3),
        "Active Task A".to_string(),
        None,
        Some("Active and healthy".to_string()),
    );
    let stake_4 = {
        let mut s = Stake::new(
            StakeId(4),
            "Completed Task B".to_string(),
            Some(StakeId(3)),
            None,
        );
        s.mark_complete();
        s
    };
    let stake_5 = {
        let mut s = Stake::new(
            StakeId(5),
            "Dropped Task C".to_string(),
            None,
            Some("Was dropped due to scope change".to_string()),
        );
        s.mark_dropped();
        s
    };
    let stake_6 = Stake::new(
        StakeId(6),
        "Active Task D".to_string(),
        Some(StakeId(3)),
        None,
    );

    collection.add_stake(stake_3);
    collection.add_stake(stake_4);
    collection.add_stake(stake_5);
    collection.add_stake(stake_6);

    println!("Total stakes in collection: {}", collection.len());

    let active_stakes = collection.active_stakes();
    println!("Active stakes ({}):", active_stakes.len());
    for s in active_stakes {
        println!("  - {:?}", s.stake_name);
    }

    let completed_stakes = collection.completed_stakes();
    println!("Completed stakes ({}):", completed_stakes.len());
    for s in completed_stakes {
        println!("  - {:?}", s.stake_name);
    }
}
