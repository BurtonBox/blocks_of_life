use biology::BipedalMobility;
use biology::Mobility;
use crate::Human;

impl BipedalMobility for Human {}

impl Mobility for Human {
    fn describe_locomotion(&self) -> String {
        self.describe_bipedal_movement()
    }
}
