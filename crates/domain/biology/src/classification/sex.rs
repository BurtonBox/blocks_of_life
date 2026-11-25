use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum Sex {
    Unknown,
    Male,
    Female,
}

impl Display for Sex {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Sex::Unknown => write!(f, "Unknown"),
            Sex::Male => write!(f, "Male"),
            Sex::Female => write!(f, "Female"),
        }
    }
}
