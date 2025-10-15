//! Locomotion characteristics of organisms.
//!
//! This module defines traits related to movement and mobility.

/// Describes how an organism moves and gets around.
///
/// This trait allows organisms to describe their locomotion capabilities,
/// such as walking, swimming, flying, or being sessile.
///
/// # Examples
///
/// ```rust,ignore
/// use biology::characteristics::locomotion::Mobility;
///
/// impl Mobility for Human {
///     fn describe_locomotion(&self) -> String {
///         "Walks and runs on two legs".to_string()
///     }
/// }
/// ```
pub trait Mobility {
    fn describe_locomotion(&self) -> String;
}
