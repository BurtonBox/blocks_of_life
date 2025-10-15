use biology::anatomy::appendage::Appendage;
use biology::patterns::presentation_patterns::{BasicSummary, DetailedSummary};
use biology::{Nomenclature, Sex, Summarizable};
use biology::characteristics::structural::Anatomy;
use biology::characteristics::locomotion::Mobility;
use crate::Human;

impl BasicSummary for Human {
    fn get_name(&self) -> String {
        self.name.display_name().clone()
    }

    fn get_gender(&self) -> &Sex {
        &self.sex
    }
}

impl DetailedSummary for Human {
    fn get_appendages(&self) -> &Vec<Appendage> {
        &self.appendages
    }
}

impl Summarizable for Human {
    fn summary(&self) -> String {
        println!("\n--- {} Body ---", self.name.display_name());
        format!(
            "--- Human: {} ---\nAnatomy: {}\nMobility: {}\n\n {}",
            self.name.display_name(),
            self.describe_anatomy(),
            self.describe_locomotion(),
            self.generate_summary()
        )
    }
}
