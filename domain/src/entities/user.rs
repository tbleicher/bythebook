#[derive(Clone, Debug)]
pub struct User {
    pub id: String,
    pub email: String,
    pub name: String,
    pub organisation_id: String,
}

#[derive(Clone, Debug)]
pub struct NewUserDTO {
    pub email: String,
    pub name: String,
    pub organisation_id: String,
}
