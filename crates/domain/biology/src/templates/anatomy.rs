use crate::anatomy::ArmMeasurements;
use crate::anatomy::LegMeasurements;

pub struct AnatomyTemplate {
    pub arm_measurements: ArmMeasurements,
    pub leg_measurements: LegMeasurements,
    // We could add others here later, like torso_measurements, etc.
}