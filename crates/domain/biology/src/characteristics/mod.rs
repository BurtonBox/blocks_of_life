//! Core characteristics that define organism capabilities.
//!
//! This module contains trait definitions for fundamental biological characteristics.
//! Files are organized by domain concept, not trait name:
//!
//! - `structural` → `Anatomy` trait
//! - `locomotion` → `Mobility` trait
//! - `presentation` → `Summarizable` trait
//!
//! See the module README.md for detailed guidance.

pub mod structural;
pub mod locomotion;
pub mod presentation;

// Re-export the main traits for convenience
pub use structural::Anatomy;
pub use locomotion::Mobility;
pub use presentation::Summarizable;
