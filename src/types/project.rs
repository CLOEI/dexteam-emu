use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ProjectEmployee {
    pub outlet_id: u64,
    pub id: u64, 
}

#[derive(Debug, Deserialize)]
pub struct ProjectStaff {
    pub id: u64,
    pub employee: ProjectEmployee
}

#[derive(Debug, Deserialize)]
pub struct ProjectSelf {
    pub id: u64,
    pub project_id: u64,
    pub staff: ProjectStaff
}

#[derive(Debug, Deserialize)]
pub struct Project {
    pub id: u64,
    pub project_name: String,
    #[serde(rename = "self")]
    pub self_data: ProjectSelf,
}

#[derive(Debug, Deserialize)]
pub struct ProjectData {
    pub projects: Vec<Project>
}

#[derive(Debug, Deserialize)]
pub struct ProjectsResponse {
    pub data: ProjectData,
}