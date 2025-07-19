mod entities;

use entities::{AreaId, Project, ProjectId};

fn main() {
    let project_id = ProjectId(1);
    let area_id = AreaId(22);
    let mut project = Project::new(project_id, "Clean the Garage".to_string(), area_id);

    println!("Initial Project: {:?}", project);

    project.mark_complete();
    println!("project after completion: {:?}", project);
}
