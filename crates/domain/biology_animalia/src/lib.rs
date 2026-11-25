// Species definitions (taxonomy as data).
// Public because it is a namespace of constants: `catalog::HOMO_SAPIENS`.
pub mod catalog;

// Generic animal types
mod animal;

// Taxonomic hierarchy
mod mammals;
mod birds;
mod reptiles;
mod fishes;

// Public API: modules are private, types are re-exported from the crate root.
pub use animal::Animal;
pub use mammals::{
    Designation, Human, HumanBuilder, Mammal, Moniker, NameParts, NomenclatureComponents,
};
