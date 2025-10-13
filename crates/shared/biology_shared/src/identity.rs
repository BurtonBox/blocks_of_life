use uuid::Uuid;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};

pub struct Identifier(Uuid);

impl Identifier {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Debug for Identifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Id: {}", self.0)
    }
}

impl Display for Identifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}