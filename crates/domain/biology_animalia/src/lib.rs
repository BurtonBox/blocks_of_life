// Generic animal types
pub mod animal;

// Taxonomic hierarchy
pub mod mammals;
pub mod birds;
pub mod reptiles;
pub mod fish;

// Re-export generic types at top level (Option C: both paths work)
pub use animal::Animal;
pub use mammals::Mammal;

// Re-export specific types for convenience (Option C: both paths work)
pub use mammals::primates::{Human, HumanBuilder, Moniker, NameParts};
