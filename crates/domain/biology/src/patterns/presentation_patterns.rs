//! Presentation and summary formatting patterns.
//!
//! This module provides helper traits for generating formatted summaries.

use crate::anatomy::appendage::Appendage;
use crate::Sex;

/// Provides basic summary information for an organism.
///
/// This trait defines the minimum information needed to create a simple summary:
/// name and gender. Other summary traits can build on this foundation.
pub trait BasicSummary {
    fn get_name(&self) -> String;
    fn get_gender(&self) -> &Sex;
}

/// Provides detailed summary generation with default implementation.
///
/// This trait extends `BasicSummary` and adds appendage information, providing
/// a default `generate_summary()` method that creates a formatted output.
///
/// # Examples
///
/// ```rust,ignore
/// impl BasicSummary for Human {
///     fn get_name(&self) -> String { self.name.clone() }
///     fn get_gender(&self) -> &Sex { &self.sex }
/// }
///
/// impl DetailedSummary for Human {
///     fn get_appendages(&self) -> &Vec<Appendage> { &self.appendages }
/// }
///
/// // Now you can use the provided generate_summary() method:
/// let summary = human.generate_summary();
/// ```
pub trait DetailedSummary: BasicSummary {
    fn get_appendages(&self) -> &Vec<Appendage>;

    fn generate_summary(&self) -> String {
        let mut summary = format!(" --- Summary for: {} ({:?}) ---\n", self.get_name(), self.get_gender());
        summary.push_str("Appendages:\n");

        for appendage in self.get_appendages() {
            summary.push_str(&appendage.description());
            summary.push('\n');
        }

        summary
    }
}
