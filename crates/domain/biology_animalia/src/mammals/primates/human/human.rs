use crate::{HumanBuilder, Moniker};
use biology::{Nomenclature, Sex};
use biology::anatomy::appendage::Appendage;
use biology::anatomy::limb_status::LimbStatus;
use biology::vitals::vital_status::VitalStatus;
use core_shared::{Direction, Identifier};

pub struct Human {
    pub id: Identifier,
    pub name: Moniker,
    pub sex: Sex,
    pub appendages: Vec<Appendage>,
    pub vitals: VitalStatus,
}

impl Human {

    pub fn builder() -> HumanBuilder {
       HumanBuilder::new()
    }

    #[allow(unused_variables)]
    pub fn set_limb_status(
        &mut self,
        target_side: Direction,
        target_kind: &str,
        new_status: LimbStatus,
    ) {
        for appendage in &mut self.appendages {
            match appendage {
                Appendage::Arm {
                    side,
                    measurements,
                    status,
                } if side == &target_side && target_kind == "Arm" => {
                    *status = new_status;
                    return;
                }
                Appendage::Leg {
                    side,
                    measurements,
                    status,
                } if side == &target_side && target_kind == "Leg" => {
                    *status = new_status;
                    return;
                }
                _ => (),
            }
        }
    }

    pub fn get_vitals(&self) -> String {
        match &self.vitals {
            // We can destructure to get the whole 'vitals' struct
            VitalStatus::Alive(vitals) => {
                format!(
                    "{} is alive. Vitals: Temp: {:?}, Pulse: {:?}, BP: {:?}/{:?}",
                    self.name.display_name(),
                    vitals.temperature,
                    vitals.pulse,
                    vitals.blood_pressure.0,
                    vitals.blood_pressure.1
                )
            }
            // And here we get the whole 'report' struct
            VitalStatus::Deceased(report) => {
                format!(
                    "{} is deceased. Cause of death: {}.",
                    self.name.display_name(), report.cause_of_death
                )
            }
        }
    }
}
