//! Core characteristics that define organism capabilities.
//!
//! This module contains trait definitions for fundamental biological characteristics.
//! Files are organized by domain concept, not trait name:
//!
//! - `structural` → `Anatomy` trait
//! - `locomotion` → `Mobility` trait
//! - `naming` → `Nomenclature` trait
//! - `presentation` → `Summarizable` trait
//!
//! See the module README.md for detailed guidance.
mod locomotion;
mod naming;
mod presentation;
mod structural;

// Re-export the main traits for convenience
pub use locomotion::Mobility;
pub use naming::Nomenclature;
pub use presentation::Summarizable;
pub use structural::Anatomy;