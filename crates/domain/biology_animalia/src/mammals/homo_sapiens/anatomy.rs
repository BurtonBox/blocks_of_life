use biology::Anatomy;
use biology::Appendage;
use biology::AnatomyAnalyzer;


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