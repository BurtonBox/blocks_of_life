use crate::{Human, MonikerType};

use biology::Nomenclature;

impl Nomenclature for Human {
    fn display_name(&self) -> String {
        match &self.name {
            MonikerType::Designation(d) => d.clone(),

            MonikerType::Name(name) => {
                let mut parts = vec![];
                if let Some(p) = &name.prefix { parts.push(p.as_str()); }
                if let Some(f) = &name.first { parts.push(f.as_str()); }
                if let Some(m) = &name.middle { parts.push(m.as_str()); }
                if let Some(l) = &name.last { parts.push(l.as_str()); }
                if let Some(s) = &name.suffix { parts.push(s.as_str()); }
                parts.join(" ")
            }
        }
    }
}
