use core::fmt;

pub struct Person {
    pub name: String,
    pub age: i32,
    pub gender: Genders,
}

pub enum Genders {
    Male,
    Female,
    Other,
}

impl fmt::Display for Genders {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Genders::Male => write!(f, "Male"),
            Genders::Female => write!(f, "Female"),
            Genders::Other => write!(f, "Other"),
        }
    }
}