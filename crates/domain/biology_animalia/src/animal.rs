use biology::Nomenclature;
use biology::characteristics::structural::Anatomy;
use biology::characteristics::locomotion::Mobility;
use biology::characteristics::presentation::Summarizable;
use biology::vitals::vital_status::VitalStatus;
use biology::vitals::live_vitals::LiveVitals;
use core_shared::Identifier;

/// A generic animal - the simplest representation of an organism.
///
/// This struct demonstrates the baseline structure that all animals share,
/// with minimal complexity. More specific animal types (like Human) add
/// additional fields and behavior.
#[derive(Debug)]
pub struct Animal {
    pub id: Identifier,
    pub species: String,
    pub vitals: VitalStatus,
}

impl Animal {
    pub fn new(species: String) -> Self {
        Self {
            id: Identifier::new(),
            species,
            vitals: VitalStatus::Alive(LiveVitals::default()),
        }
    }

    pub fn with_vitals(mut self, vitals: VitalStatus) -> Self {
        self.vitals = vitals;
        self
    }
}

impl Nomenclature for Animal {
    fn display_name(&self) -> String {
        self.species.clone()
    }
}

impl Anatomy for Animal {
    fn describe_anatomy(&self) -> String {
        format!("A {} with standard anatomical structure.", self.species)
    }
}

impl Mobility for Animal {
    fn describe_locomotion(&self) -> String {
        "Moves in species-appropriate manner.".to_string()
    }
}

impl Summarizable for Animal {
    fn summary(&self) -> String {
        format!(
            "--- Generic Animal: {} ---\nID: {}\nAnatomy: {}\nMobility: {}",
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
    use biology::vitals::post_mortem_report::PostMortemReport;

    #[test]
    fn test_animal_creation() {
        let animal = Animal::new("Generic Dog".to_string());

        assert_eq!(animal.species, "Generic Dog");
        assert!(matches!(animal.vitals, VitalStatus::Alive(_)));
    }

    #[test]
    fn test_animal_with_vitals() {
        let deceased_report = PostMortemReport {
            cause_of_death: "Natural causes".to_string(),
            date_of_death: "2024-01-01".to_string(),
            findings: vec![],
        };

        let animal = Animal::new("Test Animal".to_string())
            .with_vitals(VitalStatus::Deceased(deceased_report));

        assert!(matches!(animal.vitals, VitalStatus::Deceased(_)));
    }

    #[test]
    fn test_animal_nomenclature() {
        let animal = Animal::new("Wild Cat".to_string());
        assert_eq!(animal.display_name(), "Wild Cat");
    }

    #[test]
    fn test_animal_anatomy() {
        let animal = Animal::new("Bird".to_string());
        let anatomy = animal.describe_anatomy();
        assert!(anatomy.contains("Bird"));
        assert!(anatomy.contains("anatomical structure"));
    }

    #[test]
    fn test_animal_mobility() {
        let animal = Animal::new("Fish".to_string());
        let mobility = animal.describe_locomotion();
        assert!(mobility.contains("species-appropriate"));
    }

    #[test]
    fn test_animal_summary() {
        let animal = Animal::new("Test Species".to_string());
        let summary = animal.summary();

        assert!(summary.contains("Generic Animal"));
        assert!(summary.contains("Test Species"));
        assert!(summary.contains("Anatomy:"));
        assert!(summary.contains("Mobility:"));
    }
}
