use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;
use uuid::Uuid;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Identifier(Uuid);

impl Identifier {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl FromStr for Identifier {
    type Err = uuid::Error;

    fn from_str(uuid: &str) -> Result<Self, Self::Err> {
        Uuid::parse_str(uuid).map(Identifier)
    }
}

impl From<Uuid> for Identifier {
    fn from(uuid: Uuid) -> Self {
        Identifier(uuid)
    }
}

impl From<Identifier> for Uuid {
    fn from(id: Identifier) -> Self {
        id.0
    }
}

impl From<Identifier> for String {
    fn from(id: Identifier) -> Self {
        id.0.to_string()
    }
}

impl AsRef<Uuid> for Identifier {
    fn as_ref(&self) -> &Uuid {
        &self.0
    }
}

impl Debug for Identifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Id({})", self.0)
    }
}

impl Display for Identifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}