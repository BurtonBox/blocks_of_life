pub mod mammal;
pub mod primates;

// Re-export generic and specific types
pub use mammal::Mammal;
pub use primates::{Human, HumanBuilder, Moniker, NameParts};
