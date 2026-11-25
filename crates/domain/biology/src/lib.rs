mod anatomy;
mod characteristics;
mod classification;
mod ecosystem;
mod patterns;
mod templates;
mod vitals;
mod organism;

pub use anatomy::{Appendage, ArmMeasurements, Body, LimbStatus, LegMeasurements};
pub use characteristics::{Anatomy, Mobility, Nomenclature, Summarizable};
pub use classification::{Sex, Kingdom, Taxonomy};
pub use ecosystem::{Climate, Environment, Interaction, Population, Terrain};
pub use patterns::{
    AnatomyAnalyzer, BasicSummary, BipedalMobility, DetailedSummary, QuadrupedalMobility,
    SessileMovement,
};
pub use organism::{Organism, OrganismCore};
pub use templates::{regional_defaults, AnatomyTemplate};
pub use vitals::{BloodPressure, BPM, Celsius, LiveVitals, PostMortemReport, VitalStatus};
