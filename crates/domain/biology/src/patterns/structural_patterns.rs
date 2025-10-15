//! Structural analysis patterns.
//!
//! This module provides helper traits for analyzing anatomical structures.

use crate::anatomy::appendage::Appendage;
use crate::anatomy::limb_status::LimbStatus;

/// Provides analysis tools for examining anatomical structures.
///
/// This trait offers default implementations for common anatomical analysis tasks,
/// such as counting limbs and describing limb status. Types only need to implement
/// `get_appendages()` to gain access to all the helper methods.
///
/// # Examples
///
/// ```rust,ignore
/// use biology::patterns::structural_patterns::AnatomyAnalyzer;
///
/// impl AnatomyAnalyzer for Human {
///     fn get_appendages(&self) -> &Vec<Appendage> {
///         &self.appendages
///     }
/// }
///
/// // Now you can use the provided methods:
/// let (arms, missing_arms, legs, missing_legs) = human.count_limbs();
/// let description = human.describe_limb_status();
/// ```
pub trait AnatomyAnalyzer {
    fn get_appendages(&self) -> &Vec<Appendage>;

    fn count_limbs(&self) -> (u32, u32, u32, u32) {
        let mut attached_arms = 0;
        let mut missing_arms = 0;
        let mut attached_legs = 0;
        let mut missing_legs = 0;

        for appendage in self.get_appendages() {
            match appendage {
                Appendage::Arm { status, .. } => match status {
                    LimbStatus::Intact => attached_arms += 1,
                    LimbStatus::Amputated => missing_arms += 1,
                    _ => attached_arms += 1,
                },
                Appendage::Leg { status, .. } => match status {
                    LimbStatus::Intact => attached_legs += 1,
                    LimbStatus::Amputated => missing_legs += 1,
                    _ => attached_legs += 1,
                },
            }
        }

        (attached_arms, missing_arms, attached_legs, missing_legs)
    }

    fn describe_limb_status(&self) -> String {
        let (attached_arms, missing_arms, attached_legs, missing_legs) = self.count_limbs();

        let arm_desc = format!("{} attached arms", attached_arms);
        let leg_desc = format!("{} attached legs", attached_legs);

        let mut description = format!("A bipedal form with {} and {}.", arm_desc, leg_desc);

        if missing_arms > 0 || missing_legs > 0 {
            description.push_str(&format!(
                " It is noted that {} arms and {} legs are missing.",
                missing_arms, missing_legs
            ));
        }

        description
    }
}
