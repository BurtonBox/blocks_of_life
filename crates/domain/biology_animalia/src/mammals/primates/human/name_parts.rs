use biology::nomenclature::nomenclature::NomenclatureComponents;

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
            first: first.map(str::to_string),
            middle: middle.map(str::to_string),
            last: last.map(str::to_string),
            ..Default::default()
        }
    }

    pub fn from(full_name: &str) -> Self {
        let parts: Vec<&str> = full_name.split_whitespace().collect();
        match parts.len() {
            0 => Self::default(),
            1 => Self {
                first: Some(parts[0].to_string()),
                ..Default::default()
            },
            2 => Self {
                first: Some(parts[0].to_string()),
                last: Some(parts[1].to_string()),
                ..Default::default()
            },
            3 => Self {
                first: Some(parts[0].to_string()),
                middle: Some(parts[1].to_string()),
                last: Some(parts[2].to_string()),
                ..Default::default()
            },
            _ => Self::default(),
        }
    }
}

impl NomenclatureComponents for NameParts {
    fn prefix_name(&self) -> Option<String> {
        self.prefix.clone()
    }

    fn first_name(&self) -> Option<String> {
        self.first.clone()
    }

    fn middle_name(&self) -> Option<String> {
        self.middle.clone()
    }

    fn last_name(&self) -> Option<String> {
        self.last.clone()
    }

    fn suffix_name(&self) -> Option<String> {
        self.suffix.clone()
    }
}
