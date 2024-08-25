#[derive(Debug)]
pub enum UserProfile {
    LegalGuardian,
    Admin,
    Doctor,
}

pub struct User {
    pub id: u32,
    pub name: String,
    pub profile: UserProfile,
}
