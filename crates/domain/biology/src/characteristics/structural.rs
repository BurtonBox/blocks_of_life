//! Structural characteristics of organisms.
//!
//! This module defines traits related to physical structure and anatomy.

/// Describes the physical structure and anatomy of an organism.
///
/// This trait allows organisms to provide a textual description of their
/// anatomical features, including body parts, limbs, and structural characteristics.
///
/// # Examples
///
/// ```rust,ignore
/// use biology::characteristics::structural::Anatomy;
///
/// impl Anatomy for Human {
///     fn describe_anatomy(&self) -> String {
///         format!("A bipedal form with {} arms and {} legs", ...)
///     }
/// }
/// ```
pub trait Anatomy {
    fn describe_anatomy(&self) -> String;
}
