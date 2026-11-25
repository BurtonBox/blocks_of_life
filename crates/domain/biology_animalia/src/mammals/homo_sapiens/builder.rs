use biology::Appendage;
use crate::catalog::HOMO_SAPIENS;
use crate::{Human, Moniker, NameParts};
use biology::ArmMeasurements;
use biology::LegMeasurements;
use biology::LimbStatus;
use biology::LiveVitals;
use biology::VitalStatus;
use core_shared::Direction;
use biology::{OrganismCore, Sex};
use biology::AnatomyTemplate;

pub struct HumanBuilder {
    name: Moniker,
    sex: Sex,
    vitals: VitalStatus,
    arm_measurements: ArmMeasurements,
    leg_measurements: LegMeasurements,
}

impl HumanBuilder {
    pub fn new() -> Self {
        Self {
            name: Moniker::Name(NameParts::default()),
            sex: Sex::Unknown,
            arm_measurements: ArmMeasurements::default(),
            leg_measurements: LegMeasurements::default(),
            vitals: VitalStatus::Alive(LiveVitals::default()),
        }
    }

    /// Sets the human's name. Accepts anything that converts into a `Moniker`:
    ///
    /// - `Designation::from("Case #12345")`
    /// - `NameParts::from("Jane Doe")`
    /// - `NameParts::new(Some("Jane"), Some("Marie"), Some("Doe"))`
    pub fn name(mut self, name: impl Into<Moniker>) -> Self {
        self.name = name.into();
        self
    }

    pub fn sex(mut self, sex: Sex) -> Self {
        self.sex = sex;
        self
    }

    pub fn vitals(mut self, vital_status: VitalStatus) -> Self {
        self.vitals = vital_status;
        self
    }

    // A method to load all measurements from a template.
    pub fn with_anatomy_template(mut self, template: &AnatomyTemplate) -> Self {
        self.arm_measurements = template.arm_measurements;
        self.leg_measurements = template.leg_measurements;
        self
    }

    // A method to override just the arm measurements.
    pub fn with_arm_measurements(mut self, measurements: ArmMeasurements) -> Self {
        self.arm_measurements = measurements;
        self
    }

    pub fn with_vitals(mut self, vitals: VitalStatus) -> Self {
        self.vitals = vitals;
        self
    }

    pub fn build(self) -> Human {
        let appendages = vec![
            Appendage::Arm {
                side: Direction::Left,
                measurements: self.arm_measurements, // Use the configured measurements
                status: LimbStatus::Intact,
            },
            Appendage::Arm {
                side: Direction::Right,
                measurements: self.arm_measurements,
                status: LimbStatus::Intact,
            },
            Appendage::Leg {
                side: Direction::Left,
                measurements: self.leg_measurements,
                status: LimbStatus::Intact,
            },
            Appendage::Leg {
                side: Direction::Right,
                measurements: self.leg_measurements,
                status: LimbStatus::Intact,
            },
        ];

        Human {
            core: OrganismCore::new(&HOMO_SAPIENS).with_vitals(self.vitals),
            name: self.name,
            sex: self.sex,
            appendages,
        }
    }
}
