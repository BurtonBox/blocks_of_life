use biology::Nomenclature;
use biology::characteristics::structural::Anatomy;
use biology::characteristics::locomotion::Mobility;
use biology::characteristics::presentation::Summarizable;
use biology::vitals::vital_status::VitalStatus;
use biology::vitals::live_vitals::LiveVitals;
use core_shared::Identifier;

/// A generic mammal - more specific than Animal, demonstrates class-level characteristics.
///
/// Mammals are warm-blooded vertebrates with hair/fur and mammary glands.
/// This struct adds mammalian-specific traits while remaining simpler than
/// detailed implementations like Human.
#[derive(Debug)]
pub struct Mammal {
    pub id: Identifier,
    pub species: String,
    pub vitals: VitalStatus,
    pub has_fur: bool,
}

impl Mammal {
    pub fn new(species: String) -> Self {
        Self {
            id: Identifier::new(),
            species,
            vitals: VitalStatus::Alive(LiveVitals::default()),
            has_fur: true, // Most mammals have fur
        }
    }

    pub fn with_vitals(mut self, vitals: VitalStatus) -> Self {
        self.vitals = vitals;
        self
    }

    pub fn with_fur(mut self, has_fur: bool) -> Self {
        self.has_fur = has_fur;
        self
    }
}

impl Nomenclature for Mammal {
    fn display_name(&self) -> String {
        self.species.clone()
    }
}

impl Anatomy for Mammal {
    fn describe_anatomy(&self) -> String {
        let fur_status = if self.has_fur { "with fur" } else { "hairless" };
        format!(
            "A {} - warm-blooded mammal {} and mammary glands.",
            self.species, fur_status
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
            self.species,
            self.id,
            self.describe_anatomy(),
            self.describe_locomotion()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mammal_creation() {
        let mammal = Mammal::new("Domestic Cat".to_string());

        assert_eq!(mammal.species, "Domestic Cat");
        assert_eq!(mammal.has_fur, true);
        assert!(matches!(mammal.vitals, VitalStatus::Alive(_)));
    }

    #[test]
    fn test_mammal_with_fur_false() {
        let mammal = Mammal::new("Hairless Mole Rat".to_string())
            .with_fur(false);

        assert_eq!(mammal.has_fur, false);
    }

    #[test]
    fn test_mammal_nomenclature() {
        let mammal = Mammal::new("Elephant".to_string());
        assert_eq!(mammal.display_name(), "Elephant");
    }

    #[test]
    fn test_mammal_anatomy_with_fur() {
        let mammal = Mammal::new("Wolf".to_string());
        let anatomy = mammal.describe_anatomy();

        assert!(anatomy.contains("Wolf"));
        assert!(anatomy.contains("with fur"));
        assert!(anatomy.contains("warm-blooded"));
    }

    #[test]
    fn test_mammal_anatomy_hairless() {
        let mammal = Mammal::new("Dolphin".to_string())
            .with_fur(false);
        let anatomy = mammal.describe_anatomy();

        assert!(anatomy.contains("hairless"));
    }

    #[test]
    fn test_mammal_mobility() {
        let mammal = Mammal::new("Horse".to_string());
        let mobility = mammal.describe_locomotion();

        assert!(mobility.contains("Mammalian locomotion"));
    }

    #[test]
    fn test_mammal_builder_pattern() {
        let mammal = Mammal::new("Test Mammal".to_string())
            .with_fur(false)
            .with_vitals(VitalStatus::Alive(LiveVitals::default()));

        assert_eq!(mammal.species, "Test Mammal");
        assert_eq!(mammal.has_fur, false);
    }
}
