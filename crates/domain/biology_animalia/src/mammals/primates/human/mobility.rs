use biology::patterns::locomotion_patterns::BipedalMobility;
use biology::characteristics::locomotion::Mobility;
use crate::Human;

impl BipedalMobility for Human {}

impl Mobility for Human {
    fn describe_locomotion(&self) -> String {
        self.describe_bipedal_movement()
    }
}
