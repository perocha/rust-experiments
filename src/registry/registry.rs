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

    pub fn print_users(&self) {
        for user in &self.users {
            println!("User: {} {} ({:?})", user.id, user.name, user.profile);
        }
    }

    pub fn print_patients(&self) {
        for patient in &self.patients {
            println!("Patient: {} {} (Age: {}) from user id {}", patient.id, patient.name, patient.age, patient.guardian_id);
        }
    }
}