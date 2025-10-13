use biology::Sex;
use core_shared::Identifier;
use crate::{Human, Moniker, NameParts};

pub struct HumanBuilder {
    pub id: Identifier,
    pub name_prefix: Option<String>,
    pub name_first: Option<String>,
    pub name_middle: Option<String>,
    pub name_last: Option<String>,
    pub name_suffix: Option<String>,
    pub name_moniker: Option<NameParts>,
    pub name_designation: Option<String>,
    pub sex: Sex,
}

impl HumanBuilder {
    pub fn new() -> Self {
        Self {
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

    pub fn designation(mut self, des: String) -> Self {
        self.name_designation = Some(des);
        self
    }

    pub fn name(mut self, moniker: NameParts) -> Self {
        self.name_moniker = Some(moniker);
        self
    }

    pub fn sex(mut self, sex: Sex) -> Self {
        self.sex = sex;
        self
    }

    pub fn build(self) -> Human {
        let name = if let Some(des) = self.name_designation {
            // If a designation was provided, it wins.
            Moniker::Designation(des)
        } else if let Some(mon) = self.name_moniker {
            Moniker::Name(mon)
        }
        else {
            // Otherwise, assemble a structured name from the parts.
            Moniker::Name(NameParts {
                prefix: self.name_prefix,
                first: self.name_first,
                middle: self.name_middle,
                last: self.name_last,
                suffix: self.name_suffix,
            })
        };

        Human {
            id: self.id,
            name,
            sex: self.sex,
        }
    }
}
