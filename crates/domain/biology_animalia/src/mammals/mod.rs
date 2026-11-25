mod mammal;
mod homo_sapiens;

// Re-export generic and specific types
pub use mammal::Mammal;
pub use homo_sapiens::{Designation, Human, HumanBuilder, Moniker, NameParts, NomenclatureComponents};
