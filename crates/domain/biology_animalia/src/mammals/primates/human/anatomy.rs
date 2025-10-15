use biology::anatomy::appendage::Appendage;
use biology::patterns::structural_patterns::AnatomyAnalyzer;
use biology::characteristics::structural::Anatomy;
use crate::Human;

impl Anatomy for Human {
    fn describe_anatomy(&self) -> String {
        self.describe_limb_status()
    }
}

impl AnatomyAnalyzer for Human {
    fn get_appendages(&self) -> &Vec<Appendage> {
        &self.appendages
    }
}