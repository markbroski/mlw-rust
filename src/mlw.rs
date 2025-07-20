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
    use super::*;
    use crate::entities::stake::{Stake, StakeError, StakeId};
    use crate::entities::stakes_collection::StakesCollection;
    use chrono::{TimeZone, Utc};
    use serde_json;

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

    // Existing test_mlw_new and test_mlw_serialization_roundtrip
    // Keep these two.
    #[test]
    fn test_mlw_new() {
        let mlw = MLW::new();
        assert_eq!(mlw.areas.len(), 0, "Areas collection should be empty");
        assert!(mlw.areas.is_empty(), "Areas collection should be empty");
        assert_eq!(
            mlw.areas.next_id(),
            StakeId(1),
            "Areas next_id should start at 1"
        );
        assert_eq!(mlw.projects.len(), 0, "Projects collection should be empty");
        assert!(
            mlw.projects.is_empty(),
            "Projects collection should be empty"
        );
        assert_eq!(
            mlw.projects.next_id(),
            StakeId(1),
            "Projects next_id should start at 1"
        );
        assert_eq!(mlw.tasks.len(), 0, "Tasks collection should be empty");
        assert!(mlw.tasks.is_empty(), "Tasks collection should be empty");
        assert_eq!(
            mlw.tasks.next_id(),
            StakeId(1),
            "Tasks next_id should start at 1"
        );
    }

    #[test]
    fn test_mlw_serialization_roundtrip() {
        let mut original_mlw = MLW::new();
        let fixed_time = Utc.with_ymd_and_hms(2024, 7, 19, 8, 30, 0).unwrap();

        let area_stake_id = original_mlw.areas.generate_id();
        let mut area_stake =
            create_test_stake(area_stake_id.0, "Finance Area", None, false, false, None);
        area_stake.date_created = fixed_time;
        area_stake.date_modified = fixed_time;
        original_mlw.areas.add_stake(area_stake.clone());

        for _ in 0..9 {
            original_mlw.projects.generate_id();
        }
        let project_stake_id = original_mlw.projects.generate_id();
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

        for _ in 0..99 {
            original_mlw.tasks.generate_id();
        }
        let task_stake_id = original_mlw.tasks.generate_id();
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

        let serialized_mlw =
            serde_json::to_string_pretty(&original_mlw).expect("Failed to serialize MLW object");
        println!("Serialized MLW object:\n{}", serialized_mlw);

        let deserialized_mlw: MLW =
            serde_json::from_str(&serialized_mlw).expect("Failed to deserialize MLW object");

        assert_eq!(
            original_mlw, deserialized_mlw,
            "Original and deserialized MLW objects should be identical"
        );
    }

    // REMOVED: test_mlw_area_management (will be replaced by smaller tests)
    // REMOVED: test_mlw_project_management (will be replaced by smaller tests)
    // REMOVED: test_mlw_task_management (will be replaced by smaller tests)

    // --- NEW: Granular Tests for MLW Area Management Methods ---

    #[test]
    fn test_mlw_new_area_id_correctly_generated() {
        let mut mlw = MLW::new();
        let area = mlw.new_area("Test Area".to_string(), None);
        assert_eq!(area.stake_id, StakeId(1));
    }

    #[test]
    fn test_mlw_new_area_name_correctly_set() {
        let mut mlw = MLW::new();
        let area = mlw.new_area("Test Area Name".to_string(), None);
        assert_eq!(area.stake_name, "Test Area Name");
    }

    #[test]
    fn test_mlw_new_area_note_correctly_set() {
        let mut mlw = MLW::new();
        let area = mlw.new_area("Test Area".to_string(), Some("A note".to_string()));
        assert_eq!(area.note, Some("A note".to_string()));
    }

    #[test]
    fn test_mlw_new_area_starts_active() {
        let mut mlw = MLW::new();
        let area = mlw.new_area("Test Area".to_string(), None);
        assert!(area.is_active());
    }

    #[test]
    fn test_mlw_new_area_adds_to_collection() {
        let mut mlw = MLW::new();
        mlw.new_area("Test Area".to_string(), None);
        assert_eq!(mlw.areas.len(), 1); // Check internal collection length
    }

    #[test]
    fn test_mlw_new_area_increments_next_id() {
        let mut mlw = MLW::new();
        mlw.new_area("Test Area".to_string(), None);
        assert_eq!(mlw.next_area_id(), StakeId(2));
    }

    #[test]
    fn test_mlw_new_area_has_no_parent_id() {
        let mut mlw = MLW::new();
        let area = mlw.new_area("Test Area".to_string(), None);
        assert_eq!(area.parent_id, None);
    }

    // --- NEW: Granular Tests for MLW Active/Completed Areas ---

    #[test]
    fn test_mlw_active_areas_returns_only_active() {
        let mut mlw = MLW::new();
        let active1 = mlw.new_area("Active 1".to_string(), None);
        let mut completed1 = mlw.new_area("Completed 1".to_string(), None);
        let _ = mlw.mark_area_complete(&completed1.stake_id); // Update in collection
        let active2 = mlw.new_area("Active 2".to_string(), None);

        let active_areas = mlw.active_areas();
        assert_eq!(active_areas.len(), 2);
        assert!(active_areas.contains(&&active1));
        assert!(active_areas.contains(&&active2));
        assert!(
            !active_areas
                .iter()
                .any(|&s| s.stake_id == completed1.stake_id)
        ); // Use any for completed
    }

    #[test]
    fn test_mlw_completed_areas_returns_only_completed() {
        let mut mlw = MLW::new();
        let active1 = mlw.new_area("Active 1".to_string(), None);
        let mut completed1 = mlw.new_area("Completed 1".to_string(), None);
        let _ = mlw.mark_area_complete(&completed1.stake_id); // Update in collection
        let active2 = mlw.new_area("Active 2".to_string(), None);

        let completed_areas = mlw.completed_areas();
        assert_eq!(completed_areas.len(), 1);
        let updated_completed1 = mlw.get_area_by_id(&completed1.stake_id).unwrap();
        assert!(completed_areas.contains(&updated_completed1)); // Compare with updated state
        assert!(
            !completed_areas
                .iter()
                .any(|&s| s.stake_id == active1.stake_id)
        );
        assert!(
            !completed_areas
                .iter()
                .any(|&s| s.stake_id == active2.stake_id)
        );
    }

    // --- NEW: Granular Tests for MLW Get Area By ID ---

    #[test]
    fn test_mlw_get_area_by_id_found() {
        let mut mlw = MLW::new();
        let area = mlw.new_area("Test Area".to_string(), None);
        let retrieved_area = mlw.get_area_by_id(&area.stake_id);
        assert!(retrieved_area.is_some());
        assert_eq!(retrieved_area.unwrap(), &area);
    }

    #[test]
    fn test_mlw_get_area_by_id_not_found() {
        let mlw = MLW::new();
        let not_found = mlw.get_area_by_id(&StakeId(999));
        assert!(not_found.is_none());
    }

    // --- NEW: Granular Tests for MLW Update Area ---

    #[test]
    fn test_mlw_update_area_success() {
        let mut mlw = MLW::new();
        let original_area = mlw.new_area(
            "Original Name".to_string(),
            Some("Original Note".to_string()),
        );
        let mut updated_area = original_area.clone();
        updated_area.stake_name = "New Name".to_string();
        updated_area.note = Some("New Note".to_string());

        let result = mlw.update_area(updated_area.clone());
        assert!(result.is_ok());
        let verified_area = mlw.get_area_by_id(&original_area.stake_id).unwrap();
        assert_eq!(verified_area.stake_name, "New Name");
        assert_eq!(verified_area.note, Some("New Note".to_string()));
    }

    #[test]
    fn test_mlw_update_area_not_found_error() {
        let mut mlw = MLW::new();
        let non_existent_area = create_test_stake(999, "Non Existent", None, false, false, None);
        let result = mlw.update_area(non_existent_area);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), StakeError::StakeNotFound);
    }

    // --- NEW: Granular Tests for MLW Mark Area Complete ---

    #[test]
    fn test_mlw_mark_area_complete_success() {
        let mut mlw = MLW::new();
        let area = mlw.new_area("Test Area".to_string(), None);
        let result = mlw.mark_area_complete(&area.stake_id);
        assert!(result.is_ok());
        let updated_area = mlw.get_area_by_id(&area.stake_id).unwrap();
        assert!(updated_area.complete);
        assert!(!updated_area.is_active());
        // Verify it's in the completed list
        assert!(mlw.completed_areas().contains(&updated_area));
        // Verify it's not in the active list
        assert!(!mlw.active_areas().contains(&updated_area));
    }

    #[test]
    fn test_mlw_mark_area_complete_not_found_error() {
        let mut mlw = MLW::new();
        let result = mlw.mark_area_complete(&StakeId(999));
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), StakeError::StakeNotFound);
    }

    // --- NEW: Granular Tests for MLW Mark Area Dropped ---

    #[test]
    fn test_mlw_mark_area_dropped_success() {
        let mut mlw = MLW::new();
        let area = mlw.new_area("Test Area".to_string(), None);
        let result = mlw.mark_area_dropped(&area.stake_id);
        assert!(result.is_ok());
        let updated_area = mlw.get_area_by_id(&area.stake_id).unwrap();
        assert!(updated_area.dropped);
        assert!(!updated_area.is_active());
        // Verify it's not in the active list
        assert!(!mlw.active_areas().contains(&updated_area));
    }

    #[test]
    fn test_mlw_mark_area_dropped_not_found_error() {
        let mut mlw = MLW::new();
        let result = mlw.mark_area_dropped(&StakeId(999));
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), StakeError::StakeNotFound);
    }

    #[cfg(test)] // Ensures this module is only compiled for tests
    pub mod project_tests {
        // Use `pub` so it's accessible within `tests` module
        use super::*; // Imports items from the parent `tests` module, including create_test_stake
        // You might need to import specific types like Stake, StakeId, StakeError
        // if `super::*` is not enough, or if you want explicit imports.
        // For example:
        // use crate::entities::stake::{Stake, StakeId, StakeError};

        // --- Granular Tests for MLW Project Management Methods ---

        #[test]
        fn test_mlw_new_project_id_correctly_generated() {
            let mut mlw = MLW::new();
            let project = mlw.new_project("Test Project".to_string(), None, None);
            assert_eq!(project.stake_id, StakeId(1));
        }

        #[test]
        fn test_mlw_new_project_name_correctly_set() {
            let mut mlw = MLW::new();
            let project = mlw.new_project("Test Project Name".to_string(), None, None);
            assert_eq!(project.stake_name, "Test Project Name");
        }

        #[test]
        fn test_mlw_new_project_note_correctly_set() {
            let mut mlw = MLW::new();
            let project =
                mlw.new_project("Test Project".to_string(), None, Some("A note".to_string()));
            assert_eq!(project.note, Some("A note".to_string()));
        }

        #[test]
        fn test_mlw_new_project_starts_active() {
            let mut mlw = MLW::new();
            let project = mlw.new_project("Test Project".to_string(), None, None);
            assert!(project.is_active());
        }

        #[test]
        fn test_mlw_new_project_adds_to_collection() {
            let mut mlw = MLW::new();
            mlw.new_project("Test Project".to_string(), None, None);
            assert_eq!(mlw.projects.len(), 1);
        }

        #[test]
        fn test_mlw_new_project_increments_next_id() {
            let mut mlw = MLW::new();
            mlw.new_project("Test Project".to_string(), None, None);
            assert_eq!(mlw.next_project_id(), StakeId(2));
        }

        #[test]
        fn test_mlw_new_project_with_parent_id() {
            let mut mlw = MLW::new();
            let area_id = mlw.areas.generate_id(); // Need an ID for the parent
            let project = mlw.new_project("Test Project".to_string(), Some(area_id.clone()), None);
            assert_eq!(project.parent_id, Some(area_id));
        }

        #[test]
        fn test_mlw_new_project_without_parent_id() {
            let mut mlw = MLW::new();
            let project = mlw.new_project("Test Project".to_string(), None, None);
            assert_eq!(project.parent_id, None);
        }

        // --- Granular Tests for MLW Active/Completed Projects ---

        #[test]
        fn test_mlw_active_projects_returns_only_active() {
            let mut mlw = MLW::new();
            let active1 = mlw.new_project("Active 1".to_string(), None, None);
            let mut completed1 = mlw.new_project("Completed 1".to_string(), None, None);
            let _ = mlw.mark_project_complete(&completed1.stake_id);
            let active2 = mlw.new_project("Active 2".to_string(), None, None);

            let active_projects = mlw.active_projects();
            assert_eq!(active_projects.len(), 2);
            assert!(active_projects.contains(&&active1));
            assert!(active_projects.contains(&&active2));
            assert!(
                !active_projects
                    .iter()
                    .any(|&s| s.stake_id == completed1.stake_id)
            );
        }

        #[test]
        fn test_mlw_completed_projects_returns_only_completed() {
            let mut mlw = MLW::new();
            let active1 = mlw.new_project("Active 1".to_string(), None, None);
            let mut completed1 = mlw.new_project("Completed 1".to_string(), None, None);
            let _ = mlw.mark_project_complete(&completed1.stake_id);
            let active2 = mlw.new_project("Active 2".to_string(), None, None);

            let completed_projects = mlw.completed_projects();
            assert_eq!(completed_projects.len(), 1);
            let updated_completed1 = mlw.get_project_by_id(&completed1.stake_id).unwrap();
            assert!(completed_projects.contains(&updated_completed1));
            assert!(
                !completed_projects
                    .iter()
                    .any(|&s| s.stake_id == active1.stake_id)
            );
            assert!(
                !completed_projects
                    .iter()
                    .any(|&s| s.stake_id == active2.stake_id)
            );
        }

        // --- Granular Tests for MLW Get Project By ID ---

        #[test]
        fn test_mlw_get_project_by_id_found() {
            let mut mlw = MLW::new();
            let project = mlw.new_project("Test Project".to_string(), None, None);
            let retrieved_project = mlw.get_project_by_id(&project.stake_id);
            assert!(retrieved_project.is_some());
            assert_eq!(retrieved_project.unwrap(), &project);
        }

        #[test]
        fn test_mlw_get_project_by_id_not_found() {
            let mlw = MLW::new();
            let not_found = mlw.get_project_by_id(&StakeId(999));
            assert!(not_found.is_none());
        }

        // --- Granular Tests for MLW Update Project ---

        #[test]
        fn test_mlw_update_project_success() {
            let mut mlw = MLW::new();
            let original_project = mlw.new_project("Original Name".to_string(), None, None);
            let mut updated_project = original_project.clone();
            updated_project.stake_name = "New Name".to_string();
            updated_project.note = Some("New Note".to_string());

            let result = mlw.update_project(updated_project.clone());
            assert!(result.is_ok());
            let verified_project = mlw.get_project_by_id(&original_project.stake_id).unwrap();
            assert_eq!(verified_project.stake_name, "New Name");
            assert_eq!(verified_project.note, Some("New Note".to_string()));
        }

        #[test]
        fn test_mlw_update_project_not_found_error() {
            let mut mlw = MLW::new();
            let non_existent_project =
                create_test_stake(999, "Non Existent", None, false, false, None);
            let result = mlw.update_project(non_existent_project);
            assert!(result.is_err());
            assert_eq!(result.unwrap_err(), StakeError::StakeNotFound);
        }

        // --- Granular Tests for MLW Mark Project Complete ---

        #[test]
        fn test_mlw_mark_project_complete_success() {
            let mut mlw = MLW::new();
            let project = mlw.new_project("Test Project".to_string(), None, None);
            let result = mlw.mark_project_complete(&project.stake_id);
            assert!(result.is_ok());
            let updated_project = mlw.get_project_by_id(&project.stake_id).unwrap();
            assert!(updated_project.complete);
            assert!(!updated_project.is_active());
            assert!(mlw.completed_projects().contains(&updated_project));
            assert!(!mlw.active_projects().contains(&updated_project));
        }

        #[test]
        fn test_mlw_mark_project_complete_not_found_error() {
            let mut mlw = MLW::new();
            let result = mlw.mark_project_complete(&StakeId(999));
            assert!(result.is_err());
            assert_eq!(result.unwrap_err(), StakeError::StakeNotFound);
        }

        // --- Granular Tests for MLW Mark Project Dropped ---

        #[test]
        fn test_mlw_mark_project_dropped_success() {
            let mut mlw = MLW::new();
            let project = mlw.new_project("Test Project".to_string(), None, None);
            let result = mlw.mark_project_dropped(&project.stake_id);
            assert!(result.is_ok());
            let updated_project = mlw.get_project_by_id(&project.stake_id).unwrap();
            assert!(updated_project.dropped);
            assert!(!updated_project.is_active());
            assert!(!mlw.active_projects().contains(&updated_project));
        }

        #[test]
        fn test_mlw_mark_project_dropped_not_found_error() {
            let mut mlw = MLW::new();
            let result = mlw.mark_project_dropped(&StakeId(999));
            assert!(result.is_err());
            assert_eq!(result.unwrap_err(), StakeError::StakeNotFound);
        }

        // --- Granular Tests for MLW Get Project Children ---

        #[test]
        fn test_mlw_get_project_children_found() {
            let mut mlw = MLW::new();
            let parent_project = mlw.new_project("Parent".to_string(), None, None);
            let child1 = mlw.new_project(
                "Child 1".to_string(),
                Some(parent_project.stake_id.clone()),
                None,
            );
            let child2 = mlw.new_project(
                "Child 2".to_string(),
                Some(parent_project.stake_id.clone()),
                None,
            );
            let unrelated_project = mlw.new_project("Unrelated".to_string(), None, None);

            let children = mlw.get_project_children(&parent_project.stake_id);
            assert_eq!(children.len(), 2);
            assert!(children.contains(&&child1)); // Use && since get_project_children returns Vec<&Stake>
            assert!(children.contains(&&child2));
            assert!(!children.contains(&&unrelated_project));
        }

        #[test]
        fn test_mlw_get_project_children_empty_if_no_children() {
            let mut mlw = MLW::new();
            let parent_project = mlw.new_project("Parent".to_string(), None, None);
            let children = mlw.get_project_children(&parent_project.stake_id);
            assert!(children.is_empty());
        }

        #[test]
        fn test_mlw_get_project_children_non_existent_parent() {
            let mlw = MLW::new();
            let non_existent_parent_id = StakeId(999);
            let children = mlw.get_project_children(&non_existent_parent_id);
            assert!(children.is_empty());
        }
    } // End of project_tests module

    #[cfg(test)] // Ensures this module is only compiled for tests
    pub mod task_tests {
        // Use `pub` so it's accessible within `tests` module
        use super::*; // Imports items from the parent `tests` module, including create_test_stake

        // --- Granular Tests for MLW Task Management Methods ---

        #[test]
        fn test_mlw_new_task_id_correctly_generated() {
            let mut mlw = MLW::new();
            let task = mlw.new_task("Test Task".to_string(), None, None);
            assert_eq!(task.stake_id, StakeId(1));
        }

        #[test]
        fn test_mlw_new_task_name_correctly_set() {
            let mut mlw = MLW::new();
            let task = mlw.new_task("Test Task Name".to_string(), None, None);
            assert_eq!(task.stake_name, "Test Task Name");
        }

        #[test]
        fn test_mlw_new_task_note_correctly_set() {
            let mut mlw = MLW::new();
            let task = mlw.new_task("Test Task".to_string(), None, Some("A note".to_string()));
            assert_eq!(task.note, Some("A note".to_string()));
        }

        #[test]
        fn test_mlw_new_task_starts_active() {
            let mut mlw = MLW::new();
            let task = mlw.new_task("Test Task".to_string(), None, None);
            assert!(task.is_active());
        }

        #[test]
        fn test_mlw_new_task_adds_to_collection() {
            let mut mlw = MLW::new();
            mlw.new_task("Test Task".to_string(), None, None);
            assert_eq!(mlw.tasks.len(), 1);
        }

        #[test]
        fn test_mlw_new_task_increments_next_id() {
            let mut mlw = MLW::new();
            mlw.new_task("Test Task".to_string(), None, None);
            assert_eq!(mlw.next_task_id(), StakeId(2));
        }

        #[test]
        fn test_mlw_new_task_with_parent_id() {
            let mut mlw = MLW::new();
            let project_parent = mlw.new_project("Parent Project".to_string(), None, None); // Need a parent project
            let task = mlw.new_task(
                "Test Task".to_string(),
                Some(project_parent.stake_id.clone()),
                None,
            );
            assert_eq!(task.parent_id, Some(project_parent.stake_id));
        }

        #[test]
        fn test_mlw_new_task_without_parent_id() {
            let mut mlw = MLW::new();
            let task = mlw.new_task("Test Task".to_string(), None, None);
            assert_eq!(task.parent_id, None);
        }

        // --- Granular Tests for MLW Active/Completed Tasks ---

        #[test]
        fn test_mlw_active_tasks_returns_only_active() {
            let mut mlw = MLW::new();
            let active1 = mlw.new_task("Active 1".to_string(), None, None);
            let mut completed1 = mlw.new_task("Completed 1".to_string(), None, None);
            let _ = mlw.mark_task_complete(&completed1.stake_id);
            let active2 = mlw.new_task("Active 2".to_string(), None, None);

            let active_tasks = mlw.active_tasks();
            assert_eq!(active_tasks.len(), 2);
            assert!(active_tasks.contains(&&active1));
            assert!(active_tasks.contains(&&active2));
            assert!(
                !active_tasks
                    .iter()
                    .any(|&s| s.stake_id == completed1.stake_id)
            );
        }

        #[test]
        fn test_mlw_completed_tasks_returns_only_completed() {
            let mut mlw = MLW::new();
            let active1 = mlw.new_task("Active 1".to_string(), None, None);
            let mut completed1 = mlw.new_task("Completed 1".to_string(), None, None);
            let _ = mlw.mark_task_complete(&completed1.stake_id);
            let active2 = mlw.new_task("Active 2".to_string(), None, None);

            let completed_tasks = mlw.completed_tasks();
            assert_eq!(completed_tasks.len(), 1);
            let updated_completed1 = mlw.get_task_by_id(&completed1.stake_id).unwrap();
            assert!(completed_tasks.contains(&updated_completed1));
            assert!(
                !completed_tasks
                    .iter()
                    .any(|&s| s.stake_id == active1.stake_id)
            );
            assert!(
                !completed_tasks
                    .iter()
                    .any(|&s| s.stake_id == active2.stake_id)
            );
        }

        // --- Granular Tests for MLW Get Task By ID ---

        #[test]
        fn test_mlw_get_task_by_id_found() {
            let mut mlw = MLW::new();
            let task = mlw.new_task("Test Task".to_string(), None, None);
            let retrieved_task = mlw.get_task_by_id(&task.stake_id);
            assert!(retrieved_task.is_some());
            assert_eq!(retrieved_task.unwrap(), &task);
        }

        #[test]
        fn test_mlw_get_task_by_id_not_found() {
            let mlw = MLW::new();
            let not_found = mlw.get_task_by_id(&StakeId(999));
            assert!(not_found.is_none());
        }

        // --- Granular Tests for MLW Update Task ---

        #[test]
        fn test_mlw_update_task_success() {
            let mut mlw = MLW::new();
            let original_task = mlw.new_task("Original Name".to_string(), None, None);
            let mut updated_task = original_task.clone();
            updated_task.stake_name = "New Name".to_string();
            updated_task.note = Some("New Note".to_string());

            let result = mlw.update_task(updated_task.clone());
            assert!(result.is_ok());
            let verified_task = mlw.get_task_by_id(&original_task.stake_id).unwrap();
            assert_eq!(verified_task.stake_name, "New Name");
            assert_eq!(verified_task.note, Some("New Note".to_string()));
        }

        #[test]
        fn test_mlw_update_task_not_found_error() {
            let mut mlw = MLW::new();
            let non_existent_task =
                create_test_stake(999, "Non Existent", None, false, false, None);
            let result = mlw.update_task(non_existent_task);
            assert!(result.is_err());
            assert_eq!(result.unwrap_err(), StakeError::StakeNotFound);
        }

        // --- Granular Tests for MLW Mark Task Complete ---

        #[test]
        fn test_mlw_mark_task_complete_success() {
            let mut mlw = MLW::new();
            let task = mlw.new_task("Test Task".to_string(), None, None);
            let result = mlw.mark_task_complete(&task.stake_id);
            assert!(result.is_ok());
            let updated_task = mlw.get_task_by_id(&task.stake_id).unwrap();
            assert!(updated_task.complete);
            assert!(!updated_task.is_active());
            assert!(mlw.completed_tasks().contains(&updated_task));
            assert!(!mlw.active_tasks().contains(&updated_task));
        }

        #[test]
        fn test_mlw_mark_task_complete_not_found_error() {
            let mut mlw = MLW::new();
            let result = mlw.mark_task_complete(&StakeId(999));
            assert!(result.is_err());
            assert_eq!(result.unwrap_err(), StakeError::StakeNotFound);
        }

        // --- Granular Tests for MLW Mark Task Dropped ---

        #[test]
        fn test_mlw_mark_task_dropped_success() {
            let mut mlw = MLW::new();
            let task = mlw.new_task("Test Task".to_string(), None, None);
            let result = mlw.mark_task_dropped(&task.stake_id);
            assert!(result.is_ok());
            let updated_task = mlw.get_task_by_id(&task.stake_id).unwrap();
            assert!(updated_task.dropped);
            assert!(!updated_task.is_active());
            assert!(!mlw.active_tasks().contains(&updated_task));
        }

        #[test]
        fn test_mlw_mark_task_dropped_not_found_error() {
            let mut mlw = MLW::new();
            let result = mlw.mark_task_dropped(&StakeId(999));
            assert!(result.is_err());
            assert_eq!(result.unwrap_err(), StakeError::StakeNotFound);
        }

        // --- Granular Tests for MLW Get Task Children ---

        #[test]
        fn test_mlw_get_task_children_found() {
            let mut mlw = MLW::new();
            let project_parent =
                mlw.new_project("Parent Project for Tasks".to_string(), None, None); // Parent for tasks
            let child1 = mlw.new_task(
                "Child 1".to_string(),
                Some(project_parent.stake_id.clone()),
                None,
            );
            let child2 = mlw.new_task(
                "Child 2".to_string(),
                Some(project_parent.stake_id.clone()),
                None,
            );
            let unrelated_task = mlw.new_task("Unrelated".to_string(), None, None);

            let children = mlw.get_task_children(&project_parent.stake_id);
            assert_eq!(children.len(), 2);
            assert!(children.contains(&&child1));
            assert!(children.contains(&&child2));
            assert!(!children.contains(&&unrelated_task));
        }

        #[test]
        fn test_mlw_get_task_children_empty_if_no_children() {
            let mut mlw = MLW::new();
            let parent_task = mlw.new_task("Parent Task".to_string(), None, None);
            let children = mlw.get_task_children(&parent_task.stake_id);
            assert!(children.is_empty());
        }

        #[test]
        fn test_mlw_get_task_children_non_existent_parent() {
            let mlw = MLW::new();
            let non_existent_parent_id = StakeId(999);
            let children = mlw.get_task_children(&non_existent_parent_id);
            assert!(children.is_empty());
        }
    } // E
}
