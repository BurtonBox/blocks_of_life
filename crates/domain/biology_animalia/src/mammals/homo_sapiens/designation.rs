#[derive(Debug, Clone)]
pub struct Designation(pub String);

impl From<&str> for Designation {
    fn from(value: &str) -> Self {
        Self(value.to_owned())
    }
}

impl From<String> for Designation {
    fn from(value: String) -> Self {
        Self(value)
    }
}