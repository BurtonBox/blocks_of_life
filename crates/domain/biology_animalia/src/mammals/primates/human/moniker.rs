use crate::NameParts;
use biology::Nomenclature;
use biology::nomenclature::nomenclature::NomenclatureComponents;

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

impl NomenclatureComponents for Moniker {

    fn prefix_name(&self) -> Option<String> {
        match self {
            Moniker::Name(name) => name.prefix_name(),
            Moniker::Designation(_) => None,
        }
    }

    fn first_name(&self) -> Option<String> {
        match self {
            Moniker::Name(name) => name.first_name(),
            Moniker::Designation(_) => None,
        }
    }

    fn middle_name(&self) -> Option<String> {
        match self {
            Moniker::Name(name) => name.middle_name(),
            Moniker::Designation(_) => None,
        }
    }

    fn last_name(&self) -> Option<String> {
        match self {
            Moniker::Name(name) => name.last_name(),
            Moniker::Designation(_) => None,
        }
    }

    fn suffix_name(&self) -> Option<String> {
        match self {
            Moniker::Name(name) => name.suffix_name(),
            Moniker::Designation(_) => None,
        }
    }
}
