#[derive(Debug, Default, Clone)]
pub struct Moniker {
    pub prefix: Option<String>,
    pub first: Option<String>,
    pub middle: Option<String>,
    pub last: Option<String>,
    pub suffix: Option<String>,
}

impl Moniker {
    pub fn from_full_name(full_name: &str) -> Self {
        let parts: Vec<&str> = full_name.split_whitespace().collect();
        match parts.len() {
            1 => Self { first: Some(parts[0].to_string()), ..Default::default() },
            2 => Self { first: Some(parts[0].to_string()), last: Some(parts[1].to_string()), ..Default::default() },
            3 => Self { first: Some(parts[0].to_string()), middle: Some(parts[1].to_string()), last: Some(parts[2].to_string()), ..Default::default() },
            _ => Self::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum MonikerType {
    Name(Moniker),
    Designation(String),
}

