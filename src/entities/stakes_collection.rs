use chrono::Duration; // Still needed for tests
use chrono::TimeZone;
use chrono::{DateTime, Utc};
use serde::de::{self, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt; // <--- ADD THIS LINE!

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

    pub fn is_empty(&self) -> bool {
        self.stakes.is_empty()
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

    pub fn generate_id(&mut self) -> StakeId {
        let current_id = self.next_id.clone();
        self.next_id.0 += 1;
        current_id
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
}
