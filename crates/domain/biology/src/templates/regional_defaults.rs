// Regional anatomical defaults based on anthropometric research
// Data sources: CDC NHANES, WHO anthropometric studies, peer-reviewed research
// Note: These are population averages for educational/modeling purposes
// Measurements in centimeters



// ============================================================================
// NORTH AMERICAN POPULATIONS
// ============================================================================

use core_shared::Centimeters;
use crate::anatomy::arm_measurements::ArmMeasurements;
use crate::anatomy::leg_measurements::LegMeasurements;
use crate::templates::anatomy::AnatomyTemplate;

/// Average adult male - North American/US (CDC NHANES 2015-2018)
/// Height: ~175cm (5'9"), Arm span ≈ height, Bicep: 33-35cm
pub const AVG_NORTH_AMERICAN_MALE: AnatomyTemplate = AnatomyTemplate {
    arm_measurements: ArmMeasurements {
        upper_length: Centimeters(38),      // Upper arm length
        triceps_length: Centimeters(26),    // Triceps region
        span: Centimeters(175),             // Arm span ≈ height for adults
        flexed_bicep: Centimeters(35),      // Flexed bicep circumference
        unflexed_bicep: Centimeters(33),    // Relaxed arm circumference (13 inches)
    },
    leg_measurements: LegMeasurements {
        inseam: Centimeters(81),            // ~46% of height (175 * 0.46)
        quad: Centimeters(58),              // Thigh circumference
    },
};

/// Average adult female - North American/US (CDC NHANES 2015-2018)
/// Height: ~162cm (5'4"), Arm span ≈ height, Bicep: 31-33cm
pub const AVG_NORTH_AMERICAN_FEMALE: AnatomyTemplate = AnatomyTemplate {
    arm_measurements: ArmMeasurements {
        upper_length: Centimeters(34),      // Upper arm length (shorter than male)
        triceps_length: Centimeters(23),    // Triceps region
        span: Centimeters(162),             // Arm span ≈ height
        flexed_bicep: Centimeters(33),      // Flexed bicep circumference
        unflexed_bicep: Centimeters(31),    // Relaxed arm circumference (12.5 inches)
    },
    leg_measurements: LegMeasurements {
        inseam: Centimeters(75),            // ~46% of height (162 * 0.46)
        quad: Centimeters(55),              // Thigh circumference
    },
};

// ============================================================================
// EAST ASIAN POPULATIONS
// ============================================================================

/// Average adult male - East Asian (Chinese/Korean/Japanese populations)
/// Height: ~170cm (5'7"), Notable: slightly shorter limbs relative to torso
pub const AVG_EAST_ASIAN_MALE: AnatomyTemplate = AnatomyTemplate {
    arm_measurements: ArmMeasurements {
        upper_length: Centimeters(36),      // Slightly shorter upper arm
        triceps_length: Centimeters(24),
        span: Centimeters(170),             // Arm span ≈ height
        flexed_bicep: Centimeters(33),
        unflexed_bicep: Centimeters(31),
    },
    leg_measurements: LegMeasurements {
        inseam: Centimeters(78),            // Slightly shorter inseam
        quad: Centimeters(55),
    },
};

/// Average adult female - East Asian (Chinese/Korean/Japanese populations)
/// Height: ~158cm (5'2"), Proportionally similar to males
pub const AVG_EAST_ASIAN_FEMALE: AnatomyTemplate = AnatomyTemplate {
    arm_measurements: ArmMeasurements {
        upper_length: Centimeters(32),
        triceps_length: Centimeters(22),
        span: Centimeters(158),
        flexed_bicep: Centimeters(30),
        unflexed_bicep: Centimeters(28),
    },
    leg_measurements: LegMeasurements {
        inseam: Centimeters(73),
        quad: Centimeters(52),
    },
};

// ============================================================================
// NORTHERN EUROPEAN POPULATIONS
// ============================================================================

/// Average adult male - Northern European (Scandinavian/Dutch/German)
/// Height: ~180cm (5'11"), Taller population with longer limbs
pub const AVG_NORTHERN_EUROPEAN_MALE: AnatomyTemplate = AnatomyTemplate {
    arm_measurements: ArmMeasurements {
        upper_length: Centimeters(40),      // Longer upper arm
        triceps_length: Centimeters(27),
        span: Centimeters(182),             // Arm span slightly > height
        flexed_bicep: Centimeters(36),
        unflexed_bicep: Centimeters(34),
    },
    leg_measurements: LegMeasurements {
        inseam: Centimeters(84),            // Longer legs
        quad: Centimeters(60),
    },
};

/// Average adult female - Northern European
/// Height: ~167cm (5'6"), Taller than global average
pub const AVG_NORTHERN_EUROPEAN_FEMALE: AnatomyTemplate = AnatomyTemplate {
    arm_measurements: ArmMeasurements {
        upper_length: Centimeters(36),
        triceps_length: Centimeters(24),
        span: Centimeters(168),
        flexed_bicep: Centimeters(33),
        unflexed_bicep: Centimeters(31),
    },
    leg_measurements: LegMeasurements {
        inseam: Centimeters(77),
        quad: Centimeters(56),
    },
};

// ============================================================================
// SOUTH ASIAN POPULATIONS
// ============================================================================

/// Average adult male - South Asian (Indian subcontinent)
/// Height: ~165cm (5'5"), Shorter stature, proportional build
pub const AVG_SOUTH_ASIAN_MALE: AnatomyTemplate = AnatomyTemplate {
    arm_measurements: ArmMeasurements {
        upper_length: Centimeters(35),
        triceps_length: Centimeters(24),
        span: Centimeters(166),             // Arm span slightly > height (research shows)
        flexed_bicep: Centimeters(31),
        unflexed_bicep: Centimeters(29),
    },
    leg_measurements: LegMeasurements {
        inseam: Centimeters(76),
        quad: Centimeters(54),
    },
};

/// Average adult female - South Asian
/// Height: ~152cm (5'0"), Shorter than most global populations
pub const AVG_SOUTH_ASIAN_FEMALE: AnatomyTemplate = AnatomyTemplate {
    arm_measurements: ArmMeasurements {
        upper_length: Centimeters(31),
        triceps_length: Centimeters(21),
        span: Centimeters(153),
        flexed_bicep: Centimeters(29),
        unflexed_bicep: Centimeters(27),
    },
    leg_measurements: LegMeasurements {
        inseam: Centimeters(70),
        quad: Centimeters(50),
    },
};

// ============================================================================
// AFRICAN POPULATIONS
// ============================================================================

/// Average adult male - Sub-Saharan African
/// Height: ~170cm (5'7"), Notable: longer limbs relative to torso
pub const AVG_AFRICAN_MALE: AnatomyTemplate = AnatomyTemplate {
    arm_measurements: ArmMeasurements {
        upper_length: Centimeters(39),      // Longer limbs
        triceps_length: Centimeters(26),
        span: Centimeters(175),             // Arm span > height is common
        flexed_bicep: Centimeters(33),
        unflexed_bicep: Centimeters(31),
    },
    leg_measurements: LegMeasurements {
        inseam: Centimeters(80),            // Longer legs relative to height
        quad: Centimeters(56),
    },
};

/// Average adult female - Sub-Saharan African
/// Height: ~160cm (5'3"), Proportionally longer limbs
pub const AVG_AFRICAN_FEMALE: AnatomyTemplate = AnatomyTemplate {
    arm_measurements: ArmMeasurements {
        upper_length: Centimeters(35),
        triceps_length: Centimeters(23),
        span: Centimeters(164),
        flexed_bicep: Centimeters(31),
        unflexed_bicep: Centimeters(29),
    },
    leg_measurements: LegMeasurements {
        inseam: Centimeters(75),
        quad: Centimeters(53),
    },
};
