use super::user::User;
use super::patient::Patient;

pub struct Registry {
    pub users: Vec<User>,
    pub patients: Vec<Patient>,
}

impl Registry {
    pub fn new() -> Self {
        Registry {
            users: Vec::new(),
            patients: Vec::new(),
        }
    }

    pub fn add_user(&mut self, user: User) {
        self.users.push(user);
    }

    pub fn add_patient(&mut self, patient: Patient) {
        self.patients.push(patient);
    }
}