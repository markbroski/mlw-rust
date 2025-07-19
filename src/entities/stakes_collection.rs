use chrono::Duration;
use chrono::{DateTime, Utc}; // Needed for tests of Stake, indirectly if CreateTestStake is here
use serde::{Deserialize, Serialize}; // Needed for tests that use chrono dates

// Import Stake and StakeId from the parent `entities::stake` module
use super::stake::{Stake, StakeId};

// --- StakesCollection Struct ---
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StakesCollection {
    stakes: Vec<Stake>,
}

impl StakesCollection {
    pub fn new() -> Self {
        StakesCollection { stakes: Vec::new() }
    }

    pub fn add_stake(&mut self, stake: Stake) {
        self.stakes.push(stake);
    }

    pub fn active_stakes(&self) -> Vec<Stake> {
        self.stakes
            .iter()
            .filter(|s| s.is_active())
            .cloned()
            .collect()
    }

    pub fn completed_stakes(&self) -> Vec<Stake> {
        self.stakes.iter().filter(|s| s.complete).cloned().collect()
    }

    pub fn len(&self) -> usize {
        self.stakes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.stakes.is_empty()
    }
}

// --- Unit Tests for StakesCollection ---
#[cfg(test)]
mod tests {
    use super::*; // Imports StakesCollection itself
    // We also need to import Stake, StakeId, etc. from their origin
    // Since create_test_stake is used here and it creates Stake objects,
    // we need to bring Stake and StakeId into scope.
    // They are available through `super::stake` as we use `super::stake::{Stake, StakeId};` above
    // or from the parent entities module if they are re-exported.
    // For simplicity, we can just ensure they're in scope from the re-exports or direct path.
    use crate::entities::stake::{Stake, StakeId}; // Explicit path from crate root
    // For the create_test_stake helper, we might copy it here or make it public in stake.rs.
    // For TDD, let's copy it here temporarily as it's directly needed for these tests.

    // Re-create the helper function here, specifically for these tests.
    // In a larger project, you might have a `test_utilities` module.
    fn create_test_stake(
        id: u32,
        name: &str,
        parent_id: Option<StakeId>,
        complete: bool,
        dropped: bool,
        note: Option<String>,
    ) -> Stake {
        let mut stake = Stake::new(StakeId(id), name.to_string(), parent_id, note);
        stake.complete = complete;
        stake.dropped = dropped;
        stake
    }

    #[test]
    fn test_stakes_collection_new() {
        let collection = StakesCollection::new();
        assert!(
            collection.stakes.is_empty(),
            "New collection should be empty"
        );
    }

    #[test]
    fn test_stakes_collection_add_stake() {
        let mut collection = StakesCollection::new();
        let stake1 = create_test_stake(1, "Stake 1", None, false, false, None);
        let stake2 = create_test_stake(2, "Stake 2", Some(StakeId(1)), true, false, None);

        collection.add_stake(stake1.clone());
        assert_eq!(collection.stakes.len(), 1);
        assert_eq!(collection.stakes[0], stake1);

        collection.add_stake(stake2.clone());
        assert_eq!(collection.stakes.len(), 2);
        assert_eq!(collection.stakes[1], stake2);
    }

    #[test]
    fn test_stakes_collection_active_stakes() {
        let mut collection = StakesCollection::new();
        let active_stake1 = create_test_stake(1, "Active 1", None, false, false, None);
        let completed_stake =
            create_test_stake(2, "Completed", Some(StakeId(1)), true, false, None);
        let dropped_stake = create_test_stake(3, "Dropped", None, false, true, None);
        let active_stake2 = create_test_stake(4, "Active 2", Some(StakeId(1)), false, false, None);

        collection.add_stake(active_stake1.clone());
        collection.add_stake(completed_stake.clone());
        collection.add_stake(dropped_stake.clone());
        collection.add_stake(active_stake2.clone());

        let active_stakes = collection.active_stakes();
        assert_eq!(active_stakes.len(), 2);
        assert!(active_stakes.contains(&active_stake1));
        assert!(active_stakes.contains(&active_stake2));
        assert!(!active_stakes.contains(&completed_stake));
        assert!(!active_stakes.contains(&dropped_stake));
    }

    #[test]
    fn test_stakes_collection_completed_stakes() {
        let mut collection = StakesCollection::new();
        let active_stake = create_test_stake(1, "Active", None, false, false, None);
        let completed_stake1 =
            create_test_stake(2, "Completed 1", Some(StakeId(1)), true, false, None);
        let dropped_stake = create_test_stake(3, "Dropped", None, false, true, None);
        let completed_stake2 =
            create_test_stake(4, "Completed 2", Some(StakeId(1)), true, true, None); // Completed AND Dropped

        collection.add_stake(active_stake.clone());
        collection.add_stake(completed_stake1.clone());
        collection.add_stake(dropped_stake.clone());
        collection.add_stake(completed_stake2.clone());

        let completed_stakes = collection.completed_stakes();
        assert_eq!(completed_stakes.len(), 2);
        assert!(completed_stakes.contains(&completed_stake1));
        assert!(completed_stakes.contains(&completed_stake2));
        assert!(!completed_stakes.contains(&active_stake));
        assert!(!completed_stakes.contains(&dropped_stake));
    }

    #[test]
    fn test_stakes_collection_len() {
        let mut collection = StakesCollection::new();
        assert_eq!(collection.len(), 0, "Empty collection should have length 0");

        collection.add_stake(create_test_stake(1, "A", None, false, false, None));
        assert_eq!(
            collection.len(),
            1,
            "Collection with one stake should have length 1"
        );

        collection.add_stake(create_test_stake(2, "B", None, false, false, None));
        collection.add_stake(create_test_stake(3, "C", None, false, false, None));
        assert_eq!(
            collection.len(),
            3,
            "Collection with three stakes should have length 3"
        );
    }

    // It's also common to add an `is_empty` method:
    #[test]
    fn test_stakes_collection_is_empty() {
        let mut collection = StakesCollection::new();
        assert!(collection.is_empty(), "New collection should be empty");

        collection.add_stake(create_test_stake(1, "A", None, false, false, None));
        assert!(
            !collection.is_empty(),
            "Collection with stakes should not be empty"
        );
    }
}
