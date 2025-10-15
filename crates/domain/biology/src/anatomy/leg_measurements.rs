use core_shared::Centimeters;

#[derive(Debug, Copy, Clone)]
pub struct LegMeasurements {
    pub inseam: Centimeters,
    pub quad: Centimeters,
}

impl Default for LegMeasurements {
    fn default() -> Self {
        Self {
            inseam: Centimeters(81),
            quad: Centimeters(58),
        }
    }
}
