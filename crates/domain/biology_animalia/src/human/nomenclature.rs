use crate::{Human, Moniker};

use biology_shared::Nomenclature;

impl Nomenclature for Human {
    fn display_name(&self) -> String {
        self.name.display_name()
    }
}

