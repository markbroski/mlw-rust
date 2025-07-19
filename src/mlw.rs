use crate::entities::stake::StakeId;
use crate::entities::stakes_collection::StakesCollection;

#[derive(Debug, Clone, PartialEq, Eq)] // Derive traits for debugging, cloning, and comparison in tests
pub struct MLW {
    pub areas: StakesCollection,
    pub projects: StakesCollection,
    pub tasks: StakesCollection,
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
}

#[cfg(test)]
mod tests {
    use super::*; // Imports MLW itself (once defined)
    use crate::entities::stake::StakeId;
    use crate::entities::stakes_collection::StakesCollection; // Explicitly import for tests // Explicitly import for tests

    #[test]
    fn test_mlw_new() {
        let mlw = MLW::new();

        // Assert that each collection is instantiated and empty
        assert_eq!(mlw.areas.len(), 0, "Areas collection should be empty");
        assert!(mlw.areas.is_empty(), "Areas collection should be empty");

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
}
