mod anatomy;
// `regional_defaults` is a namespace of constants (`regional_defaults::AVG_...`),
// so the module itself is public. It is still only reachable through the
// re-export in lib.rs, because `templates` is private.
pub mod regional_defaults;

pub use anatomy::AnatomyTemplate;


