use crate::entities::stake::{Stake, StakeError, StakeId};
use crate::entities::stakes_collection::StakesCollection;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MLW {
    areas: StakesCollection,
    projects: StakesCollection,
    tasks: StakesCollection,
}

impl MLW {
    /// Creates a new MLW instance, initializing its three StakesCollection fields.
    pub fn new() -> Self {
        MLW {
            areas: StakesCollection::new(),
            projects: StakesCollection::new(),
            tasks: StakesCollection::new(),
        }
    }

    // --- Area Management Methods ---
    /// Creates a new area Stake, assigns it an ID, and adds it to the areas collection.
    pub fn new_area(&mut self, name: String, note: Option<String>) -> Stake {
        let id = self.areas.generate_id();
        let new_area_stake = Stake::new(id, name, None, note); // Areas typically have no parent_id
        self.areas.add_stake(new_area_stake.clone()); // Add a clone to the collection
        new_area_stake // Return the owned Stake
    }

    /// Returns a vector of references to active area Stakes.
    pub fn active_areas(&self) -> Vec<&Stake> {
        self.areas.active_stakes()
    }

    /// Returns a vector of references to completed area Stakes.
    pub fn completed_areas(&self) -> Vec<&Stake> {
        self.areas.completed_stakes()
    }

    /// Returns the next available ID for an area Stake without consuming it.
    pub fn next_area_id(&self) -> StakeId {
        self.areas.next_id()
    }

    /// Retrieves a reference to an area Stake by its ID.
    pub fn get_area_by_id(&self, id: &StakeId) -> Option<&Stake> {
        self.areas.get_by_id(id)
    }

    /// Updates an existing area Stake in the collection.
    /// Returns `Ok(())` if the stake was found and updated, `Err(StakeError::StakeNotFound)` otherwise.
    pub fn update_area(&mut self, stake: Stake) -> Result<(), StakeError> {
        self.areas.update_stake(stake)
    }

    /// Marks an area Stake as complete and updates its modified date.
    /// Returns `Ok(())` if the stake was found and updated, `Err(StakeError::StakeNotFound)` otherwise.
    pub fn mark_area_complete(&mut self, id: &StakeId) -> Result<(), StakeError> {
        // Retrieve a mutable copy, modify it, then update in collection
        let mut stake_to_update = self
            .areas
            .get_by_id(id)
            .ok_or(StakeError::StakeNotFound)?
            .clone(); // Clone to get an owned, mutable copy

        stake_to_update.mark_complete();
        self.areas.update_stake(stake_to_update)
    }

    /// Marks an area Stake as dropped and updates its modified date.
    /// Returns `Ok(())` if the stake was found and updated, `Err(StakeError::StakeNotFound)` otherwise.
    pub fn mark_area_dropped(&mut self, id: &StakeId) -> Result<(), StakeError> {
        let mut stake_to_update = self
            .areas
            .get_by_id(id)
            .ok_or(StakeError::StakeNotFound)?
            .clone();

        stake_to_update.mark_dropped();
        self.areas.update_stake(stake_to_update)
    }

    // --- Project Management Methods (Placeholder - you'll build these out next) ---
    pub fn new_project(
        &mut self,
        name: String,
        parent_id: Option<StakeId>,
        note: Option<String>,
    ) -> Stake {
        let id = self.projects.generate_id();
        let new_project_stake = Stake::new(id, name, parent_id, note);
        self.projects.add_stake(new_project_stake.clone());
        new_project_stake
    }
    pub fn active_projects(&self) -> Vec<&Stake> {
        self.projects.active_stakes()
    }
    pub fn completed_projects(&self) -> Vec<&Stake> {
        self.projects.completed_stakes()
    }
    pub fn next_project_id(&self) -> StakeId {
        self.projects.next_id()
    }
    pub fn get_project_by_id(&self, id: &StakeId) -> Option<&Stake> {
        self.projects.get_by_id(id)
    }
    pub fn update_project(&mut self, stake: Stake) -> Result<(), StakeError> {
        self.projects.update_stake(stake)
    }
    pub fn mark_project_complete(&mut self, id: &StakeId) -> Result<(), StakeError> {
        let mut stake_to_update = self
            .projects
            .get_by_id(id)
            .ok_or(StakeError::StakeNotFound)?
            .clone();
        stake_to_update.mark_complete();
        self.projects.update_stake(stake_to_update)
    }
    pub fn mark_project_dropped(&mut self, id: &StakeId) -> Result<(), StakeError> {
        let mut stake_to_update = self
            .projects
            .get_by_id(id)
            .ok_or(StakeError::StakeNotFound)?
            .clone();
        stake_to_update.mark_dropped();
        self.projects.update_stake(stake_to_update)
    }
    pub fn get_project_children(&self, parent_id: &StakeId) -> Vec<&Stake> {
        self.projects.get_children(parent_id)
    }

    // --- Task Management Methods (Placeholder - you'll build these out next) ---
    pub fn new_task(
        &mut self,
        name: String,
        parent_id: Option<StakeId>,
        note: Option<String>,
    ) -> Stake {
        let id = self.tasks.generate_id();
        let new_task_stake = Stake::new(id, name, parent_id, note);
        self.tasks.add_stake(new_task_stake.clone());
        new_task_stake
    }
    pub fn active_tasks(&self) -> Vec<&Stake> {
        self.tasks.active_stakes()
    }
    pub fn completed_tasks(&self) -> Vec<&Stake> {
        self.tasks.completed_stakes()
    }
    pub fn next_task_id(&self) -> StakeId {
        self.tasks.next_id()
    }
    pub fn get_task_by_id(&self, id: &StakeId) -> Option<&Stake> {
        self.tasks.get_by_id(id)
    }
    pub fn update_task(&mut self, stake: Stake) -> Result<(), StakeError> {
        self.tasks.update_stake(stake)
    }
    pub fn mark_task_complete(&mut self, id: &StakeId) -> Result<(), StakeError> {
        let mut stake_to_update = self
            .tasks
            .get_by_id(id)
            .ok_or(StakeError::StakeNotFound)?
            .clone();
        stake_to_update.mark_complete();
        self.tasks.update_stake(stake_to_update)
    }
    pub fn mark_task_dropped(&mut self, id: &StakeId) -> Result<(), StakeError> {
        let mut stake_to_update = self
            .tasks
            .get_by_id(id)
            .ok_or(StakeError::StakeNotFound)?
            .clone();
        stake_to_update.mark_dropped();
        self.tasks.update_stake(stake_to_update)
    }
    pub fn get_task_children(&self, parent_id: &StakeId) -> Vec<&Stake> {
        self.tasks.get_children(parent_id)
    }
}

// --- Unit Tests for MLW ---
#[cfg(test)]
mod tests {
    use super::*; // Imports MLW itself
    use crate::entities::stake::{Stake, StakeId}; // Explicitly import for tests
    use crate::entities::stakes_collection::StakesCollection; // Explicitly import for tests
    use chrono::{TimeZone, Utc}; // For fixed_time in tests
    use serde_json; // For serialization/deserialization

    // Helper function (copied here for self-contained tests)
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
    fn test_mlw_new() {
        let mlw = MLW::new();

        assert_eq!(mlw.areas.len(), 0, "Areas collection should be empty");
        assert!(mlw.areas.is_empty(), "Areas collection should be empty");
        assert_eq!(
            mlw.areas.next_id(),
            StakeId(1),
            "Areas next_id should start at 1"
        ); // Using public next_id() method

        assert_eq!(mlw.projects.len(), 0, "Projects collection should be empty");
        assert!(
            mlw.projects.is_empty(),
            "Projects collection should be empty"
        );
        assert_eq!(
            mlw.projects.next_id(),
            StakeId(1),
            "Projects next_id should start at 1"
        ); // Using public next_id() method

        assert_eq!(mlw.tasks.len(), 0, "Tasks collection should be empty");
        assert!(mlw.tasks.is_empty(), "Tasks collection should be empty");
        assert_eq!(
            mlw.tasks.next_id(),
            StakeId(1),
            "Tasks next_id should start at 1"
        ); // Using public next_id() method
    }

    #[test]
    fn test_mlw_serialization_roundtrip() {
        let mut original_mlw = MLW::new();

        // Populate collections with some test data
        let fixed_time = Utc.with_ymd_and_hms(2024, 7, 19, 8, 30, 0).unwrap();

        // AREA COLLECTION SETUP:
        // Area stake will have ID 1. To ensure areas.next_id is 2 after adding it,
        // we call generate_id() once, then use that ID for the stake.
        let area_stake_id = original_mlw.areas.generate_id(); // area_stake_id is StakeId(1), areas.next_id becomes StakeId(2)
        let mut area_stake =
            create_test_stake(area_stake_id.0, "Finance Area", None, false, false, None);
        area_stake.date_created = fixed_time;
        area_stake.date_modified = fixed_time;
        original_mlw.areas.add_stake(area_stake.clone());

        // PROJECT COLLECTION SETUP:
        // Project stake will have ID 10. To ensure projects.next_id is 11 after adding it,
        // we call generate_id() 10 times (1 for the actual stake, 9 to advance it).
        for _ in 0..9 {
            // Advance projects.next_id to 10 (generate IDs 1-9)
            original_mlw.projects.generate_id();
        }
        let project_stake_id = original_mlw.projects.generate_id(); // project_stake_id is StakeId(10), projects.next_id becomes StakeId(11)
        let mut project_stake = create_test_stake(
            project_stake_id.0,
            "Website Project",
            Some(area_stake_id.clone()),
            false,
            false,
            Some("Client work".to_string()),
        );
        project_stake.date_created = fixed_time;
        project_stake.date_modified = fixed_time;
        original_mlw.projects.add_stake(project_stake.clone());

        // TASK COLLECTION SETUP:
        // Task stake will have ID 100. To ensure tasks.next_id is 101 after adding it,
        // we call generate_id() 100 times.
        for _ in 0..99 {
            // Advance tasks.next_id to 100 (generate IDs 1-99)
            original_mlw.tasks.generate_id();
        }
        let task_stake_id = original_mlw.tasks.generate_id(); // task_stake_id is StakeId(100), tasks.next_id becomes StakeId(101)
        let mut task_stake = create_test_stake(
            task_stake_id.0,
            "Design Layout",
            Some(project_stake_id.clone()),
            false,
            false,
            None,
        );
        task_stake.date_created = fixed_time;
        task_stake.date_modified = fixed_time;
        original_mlw.tasks.add_stake(task_stake.clone());

        // 1. Serialize the original MLW object
        let serialized_mlw =
            serde_json::to_string_pretty(&original_mlw).expect("Failed to serialize MLW object");
        println!("Serialized MLW object:\n{}", serialized_mlw);

        // 2. Deserialize the JSON string back into a new MLW object
        let deserialized_mlw: MLW =
            serde_json::from_str(&serialized_mlw).expect("Failed to deserialize MLW object");

        // 3. Compare the original and deserialized objects
        assert_eq!(
            original_mlw, deserialized_mlw,
            "Original and deserialized MLW objects should be identical"
        );
    }
}
