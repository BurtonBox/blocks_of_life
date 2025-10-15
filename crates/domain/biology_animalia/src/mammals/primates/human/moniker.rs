use crate::NameParts;
use biology::Nomenclature;

#[derive(Debug, Clone)]
pub enum Moniker {
    Name(NameParts),
    Designation(String),
}

impl Nomenclature for Moniker {
    fn display_name(&self) -> String {
        match self {
            Moniker::Name(name) => {
                let mut parts = vec![];
                if let Some(prefix) = &name.prefix {
                    parts.push(prefix.clone());
                }
                if let Some(first) = &name.first {
                    parts.push(first.clone());
                }
                if let Some(middle) = &name.middle {
                    parts.push(middle.clone());
                }
                if let Some(last) = &name.last {
                    parts.push(last.clone());
                }
                if let Some(suffix) = &name.suffix {
                    parts.push(suffix.clone());
                }
                parts.join(" ")
            }
            Moniker::Designation(title) => title.clone(),
        }
    }
}
