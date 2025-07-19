use chrono::Utc;
use serde::de::{self, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::collections::HashSet;
use std::fmt; // <--- ADD THIS LINE!
use std::time::Instant;

use super::stake::{Stake, StakeId};

#[derive(Debug, Clone, PartialEq, Eq)] // Removed Serialize, Deserialize for custom impl
pub struct StakesCollection {
    stakes: Vec<Stake>,
    next_id: StakeId,
}

// --- Custom Serialize implementation for StakesCollection ---
impl Serialize for StakesCollection {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(Some(2))?;

        map.serialize_entry("nextId", &self.next_id.0)?;

        map.serialize_entry("stakes", &self.stakes)?;

        map.end()
    }
}

// --- Custom Deserialize implementation for StakesCollection ---
impl<'de> Deserialize<'de> for StakesCollection {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        enum Field {
            NextId,
            Stakes,
        }

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("`nextId` or `stakes`")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: de::Error,
                    {
                        match value {
                            "nextId" => Ok(Field::NextId),
                            "stakes" => Ok(Field::Stakes),
                            _ => Err(de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct StakesCollectionVisitor;

        impl<'de> Visitor<'de> for StakesCollectionVisitor {
            type Value = StakesCollection;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct StakesCollection")
            }

            fn visit_map<V>(self, mut map: V) -> Result<StakesCollection, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut next_id: Option<u32> = None;
                let mut stakes: Option<Vec<Stake>> = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        Field::NextId => {
                            if next_id.is_some() {
                                return Err(de::Error::duplicate_field("nextId"));
                            }
                            next_id = Some(map.next_value()?);
                        }
                        Field::Stakes => {
                            if stakes.is_some() {
                                return Err(de::Error::duplicate_field("stakes"));
                            }
                            stakes = Some(map.next_value()?);
                        }
                    }
                }

                let next_id = next_id.ok_or_else(|| de::Error::missing_field("nextId"))?;
                let stakes = stakes.ok_or_else(|| de::Error::missing_field("stakes"))?;

                Ok(StakesCollection {
                    stakes,
                    next_id: StakeId(next_id),
                })
            }
        }

        const FIELDS: &'static [&'static str] = &["nextId", "stakes"];
        deserializer.deserialize_struct("StakesCollection", FIELDS, StakesCollectionVisitor)
    }
}

impl StakesCollection {
    pub fn new() -> Self {
        StakesCollection {
            stakes: Vec::new(),
            next_id: StakeId(1),
        }
    }

    pub fn add_stake(&mut self, stake: Stake) {
        self.stakes.push(stake);
    }

    pub fn len(&self) -> usize {
        self.stakes.len()
    }

    pub fn search_by_name(&self, query: &str) -> Vec<&Stake> {
        // Prepare the query for case-insensitive partial matching
        let lower_query = query.trim().to_lowercase(); // Trim whitespace and convert to lowercase

        if lower_query.is_empty() {
            // If the query is empty after trimming, return all active stakes (or all stakes, depending on logic)
            // For a 'search' method, an empty query usually means 'return all'
            return self.stakes.iter().collect();
        }

        self.stakes
            .iter()
            .filter(|stake| {
                // Convert stake's name to lowercase and check if it contains the lower_query
                stake.stake_name.to_lowercase().contains(&lower_query)
            })
            .collect() // Collect references to matching stakes
    }

    pub fn is_empty(&self) -> bool {
        self.stakes.is_empty()
    }

    pub fn get_by_id(&self, id: &StakeId) -> Option<&Stake> {
        self.stakes.iter().find(|stake| &stake.stake_id == id)
    }

    pub fn active_stakes(&self) -> Vec<&Stake> {
        self.stakes.iter().filter(|s| s.is_active()).collect()
    }

    pub fn completed_stakes(&self) -> Vec<&Stake> {
        self.stakes.iter().filter(|s| s.complete).collect()
    }

    pub fn generate_id(&mut self) -> StakeId {
        let current_id = self.next_id.clone();
        self.next_id.0 += 1;
        current_id
    }

    pub fn get_children(&self, parent_id: &StakeId) -> Vec<&Stake> {
        self.stakes
            .iter()
            .filter(|stake| {
                // Check if stake.parent_id is Some(id) AND that inner id matches the provided parent_id
                stake.parent_id.as_ref() == Some(parent_id)
            })
            .collect() // Collect into a new Vec
    }
}

// ... (rest of the file remains the same until the tests module)

// --- Unit Tests for StakesCollection ---
#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::stake::{Stake, StakeId};
    use chrono::TimeZone;
    use serde_json; // Needed for Utc.with_ymd_and_hms in the fixed_time setup

    // Helper function (copied here as discussed)
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

    // ... (Keep test_stakes_collection_new, test_stakes_collection_add_stake,
    //       test_stakes_collection_len, test_stakes_collection_is_empty,
    //       test_stakes_collection_active_stakes, test_stakes_collection_completed_stakes,
    //       test_stakes_collection_generate_id as they are. They are good unit tests for logic.)

    // REPLACING test_stakes_collection_serialization
    #[test]
    fn test_stakes_collection_serialization_roundtrip() {
        let mut original_collection = StakesCollection::new();
        original_collection.next_id = StakeId(5); // Set a specific next_id for testing

        // Create stakes with fixed times for consistent comparison
        let fixed_time = Utc.with_ymd_and_hms(2024, 7, 19, 8, 30, 0).unwrap();

        let mut stake1 = create_test_stake(
            1,
            "First Stake",
            None,
            false,
            false,
            Some("Note 1".to_string()),
        );
        stake1.date_created = fixed_time;
        stake1.date_modified = fixed_time;

        let mut stake2 = create_test_stake(2, "Second Stake", Some(StakeId(1)), true, false, None);
        stake2.date_created = fixed_time;
        stake2.date_modified = fixed_time;

        let mut stake3 = create_test_stake(
            3,
            "Third Stake",
            Some(StakeId(2)),
            false,
            true,
            Some("Note 3".to_string()),
        );
        stake3.date_created = fixed_time;
        stake3.date_modified = fixed_time;

        original_collection.add_stake(stake1);
        original_collection.add_stake(stake2);
        original_collection.add_stake(stake3);

        // 1. Serialize the original collection
        let serialized = serde_json::to_string_pretty(&original_collection)
            .expect("Failed to serialize collection");
        println!("Serialized collection for roundtrip test:\n{}", serialized); // Print for debugging/inspection

        // 2. Deserialize the string back into a new collection
        let deserialized_collection: StakesCollection =
            serde_json::from_str(&serialized).expect("Failed to deserialize collection");

        // 3. Compare the original collection with the deserialized one
        assert_eq!(
            original_collection, deserialized_collection,
            "Original and deserialized collections should be identical"
        );

        // You can still keep the test_stakes_collection_deserialization you had,
        // as it specifically tests deserializing from a known good string.
        // The roundtrip test ensures the entire process works end-to-end.
    }

    // Keep test_stakes_collection_deserialization as it is, as it's a good test
    // for deserializing from a specific known JSON string.
    #[test]
    fn test_stakes_collection_deserialization() {
        let fixed_time = Utc.with_ymd_and_hms(2024, 7, 19, 8, 30, 0).unwrap();

        let json_input = format!(
            r#"{{
            "nextId": 5,
            "stakes": [
                {{
                    "stake_id": 1,
                    "stake_name": "Loaded Stake 1",
                    "parent_id": null,
                    "complete": false,
                    "dropped": false,
                    "date_modified": "{}",
                    "date_created": "{}",
                    "note": "A note"
                }}
            ]
        }}"#,
            fixed_time.to_rfc3339(),
            fixed_time.to_rfc3339()
        );

        let deserialized: StakesCollection =
            serde_json::from_str(&json_input).expect("Failed to deserialize collection");

        assert_eq!(deserialized.next_id, StakeId(5));
        assert_eq!(deserialized.len(), 1);
        assert_eq!(deserialized.stakes[0].stake_id, StakeId(1));
        assert_eq!(deserialized.stakes[0].stake_name, "Loaded Stake 1");
        assert_eq!(deserialized.stakes[0].parent_id, None);
        assert_eq!(deserialized.stakes[0].complete, false);
        assert_eq!(deserialized.stakes[0].dropped, false);
        assert_eq!(deserialized.stakes[0].date_modified, fixed_time);
        assert_eq!(deserialized.stakes[0].date_created, fixed_time);
        assert_eq!(deserialized.stakes[0].note, Some("A note".to_string()));
    }

    #[test]
    fn test_stakes_collection_unique_ids() {
        let mut collection = StakesCollection::new();
        let num_stakes = 10_000; // Define the number of stakes to create
        let mut seen_ids = HashSet::new(); // To track unique IDs

        for _i in 0..num_stakes {
            let new_id = collection.generate_id();
            let stake_name = format!("Stake {}", new_id.0);
            let stake = create_test_stake(new_id.0, &stake_name, None, false, false, None);

            // Assert that the generated ID is truly unique
            assert!(
                seen_ids.insert(new_id.0),
                "Generated ID {} was not unique!",
                new_id.0
            );

            // Add the stake to the collection (optional for this test, but good practice)
            collection.add_stake(stake);
        }

        // Final checks
        assert_eq!(
            collection.len(),
            num_stakes as usize,
            "Collection should contain all generated stakes"
        );
        assert_eq!(
            collection.next_id,
            StakeId((num_stakes + 1) as u32),
            "next_id should be correctly incremented"
        );
        assert_eq!(
            seen_ids.len(),
            num_stakes as usize,
            "All generated IDs should be unique"
        );
    }

    #[test]
    fn test_stakes_collection_get_by_id() {
        let mut collection = StakesCollection::new();

        let stake1 = create_test_stake(1, "Stake A", None, false, false, None);
        let stake2 = create_test_stake(2, "Stake B", Some(StakeId(1)), true, false, None);
        let stake3 = create_test_stake(3, "Stake C", None, false, true, None);

        collection.add_stake(stake1.clone());
        collection.add_stake(stake2.clone());
        collection.add_stake(stake3.clone());

        // Test finding an existing stake
        let found_stake_1 = collection.get_by_id(&StakeId(1));
        assert!(found_stake_1.is_some(), "Should find Stake 1");
        assert_eq!(
            found_stake_1.unwrap(),
            &stake1,
            "Found Stake 1 should match original"
        ); // Note: compare reference to owned

        let found_stake_2 = collection.get_by_id(&StakeId(2));
        assert!(found_stake_2.is_some(), "Should find Stake 2");
        assert_eq!(
            found_stake_2.unwrap(),
            &stake2,
            "Found Stake 2 should match original"
        ); // Note: compare reference to owned

        // Test not finding a non-existent stake
        let not_found_stake = collection.get_by_id(&StakeId(999));
        assert!(
            not_found_stake.is_none(),
            "Should not find non-existent stake"
        );

        // Test with empty collection
        let empty_collection = StakesCollection::new();
        let not_found_in_empty = empty_collection.get_by_id(&StakeId(1));
        assert!(
            not_found_in_empty.is_none(),
            "Should not find stake in empty collection"
        );
    }

    #[test]
    fn test_stakes_collection_get_children() {
        let mut collection = StakesCollection::new();

        // Create some parent stakes
        let parent_a = create_test_stake(10, "Parent A", None, false, false, None);
        let parent_b = create_test_stake(20, "Parent B", None, false, false, None);

        // Create children for Parent A
        let child_a1 = create_test_stake(
            11,
            "Child A1",
            Some(parent_a.stake_id.clone()),
            false,
            false,
            None,
        );
        let child_a2 = create_test_stake(
            12,
            "Child A2",
            Some(parent_a.stake_id.clone()),
            false,
            false,
            None,
        );

        // Create a child for Parent B
        let child_b1 = create_test_stake(
            21,
            "Child B1",
            Some(parent_b.stake_id.clone()),
            false,
            false,
            None,
        );

        // Create a top-level stake with no parent
        let top_level_c = create_test_stake(30, "Top Level C", None, false, false, None);

        // Add all stakes to the collection
        collection.add_stake(parent_a.clone());
        collection.add_stake(parent_b.clone());
        collection.add_stake(child_a1.clone());
        collection.add_stake(child_a2.clone());
        collection.add_stake(child_b1.clone());
        collection.add_stake(top_level_c.clone());

        // Test retrieving children for Parent A
        let children_of_a = collection.get_children(&parent_a.stake_id);
        assert_eq!(children_of_a.len(), 2, "Parent A should have 2 children");
        // FIX: Add another & to convert &Stake to &&Stake
        assert!(
            children_of_a.contains(&&child_a1),
            "Children of A should contain A1"
        );
        assert!(
            children_of_a.contains(&&child_a2),
            "Children of A should contain A2"
        );
        assert!(
            !children_of_a.contains(&&child_b1),
            "Children of A should NOT contain B1"
        );

        // Test retrieving children for Parent B
        let children_of_b = collection.get_children(&parent_b.stake_id);
        assert_eq!(children_of_b.len(), 1, "Parent B should have 1 child");
        // FIX: Add another &
        assert!(
            children_of_b.contains(&&child_b1),
            "Children of B should contain B1"
        );

        // Test retrieving children for a stake that has no children
        let children_of_c = collection.get_children(&top_level_c.stake_id);
        assert!(children_of_c.is_empty(), "Stake C should have no children");

        // Test retrieving children for a non-existent parent ID
        let non_existent_parent_id = StakeId(999);
        let children_non_existent = collection.get_children(&non_existent_parent_id);
        assert!(
            children_non_existent.is_empty(),
            "Non-existent parent should have no children"
        );

        // Test with an empty collection
        let empty_collection = StakesCollection::new();
        let not_found_in_empty = empty_collection.get_children(&StakeId(1));
        assert!(
            not_found_in_empty.is_empty(),
            "Empty collection should have no children"
        );
    }

    #[test]
    fn test_performance_get_children() {
        let mut collection = StakesCollection::new();
        let num_stakes = 10_000;
        let target_parent_id = StakeId(55); // The parent ID we'll search for
        let mut expected_children_count = 0;

        // Populate the collection with 10,000 stakes
        // We'll make about 10% of them children of StakeId(55) for testing
        for i in 1..=num_stakes {
            let parent_id_option = if i % 10 == 0 && i > 0 {
                // Every 10th stake (that's not the first itself)
                if i == target_parent_id.0 {
                    // Avoid self-parenting if current ID is target ID
                    None // Or some other parent, to ensure target_parent_id itself can be a parent
                } else {
                    expected_children_count += 1;
                    Some(target_parent_id.clone())
                }
            } else if i == target_parent_id.0 {
                // Ensure target_parent_id itself is added
                None
            } else {
                Some(StakeId(i % 50 + 1)) // Random-ish other parent_ids
            };
            let stake_name = format!("Stake {}", i);
            collection.add_stake(create_test_stake(
                i as u32,
                &stake_name,
                parent_id_option,
                false,
                false,
                None,
            ));
        }

        println!(
            "\nPerformance Test: Finding children of StakeId({}) in {} stakes.",
            target_parent_id.0, num_stakes
        );

        let start_time = Instant::now(); // Start measuring time

        let children = collection.get_children(&target_parent_id);

        let elapsed_time = start_time.elapsed(); // Stop measuring time

        println!("Found {} children in {:?}", children.len(), elapsed_time);

        // Assert that we found the expected number of children
        // The helper `create_test_stake` will generate child_a1/child_a2 for id 11 and 12, etc.
        // It's more reliable to check the exact count you expect from your generation logic.
        assert_eq!(
            children.len(),
            expected_children_count,
            "The number of children found should match expectations."
        );
        assert!(
            !children.is_empty(),
            "Should find at least some children for the target ID."
        );

        // Additional sanity check: ensure the children actually belong to the parent
        for child in children {
            assert_eq!(
                child.parent_id,
                Some(target_parent_id.clone()),
                "Found child has incorrect parent_id"
            );
        }
    }

    #[test]
    fn test_stakes_collection_is_empty() {
        let mut collection = StakesCollection::new();
        // 1. Assert that a new collection is empty
        assert!(collection.is_empty(), "New collection should be empty");

        // 2. Add a stake
        collection.add_stake(create_test_stake(1, "A", None, false, false, None));
        // 3. Assert that it's no longer empty
        assert!(
            !collection.is_empty(),
            "Collection with stakes should not be empty"
        );
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
        // FIX: Use && to compare references
        assert!(active_stakes.contains(&&active_stake1));
        assert!(active_stakes.contains(&&active_stake2));
        assert!(!active_stakes.contains(&&completed_stake));
        assert!(!active_stakes.contains(&&dropped_stake));
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
        // FIX: Use && to compare references
        assert!(completed_stakes.contains(&&completed_stake1));
        assert!(completed_stakes.contains(&&completed_stake2));
        assert!(!completed_stakes.contains(&&active_stake));
        assert!(!completed_stakes.contains(&&dropped_stake));
    }

    #[test]
    fn test_stakes_collection_search_by_name_functional() {
        let mut collection = StakesCollection::new();

        // Setup various stakes for testing search
        let stake1 = create_test_stake(1, "Website Redesign", None, false, false, None);
        let stake2 = create_test_stake(2, "Mobile App Development", None, true, false, None);
        let stake3 = create_test_stake(3, "Marketing Campaign Launch", None, false, true, None);
        let stake4 =
            create_test_stake(4, "Redesign Homepage", Some(StakeId(1)), false, false, None); // Partial match 'Redesign'
        let stake5 = create_test_stake(
            5,
            "Internal Review",
            None,
            false,
            false,
            Some("Review all marketing materials".to_string()),
        );
        let stake6 = create_test_stake(6, "Brand Refresh", None, false, false, None);

        collection.add_stake(stake1.clone());
        collection.add_stake(stake2.clone());
        collection.add_stake(stake3.clone());
        collection.add_stake(stake4.clone());
        collection.add_stake(stake5.clone());
        collection.add_stake(stake6.clone());

        // Test 1: Exact match, case-insensitive
        let results1 = collection.search_by_name("website redesign");
        assert_eq!(
            results1.len(),
            1,
            "Should find 1 exact match (case-insensitive)"
        );
        assert!(results1.contains(&&stake1));

        // Test 2: Partial match, case-insensitive
        let results2 = collection.search_by_name("design");
        assert_eq!(
            results2.len(),
            2,
            "Should find 2 partial matches (Website Redesign, Redesign Homepage)"
        );
        assert!(results2.contains(&&stake1));
        assert!(results2.contains(&&stake4));

        // Test 3: No match
        let results3 = collection.search_by_name("nonexistent");
        assert!(
            results3.is_empty(),
            "Should find no matches for non-existent query"
        );

        // Test 4: Match beginning of word
        let results4 = collection.search_by_name("web");
        assert_eq!(results4.len(), 1, "Should find 'Website Redesign'");
        assert!(results4.contains(&&stake1));

        // Test 5: Match middle of word
        let results5 = collection.search_by_name("app develop");
        assert_eq!(results5.len(), 1, "Should find 'Mobile App Development'");
        assert!(results5.contains(&&stake2));

        // Test 6: Search for an empty string (should return all stakes)
        let results6 = collection.search_by_name("");
        assert_eq!(
            results6.len(),
            collection.len(),
            "Empty query should return all stakes"
        );

        // Test 7: Search with leading/trailing whitespace (should still work after trimming if implemented, or might fail if not trimmed)
        let results7 = collection.search_by_name("  redesign  "); // Assuming no trim in method
        assert_eq!(
            results7.len(),
            2,
            "Should find matches even with whitespace around query"
        );
        assert!(results7.contains(&&stake1));
        assert!(results7.contains(&&stake4));
    }

    // NEW FAILING TEST: test_performance_search_by_name
    #[test]
    fn test_performance_search_by_name() {
        let mut collection = StakesCollection::new();
        let num_stakes = 10_000;
        let search_query = "perf_target"; // A unique string to search for
        let mut expected_matches = 0;

        // Populate the collection
        for i in 1..=num_stakes {
            let stake_name = if i % 100 == 0 {
                // Every 100th stake will contain the query
                expected_matches += 1;
                format!("Stake {} - {} - other text", i, search_query)
            } else if i == 500 {
                // Add one exact match at a specific ID
                expected_matches += 1;
                search_query.to_string()
            } else {
                format!("Stake {}", i)
            };
            collection.add_stake(create_test_stake(
                i as u32,
                &stake_name,
                None,
                false,
                false,
                None,
            ));
        }

        println!(
            "\nPerformance Test: Searching for '{}' in {} stakes.",
            search_query, num_stakes
        );

        let start_time = Instant::now(); // Start measuring time

        let results = collection.search_by_name(search_query);

        let elapsed_time = start_time.elapsed(); // Stop measuring time

        println!("Found {} matches in {:?}", results.len(), elapsed_time);

        assert_eq!(
            results.len(),
            expected_matches,
            "The number of search matches should be as expected."
        );
        assert!(
            !results.is_empty(),
            "Should find at least some matches for the target query."
        );
    }
}
