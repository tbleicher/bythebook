#[derive(Clone, Debug)]
pub struct Project {
    pub id: String,
    pub title: String,
    pub description: String,
    pub organisation_id: String,
}

#[derive(Clone, Debug)]
pub struct NewProjectDTO {
    pub title: String,
    pub description: String,
    pub organisation_id: String,
}
