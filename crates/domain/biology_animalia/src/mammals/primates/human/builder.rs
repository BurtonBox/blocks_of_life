use biology::anatomy::appendage::Appendage;
use crate::{Human, Moniker, NameParts};
use biology::anatomy::arm_measurements::ArmMeasurements;
use biology::anatomy::leg_measurements::LegMeasurements;
use biology::anatomy::limb_status::LimbStatus;
use biology::vitals::live_vitals::LiveVitals;
use biology::vitals::vital_status::VitalStatus;
use core_shared::{Direction, Identifier};
use biology::Sex;
use biology::templates::anatomy::AnatomyTemplate;

pub struct HumanBuilder {
    pub id: Identifier,
    pub name_prefix: Option<String>,
    pub name_first: Option<String>,
    pub name_middle: Option<String>,
    pub name_last: Option<String>,
    pub name_suffix: Option<String>,
    pub name_moniker: Option<NameParts>,
    pub name_designation: Option<String>,
    pub sex: Sex,
    pub vitals: VitalStatus,
    pub arm_measurements: ArmMeasurements,
    pub leg_measurements: LegMeasurements,
}

impl HumanBuilder {
    pub fn new() -> Self {
        Self {
            id: Identifier::new(),
            name_prefix: None,
            name_first: None,
            name_middle: None,
            name_last: None,
            name_suffix: None,
            name_moniker: None,
            name_designation: None,
            sex: Sex::Unknown,
            arm_measurements: ArmMeasurements::default(),
            leg_measurements: LegMeasurements::default(),
            vitals: VitalStatus::Alive(LiveVitals::default()),
        }
    }

    pub fn designation(mut self, des: String) -> Self {
        self.name_designation = Some(des);
        self
    }

    pub fn name(mut self, moniker: NameParts) -> Self {
        self.name_moniker = Some(moniker);
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
    pub fn using_template(mut self, template: &AnatomyTemplate) -> Self {
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
        let name = if let Some(des) = self.name_designation {
            // If a designation was provided, it wins.
            Moniker::Designation(des)
        } else if let Some(mon) = self.name_moniker {
            Moniker::Name(mon)
        } else {
            // Otherwise, assemble a structured name from the parts.
            Moniker::Name(NameParts {
                prefix: self.name_prefix,
                first: self.name_first,
                middle: self.name_middle,
                last: self.name_last,
                suffix: self.name_suffix,
            })
        };

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
            id: self.id,
            name,
            sex: self.sex,
            vitals: self.vitals,
            appendages,
        }
    }
}
