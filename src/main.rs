mod entities;

use entities::{Stake, StakeId};

fn main() {
    let project_id = StakeId(1);
    let area_id = StakeId(22);
    let mut project = Stake::new(
        project_id,
        "Clean the Garage".to_string(),
        Some(area_id),
        None,
    );

    println!("Initial Project: {:?}", project);

    project.mark_complete();
    println!("project after completion: {:?}", project);
}
