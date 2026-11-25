use crate::{HumanBuilder, Moniker};
use biology::{Nomenclature, Organism, OrganismCore, Sex};
use biology::Appendage;
use biology::LimbStatus;
use biology::VitalStatus;
use core_shared::Direction;

pub struct Human {
    // Identity, taxonomy (always Homo sapiens) and vitals. Set by `HumanBuilder`.
    pub(crate) core: OrganismCore,
    pub name: Moniker,
    pub sex: Sex,
    pub appendages: Vec<Appendage>,
}

impl Organism for Human {
    fn core(&self) -> &OrganismCore {
        &self.core
    }
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
        match self.vitals() {
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
