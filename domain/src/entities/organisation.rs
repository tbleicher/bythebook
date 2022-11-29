#[derive(Clone, Debug)]
pub struct Organisation {
    pub id: String,
    pub admin_id: String,
    pub name: String,
}

#[derive(Clone, Debug)]
pub struct NewOrganisationDTO {
    pub name: String,
    pub admin_email: String,
    pub admin_name: String,
}
