#[derive(Clone, Debug)]
pub struct Note {
    pub id: String,
    pub project_id: String,
    pub text: String,
    pub title: String,
}

#[derive(Clone, Debug)]
pub struct NewNoteDTO {
    pub title: String,
    pub text: String,
    pub project_id: String,
}
