use core_shared::Centimeters;

#[derive(Debug, Copy, Clone)]
pub struct ArmMeasurements {
    pub upper_length: Centimeters,
    pub triceps_length: Centimeters,
    pub span: Centimeters,
    pub flexed_bicep: Centimeters,
    pub unflexed_bicep: Centimeters,
}

impl Default for ArmMeasurements {
    fn default() -> Self {
        Self {
            upper_length: Centimeters(38),
            triceps_length: Centimeters(25),
            span: Centimeters(175),
            flexed_bicep: Centimeters(35),
            unflexed_bicep: Centimeters(32),
        }
    }
}
