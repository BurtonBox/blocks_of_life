pub trait Nomenclature {
    fn display_name(&self) -> String;
}

pub trait NomenclatureComponents {
    fn prefix_name(&self) -> Option<String>;
    fn first_name(&self) -> Option<String>;
    fn middle_name(&self) -> Option<String>;
    fn last_name(&self) -> Option<String>;
    fn suffix_name(&self) -> Option<String>;
}