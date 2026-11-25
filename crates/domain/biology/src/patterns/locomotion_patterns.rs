//! Locomotion behavior patterns.
//!
//! This module provides common movement patterns for different organism types.

/// Provides bipedal (two-legged) movement behavior.
///
/// This trait offers a default implementation for describing bipedal locomotion.
/// Types can simply implement this trait to gain the behavior, or override
/// the method for custom descriptions.
pub trait BipedalMobility {
    fn describe_bipedal_movement(&self) -> String {
        "Walks and runs on two legs.".to_string()
    }
}

/// Provides quadrupedal (four-legged) movement behavior.
///
/// This trait offers a default implementation for describing quadrupedal locomotion.
pub trait QuadrupedalMobility {
    fn describe_quadrupedal_movement(&self) -> String {
        "Walks and runs on four legs.".to_string()
    }
}

/// Provides sessile (stationary) organism behavior.
///
/// This trait offers a default implementation for describing sessile organisms
/// that are fixed in one location.
pub trait SessileMovement {
    fn describe_sessile_movement(&self) -> String {
        "Sessile (fixed in one place).".to_string()
    }
}
