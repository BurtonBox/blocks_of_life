//! Reusable behavior patterns and helper traits.
//!
//! This module contains trait implementations that provide common behaviors.
//! Files are organized by pattern category and may contain multiple related traits:
//!
//! - `structural_patterns` → `AnatomyAnalyzer`
//! - `locomotion_patterns` → `BipedalMobility`, `QuadrupedalMobility`, `SessileMovement`
//! - `presentation_patterns` → `BasicSummary`, `DetailedSummary`
//!
//! See the module README.md for detailed guidance.

pub mod structural_patterns;
pub mod locomotion_patterns;
pub mod presentation_patterns;

// Re-export commonly used patterns
pub use structural_patterns::AnatomyAnalyzer;
pub use locomotion_patterns::{BipedalMobility, QuadrupedalMobility, SessileMovement};
pub use presentation_patterns::{BasicSummary, DetailedSummary};
