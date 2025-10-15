use crate::vitals::blood_pressure::BloodPressure;
use crate::vitals::vital_types::{Celsius, BPM};

#[derive(Debug, Default)]
pub struct LiveVitals {
    pub blood_pressure: BloodPressure,
    pub temperature: Celsius,
    pub pulse: BPM,
    pub respiration_rate: u32,
}

impl LiveVitals {
    // This is the constructor you suggested!
    pub fn new(
        blood_pressure: BloodPressure,
        temperature: Celsius,
        pulse: BPM,
        respiration_rate: u32,
    ) -> Self {
        Self {
            blood_pressure,
            temperature,
            pulse,
            respiration_rate,
        }
    }
}
