use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// --- Value Objects / Newtypes for IDs ---
// Using newtype pattern for strong typing of IDs.
// This prevents mixing up a ProjectId with an AreaId, for example.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ProjectId(pub u32);

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AreaId(pub u32);

// --- Custom Error Enum for Project operations ---
#[derive(Debug, PartialEq, Eq)]
pub enum ProjectError {
    CannotActivateDroppedProject,
    // Add other specific errors here later if needed
}

// --- Project Struct (Entity) ---
// Derive common traits for convenience:
// Debug: Allows printing the struct with {:?}
// Clone: Allows creating a deep copy of the struct
// PartialEq, Eq: Allows comparing two Project instances for equality
// Serialize, Deserialize: Allows converting to/from formats like JSON (requires `serde` feature in chrono)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Project {
    pub project_id: ProjectId,
    pub project_name: String,
    pub area_id: AreaId, // Reference to an Area entity
    pub complete: bool,
    pub dropped: bool,
    pub date_modified: DateTime<Utc>, // Store date and time with UTC timezone
    pub date_created: DateTime<Utc>,
}

impl Project {
    /// Creates a new Project instance with initial values.
    /// `date_created` and `date_modified` are set to the current UTC time.
    pub fn new(project_id: ProjectId, project_name: String, area_id: AreaId) -> Self {
        let now = Utc::now();
        Project {
            project_id,
            project_name,
            area_id,
            complete: false, // Projects typically start as incomplete
            dropped: false,  // Projects typically start as not dropped
            date_modified: now,
            date_created: now,
        }
    }

    /// Marks the project as complete and updates `date_modified`.
    pub fn mark_complete(&mut self) {
        self.complete = true;
        self.date_modified = Utc::now();
    }

    /// Marks the project as dropped and updates `date_modified`.
    /// Dropped projects are typically also inactive.
    pub fn mark_dropped(&mut self) {
        self.dropped = true;
        self.date_modified = Utc::now();
    }

    pub fn is_active(&self) -> bool {
        !self.dropped && !self.complete
    }
}

// --- Unit Tests ---
// This module will only be compiled and run when `cargo test` is executed.
#[cfg(test)]
mod tests {
    // Import everything from the parent module (entities.rs)
    // This allows us to use Project, ProjectId, AreaId, ProjectError directly.
    use super::*;
    // Import Duration from chrono for date comparisons
    use chrono::Duration;

    // Helper function to create a basic project for tests
    fn create_test_project() -> Project {
        Project::new(ProjectId(100), "Test Project Name".to_string(), AreaId(1))
    }

    #[test]
    fn test_project_new() {
        let project_id = ProjectId(1);
        let area_id = AreaId(10);
        let project_name = "New Initiative".to_string();
        let now_before_creation = Utc::now(); // Capture time before creating

        let project = Project::new(project_id.clone(), project_name.clone(), area_id.clone());

        // Assert initial state
        assert_eq!(project.project_id, project_id);
        assert_eq!(project.project_name, project_name);
        assert_eq!(project.area_id, area_id);
        assert!(!project.complete);
        assert!(!project.dropped);

        // Check dates are approximately now (within a small window)
        // Using `>=` is safer than `==` due to microsecond differences in `Utc::now()` calls.
        assert!(project.date_created >= now_before_creation);
        assert!(project.date_modified >= now_before_creation);
        // Also ensure they are not too far in the future (e.g., within 1 second)
        assert!(project.date_created <= Utc::now() + Duration::seconds(1));
        assert!(project.date_modified <= Utc::now() + Duration::seconds(1));
        assert_eq!(
            project.date_created, project.date_modified,
            "Initially, created and modified dates should be the same"
        );
    }

    #[test]
    fn test_mark_complete() {
        let mut project = create_test_project();
        let initial_modified_date = project.date_modified;
        let now_before_complete = Utc::now();

        // Simulate a small delay to ensure date_modified changes
        std::thread::sleep(std::time::Duration::from_millis(10));

        project.mark_complete();

        assert!(project.complete);
        assert!(!project.dropped); // Should not affect dropped status
        assert!(!project.is_active()); // Should not affect active status
        assert!(
            project.date_modified > initial_modified_date,
            "date_modified should be updated"
        );
        assert!(
            project.date_modified >= now_before_complete,
            "date_modified should be updated to current time"
        );
    }

    #[test]
    fn test_mark_dropped() {
        let mut project = create_test_project();
        let initial_modified_date = project.date_modified;
        let now_before_dropped = Utc::now();

        // Simulate a small delay
        std::thread::sleep(std::time::Duration::from_millis(10));

        project.mark_dropped();

        assert!(!project.complete); // Should not affect complete status
        assert!(project.dropped);
        assert!(
            !project.is_active(),
            "Dropped project should become inactive"
        );
        assert!(
            project.date_modified > initial_modified_date,
            "date_modified should be updated"
        );
        assert!(
            project.date_modified >= now_before_dropped,
            "date_modified should be updated to current time"
        );
    }
}
