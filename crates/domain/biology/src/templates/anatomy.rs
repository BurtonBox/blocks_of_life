use crate::anatomy::arm_measurements::ArmMeasurements;
use crate::anatomy::leg_measurements::LegMeasurements;

pub struct AnatomyTemplate {
    pub arm_measurements: ArmMeasurements,
    pub leg_measurements: LegMeasurements,
    // We could add others here later, like torso_measurements, etc.
}