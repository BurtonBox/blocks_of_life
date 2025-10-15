use core_shared::Direction;
use crate::anatomy::arm_measurements::ArmMeasurements;
use crate::anatomy::leg_measurements::LegMeasurements;
use crate::anatomy::limb_status::LimbStatus;

// The new, more powerful Appendage enum
#[derive(Debug)]
pub enum Appendage {
    Arm {
        side: Direction,
        measurements: ArmMeasurements,
        status: LimbStatus,
    },
    Leg {
        side: Direction,
        measurements: LegMeasurements,
        status: LimbStatus,
    },
    // We could later add variants with different data, for example:
    // Tail { length_cm: u32, status: Status },
}

impl Appendage {
    // This method creates a detailed string description of a single anatomy.
    pub fn description(&self) -> String {
        match self {
            Appendage::Arm {
                side,
                measurements,
                status,
            } => {
                format!(
                    "\t- {:?} Arm ({:?}): Span: {}cm, Bicep (Flexed): {}cm",
                    side, status, measurements.span.0, measurements.flexed_bicep.0
                )
            }
            Appendage::Leg {
                side,
                measurements,
                status,
            } => {
                format!(
                    "\t- {:?} Leg ({:?}): Inseam: {}cm, Quad: {}cm",
                    side, status, measurements.inseam.0, measurements.quad.0
                )
            }
        }
    }
}
