use crate::NomenclatureComponents;

#[derive(Debug, Default, Clone)]
pub struct NameParts {
    pub prefix: Option<String>,
    pub first: Option<String>,
    pub middle: Option<String>,
    pub last: Option<String>,
    pub suffix: Option<String>,
}

impl NameParts {
    pub fn new(first: Option<&str>, middle: Option<&str>, last: Option<&str>) -> Self {
        Self {
            first: first.map(str::to_owned),
            middle: middle.map(str::to_owned),
            last: last.map(str::to_owned),
            ..Default::default()
        }
    }
}

impl From<&str> for NameParts {
    fn from(full_name: &str) -> Self {
        let parts: Vec<&str> = full_name.split_whitespace().collect();
        match parts.len() {
            0 => Self::default(),
            1 => Self {
                first: Some(parts[0].to_owned()),
                ..Default::default()
            },
            2 => Self {
                first: Some(parts[0].to_owned()),
                last: Some(parts[1].to_owned()),
                ..Default::default()
            },
            3 => Self {
                first: Some(parts[0].to_owned()),
                middle: Some(parts[1].to_owned()),
                last: Some(parts[2].to_owned()),
                ..Default::default()
            },
            _ => Self::default(),
        }
    }
}

impl NomenclatureComponents for NameParts {
    fn prefix_name(&self) -> Option<&str> {
        self.prefix.as_deref()
    }

    fn first_name(&self) -> Option<&str> {
        self.first.as_deref()
    }

    fn middle_name(&self) -> Option<&str> {
        self.middle.as_deref()
    }

    fn last_name(&self) -> Option<&str> {
        self.last.as_deref()
    }

    fn suffix_name(&self) -> Option<&str> {
        self.suffix.as_deref()
    }
}
