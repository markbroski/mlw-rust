use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct StakeId(pub u32);

// --- Custom Error Enum for Stake operations ---
#[derive(Debug, PartialEq, Eq)]
#[allow(dead_code)]
pub enum StakeError {
    // Note: Since `activate` method was removed, this error is not currently
    // returned by any method in the `Stake` struct.
    // If you re-introduce a method that can fail with this error, add a test for it.
    CannotActivateDroppedStake,
    // Add other specific errors here later if needed
}

// --- Stake Struct (Entity) ---
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Stake {
    pub stake_id: StakeId,
    pub stake_name: String,
    pub parent_id: Option<StakeId>, // Changed from AreaId to Option<StakeId>
    pub complete: bool,
    pub dropped: bool,
    pub note: Option<String>,
    pub date_modified: DateTime<Utc>,
    pub date_created: DateTime<Utc>,
}

impl Stake {
    /// Creates a new Stake instance with initial values.
    /// `date_created` and `date_modified` are set to the current UTC time.
    pub fn new(
        stake_id: StakeId,
        stake_name: String,
        parent_id: Option<StakeId>,
        note: Option<String>,
    ) -> Self {
        let now = Utc::now();
        Stake {
            stake_id,
            stake_name,
            parent_id,
            complete: false, // Stakes typically start as incomplete
            dropped: false,  // Stakes typically start as not dropped
            note,
            date_modified: now,
            date_created: now,
        }
    }

    /// Marks the stake as complete and updates `date_modified`.
    pub fn mark_complete(&mut self) {
        self.complete = true;
        self.date_modified = Utc::now();
    }

    /// Marks the stake as dropped and updates `date_modified`.
    /// Dropped stakes are implicitly inactive.
    pub fn mark_dropped(&mut self) {
        self.dropped = true;
        self.date_modified = Utc::now();
    }

    /// Computes whether the stake is currently active based on its complete and dropped status.
    /// Logic: active = !dropped AND !complete
    pub fn is_active(&self) -> bool {
        !self.dropped && !self.complete
    }
}

// --- Unit Tests ---
// This module will only be compiled and run when `cargo test` is executed.
#[cfg(test)]
mod tests {
    use super::*; // `super::*` now refers to the `stake` module itself
    use chrono::Duration; // For date comparisons in tests

    // Helper function to create a basic stake for tests
    // Now accepts an Option<StakeId> for parent_id
    fn create_test_stake(parent_id: Option<StakeId>) -> Stake {
        Stake::new(StakeId(100), "Test Stake Name".to_string(), parent_id, None)
    }

    #[test]
    fn test_stake_new_with_parent() {
        let stake_id = StakeId(1);
        let parent_id = Some(StakeId(10)); // Test with a parent
        let stake_name = "New Initiative".to_string();
        let now_before_creation = Utc::now();

        let stake = Stake::new(
            stake_id.clone(),
            stake_name.clone(),
            parent_id.clone(),
            None,
        );

        // Assert initial state
        assert_eq!(stake.stake_id, stake_id);
        assert_eq!(stake.stake_name, stake_name);
        assert_eq!(stake.parent_id, parent_id); // Check parent_id
        assert!(!stake.complete);
        assert!(!stake.dropped);
        assert!(stake.is_active(), "New stake should be active");

        // Check dates are approximately now
        assert!(stake.date_created >= now_before_creation);
        assert!(stake.date_modified >= now_before_creation);
        assert!(stake.date_created <= Utc::now() + Duration::seconds(1));
        assert!(stake.date_modified <= Utc::now() + Duration::seconds(1));
        assert_eq!(
            stake.date_created, stake.date_modified,
            "Initially, created and modified dates should be the same"
        );
    }

    #[test]
    fn test_stake_new_without_parent() {
        let stake_id = StakeId(2);
        let parent_id = None; // Test without a parent
        let stake_name = "Root Stake".to_string();
        let now_before_creation = Utc::now();

        let stake = Stake::new(stake_id.clone(), stake_name.clone(), parent_id, None);

        // Assert initial state
        assert_eq!(stake.stake_id, stake_id);
        assert_eq!(stake.stake_name, stake_name);
        assert_eq!(stake.parent_id, None); // Check parent_id is None
        assert!(!stake.complete);
        assert!(!stake.dropped);
        assert!(stake.is_active(), "New stake should be active");

        // Check dates are approximately now
        assert!(stake.date_created >= now_before_creation);
        assert!(stake.date_modified >= now_before_creation);
        assert!(stake.date_created <= Utc::now() + Duration::seconds(1));
        assert!(stake.date_modified <= Utc::now() + Duration::seconds(1));
        assert_eq!(
            stake.date_created, stake.date_modified,
            "Initially, created and modified dates should be the same"
        );
    }

    #[test]
    fn test_mark_complete() {
        // Use create_test_stake with a parent for a standard test case
        let mut stake = create_test_stake(Some(StakeId(999)));
        let initial_modified_date = stake.date_modified;
        let now_before_complete = Utc::now();

        std::thread::sleep(std::time::Duration::from_millis(10));

        stake.mark_complete();

        assert!(stake.complete);
        assert!(!stake.dropped);
        assert!(!stake.is_active(), "Completed stake should become inactive");
        assert!(
            stake.date_modified > initial_modified_date,
            "date_modified should be updated"
        );
        assert!(
            stake.date_modified >= now_before_complete,
            "date_modified should be updated to current time"
        );
    }

    #[test]
    fn test_mark_dropped() {
        // Use create_test_stake with no parent for a standard test case
        let mut stake = create_test_stake(None);
        let initial_modified_date = stake.date_modified;
        let now_before_dropped = Utc::now();

        std::thread::sleep(std::time::Duration::from_millis(10));

        stake.mark_dropped();

        assert!(!stake.complete);
        assert!(stake.dropped);
        assert!(!stake.is_active(), "Dropped stake should become inactive");
        assert!(
            stake.date_modified > initial_modified_date,
            "date_modified should be updated"
        );
        assert!(
            stake.date_modified >= now_before_dropped,
            "date_modified should be updated to current time"
        );
    }

    #[test]
    fn test_is_active_logic() {
        // Use create_test_stake with arbitrary parent for this logic test
        let mut stake = create_test_stake(Some(StakeId(50))); // Starts: complete=false, dropped=false -> active

        assert!(stake.is_active(), "Fresh stake should be active");

        // Case 1: Complete = true, Dropped = false
        stake.complete = true;
        stake.dropped = false;
        assert!(!stake.is_active(), "Completed stake should be inactive");

        // Case 2: Complete = false, Dropped = true
        stake.complete = false; // Reset for this case
        stake.dropped = true;
        assert!(!stake.is_active(), "Dropped stake should be inactive");

        // Case 3: Complete = true, Dropped = true (should still be inactive)
        stake.complete = true;
        stake.dropped = true;
        assert!(
            !stake.is_active(),
            "Completed and dropped stake should be inactive"
        );

        // Revert to active state (for testing purposes, if allowed by business rules)
        stake.complete = false;
        stake.dropped = false;
        assert!(stake.is_active(), "Reset stake should be active");
    }
}
