use crate::{Human};
use biology::Nomenclature;

impl Nomenclature for Human {
    fn display_name(&self) -> String {
        self.name.display_name()
    }
}
