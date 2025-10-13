use crate::{HumanBuilder, Moniker};
use biology::Sex;
use core_shared::Identifier;

pub struct Human {
    pub id: Identifier,
    pub name: Moniker,
    pub sex: Sex,
}

impl Human {
    pub fn new() -> Self {
        Self {
            id: Identifier::new(),
            name: Moniker::Designation(String::from("Unknown")),
            sex: Sex::Unknown,
        }
    }

    pub fn builder() -> HumanBuilder {
        HumanBuilder {
            id: Identifier::new(),
            name_prefix: None,
            name_first: None,
            name_middle: None,
            name_last: None,
            name_suffix: None,
            name_moniker: None,
            name_designation: None,
            sex: Sex::Unknown,
        }
    }
}
