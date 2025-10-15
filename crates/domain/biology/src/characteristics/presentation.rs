//! Presentation characteristics of organisms.
//!
//! This module defines traits related to how organisms present and display themselves.

/// Provides a summary representation of an organism.
///
/// This trait allows organisms to generate a comprehensive textual summary
/// of their characteristics, often combining multiple aspects like anatomy,
/// mobility, and vital signs.
///
/// # Examples
///
/// ```rust,ignore
/// use biology::characteristics::presentation::Summarizable;
///
/// impl Summarizable for Human {
///     fn summary(&self) -> String {
///         format!("Human: {}\nAnatomy: {}\nMobility: {}",
///             self.name, self.describe_anatomy(), self.describe_locomotion())
///     }
/// }
/// ```
pub trait Summarizable {
    fn summary(&self) -> String;
}
