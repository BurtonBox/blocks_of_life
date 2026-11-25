use biology::{Nomenclature, Organism, OrganismCore, Taxonomy};
use biology::Anatomy;
use biology::Mobility;
use biology::Summarizable;
use biology::VitalStatus;

/// A generic mammal - more specific than Animal, demonstrates class-level characteristics.
///
/// Mammals are warm-blooded vertebrates with hair/fur and mammary glands.
/// This struct composes the shared `OrganismCore` and adds mammalian-specific
/// traits while remaining simpler than detailed implementations like Human.
#[derive(Debug)]
pub struct Mammal {
    core: OrganismCore,
    pub has_fur: bool,
}

impl Mammal {
    /// Creates a living mammal of the given species.
    ///
    /// # Panics
    ///
    /// Panics if `taxonomy` is not in the class Mammalia.
    pub fn new(taxonomy: &'static Taxonomy) -> Self {
        assert_eq!(
            taxonomy.class, "Mammalia",
            "Mammal::new requires a Mammalia taxonomy, got {}",
            taxonomy
        );
        Self {
            core: OrganismCore::new(taxonomy),
            has_fur: true, // Most mammals have fur
        }
    }

    pub fn with_vitals(mut self, vitals: VitalStatus) -> Self {
        self.core = self.core.with_vitals(vitals);
        self
    }

    pub fn with_fur(mut self, has_fur: bool) -> Self {
        self.has_fur = has_fur;
        self
    }
}

impl Organism for Mammal {
    fn core(&self) -> &OrganismCore {
        &self.core
    }
}

impl Nomenclature for Mammal {
    fn display_name(&self) -> String {
        self.taxonomy().common_name.to_string()
    }
}

impl Anatomy for Mammal {
    fn describe_anatomy(&self) -> String {
        let fur_status = if self.has_fur { "with fur" } else { "hairless" };
        format!(
            "A {} - warm-blooded mammal {} and mammary glands.",
            self.taxonomy().common_name,
            fur_status
        )
    }
}

impl Mobility for Mammal {
    fn describe_locomotion(&self) -> String {
        "Mammalian locomotion (walking, running, or swimming).".to_string()
    }
}

impl Summarizable for Mammal {
    fn summary(&self) -> String {
        format!(
            "--- Generic Mammal: {} ---\nID: {}\nAnatomy: {}\nMobility: {}",
            self.taxonomy(),
            self.id(),
            self.describe_anatomy(),
            self.describe_locomotion()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::catalog;
    use biology::LiveVitals;

    #[test]
    fn test_mammal_creation() {
        let mammal = Mammal::new(&catalog::FELIS_CATUS);

        assert_eq!(mammal.taxonomy(), &catalog::FELIS_CATUS);
        assert_eq!(mammal.has_fur, true);
        assert!(mammal.is_alive());
    }

    #[test]
    fn test_mammal_with_fur_false() {
        let mammal = Mammal::new(&catalog::HETEROCEPHALUS_GLABER).with_fur(false);

        assert_eq!(mammal.has_fur, false);
    }

    #[test]
    fn test_mammal_nomenclature() {
        let mammal = Mammal::new(&catalog::LOXODONTA_AFRICANA);
        assert_eq!(mammal.display_name(), "African Bush Elephant");
        assert_eq!(mammal.scientific_name(), "Loxodonta africana");
    }

    #[test]
    fn test_mammal_anatomy_with_fur() {
        let mammal = Mammal::new(&catalog::CANIS_LUPUS);
        let anatomy = mammal.describe_anatomy();

        assert!(anatomy.contains("Gray Wolf"));
        assert!(anatomy.contains("with fur"));
        assert!(anatomy.contains("warm-blooded"));
    }

    #[test]
    fn test_mammal_anatomy_hairless() {
        let mammal = Mammal::new(&catalog::TURSIOPS_TRUNCATUS).with_fur(false);
        let anatomy = mammal.describe_anatomy();

        assert!(anatomy.contains("hairless"));
    }

    #[test]
    fn test_mammal_mobility() {
        let mammal = Mammal::new(&catalog::CANIS_LUPUS);
        let mobility = mammal.describe_locomotion();

        assert!(mobility.contains("Mammalian locomotion"));
    }

    #[test]
    fn test_mammal_builder_pattern() {
        let mammal = Mammal::new(&catalog::TURSIOPS_TRUNCATUS)
            .with_fur(false)
            .with_vitals(VitalStatus::Alive(LiveVitals::default()));

        assert_eq!(mammal.scientific_name(), "Tursiops truncatus");
        assert_eq!(mammal.has_fur, false);
    }

    #[test]
    #[should_panic(expected = "requires a Mammalia taxonomy")]
    fn test_mammal_rejects_non_mammal_taxonomy() {
        Mammal::new(&catalog::SALMO_SALAR);
    }
}
