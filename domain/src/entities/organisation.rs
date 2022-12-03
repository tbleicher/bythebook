use chrono::prelude::*;

#[derive(Clone, Debug)]
pub struct Organisation {
    pub id: String,
    pub created_at: DateTime<Utc>,
    pub active: bool,
    pub admin_id: String,
    pub name: String,
    pub deleted: bool,
}

#[derive(Clone, Debug)]
pub struct NewOrganisationDTO {
    pub name: String,
    pub admin_email: String,
    pub admin_name: String,
}
