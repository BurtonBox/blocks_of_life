use crate::{Human};
use biology::Nomenclature;
use biology::nomenclature::nomenclature::NomenclatureComponents;

impl Nomenclature for Human {
    fn display_name(&self) -> String {
        self.name.display_name()
    }
}

impl NomenclatureComponents for Human {

    fn prefix_name(&self) -> Option<String> {
        self.name.prefix_name()
    }

    fn first_name(&self) -> Option<String> {
        self.name.first_name()
    }

    fn middle_name(&self) -> Option<String> {
        self.name.middle_name()
    }

    fn last_name(&self) -> Option<String> {
        self.name.last_name()
    }

    fn suffix_name(&self) -> Option<String> {
        self.name.suffix_name()
    }
}