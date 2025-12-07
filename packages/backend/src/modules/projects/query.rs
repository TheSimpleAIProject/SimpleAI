use std::path::Path;

use crate::modules::utils::prelude::*;
use walkdir::WalkDir;

pub fn get_all_projects() -> Result<Vec<Project>, String> {
    let projects_dir = Path::new("projects");
    if !projects_dir.exists() {
        return Ok(Vec::default());
    }

    let mut projects = Vec::default();

    for entry in WalkDir::new(projects_dir)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.path().extension().is_some_and(|ext| ext == "json"))
    {
        let path = entry.path();
        let data = std::fs::read_to_string(path)
            .map_err(|e| format!("Failed to read {}: {}", path.display(), e))?;

        let project: Project = serde_json::from_str(&data)
            .map_err(|e| format!("Failed to deserialize {}: {}", path.display(), e))?;

        projects.push(project);
    }

    Ok(projects)
}

/// This function searches through all available Projects
pub fn query_projects(query_filters: Vec<ProjectQueryFilter>) -> Vec<Project> {
    let all_projects = get_all_projects().expect("Error walking directory!");

    all_projects
        .iter()
        .filter(|project| {
            query_filters
                .iter()
                .all(|filter| filter.clone().is_ok(project))
        })
        .cloned()
        .collect()
}
