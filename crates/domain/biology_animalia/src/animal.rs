use biology::{Kingdom, Nomenclature, Organism, OrganismCore, Taxonomy};
use biology::Anatomy;
use biology::Mobility;
use biology::Summarizable;
use biology::VitalStatus;

/// A generic animal - the simplest representation of an organism.
///
/// This struct demonstrates the baseline structure that all animals share,
/// with minimal complexity: just the shared `OrganismCore`. More specific
/// animal types (like `Mammal` or `Human`) compose the same core and add
/// their own fields and behavior.
#[derive(Debug)]
pub struct Animal {
    core: OrganismCore,
}

impl Animal {
    /// Creates a living animal of the given species.
    ///
    /// # Panics
    ///
    /// Panics if `taxonomy` is not in the Animalia kingdom.
    pub fn new(taxonomy: &'static Taxonomy) -> Self {
        assert_eq!(
            taxonomy.kingdom,
            Kingdom::Animalia,
            "Animal::new requires an Animalia taxonomy, got {}",
            taxonomy
        );
        Self { core: OrganismCore::new(taxonomy) }
    }

    pub fn with_vitals(mut self, vitals: VitalStatus) -> Self {
        self.core = self.core.with_vitals(vitals);
        self
    }
}

impl Organism for Animal {
    fn core(&self) -> &OrganismCore {
        &self.core
    }
}

impl Nomenclature for Animal {
    fn display_name(&self) -> String {
        self.taxonomy().common_name.to_string()
    }
}

impl Anatomy for Animal {
    fn describe_anatomy(&self) -> String {
        format!("A {} with standard anatomical structure.", self.taxonomy().common_name)
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
    use biology::PostMortemReport;

    #[test]
    fn test_animal_creation() {
        let animal = Animal::new(&catalog::CANIS_LUPUS);

        assert_eq!(animal.taxonomy(), &catalog::CANIS_LUPUS);
        assert!(animal.is_alive());
    }

    #[test]
    fn test_animal_with_vitals() {
        let deceased_report = PostMortemReport {
            cause_of_death: "Natural causes".to_string(),
            date_of_death: "2024-01-01".to_string(),
            findings: vec![],
        };

        let animal = Animal::new(&catalog::FELIS_CATUS)
            .with_vitals(VitalStatus::Deceased(deceased_report));

        assert!(matches!(animal.vitals(), VitalStatus::Deceased(_)));
    }

    #[test]
    fn test_animal_nomenclature() {
        let animal = Animal::new(&catalog::FELIS_CATUS);
        assert_eq!(animal.display_name(), "Domestic Cat");
        assert_eq!(animal.scientific_name(), "Felis catus");
    }

    #[test]
    fn test_animal_anatomy() {
        let animal = Animal::new(&catalog::PASSER_DOMESTICUS);
        let anatomy = animal.describe_anatomy();
        assert!(anatomy.contains("House Sparrow"));
        assert!(anatomy.contains("anatomical structure"));
    }

    #[test]
    fn test_animal_mobility() {
        let animal = Animal::new(&catalog::SALMO_SALAR);
        let mobility = animal.describe_locomotion();
        assert!(mobility.contains("species-appropriate"));
    }

    #[test]
    fn test_animal_summary() {
        let animal = Animal::new(&catalog::SALMO_SALAR);
        let summary = animal.summary();

        assert!(summary.contains("Generic Animal"));
        assert!(summary.contains("Atlantic Salmon"));
        assert!(summary.contains("Salmo salar"));
        assert!(summary.contains("Anatomy:"));
        assert!(summary.contains("Mobility:"));
    }

    #[test]
    #[should_panic(expected = "requires an Animalia taxonomy")]
    fn test_animal_rejects_non_animalia_taxonomy() {
        const ENGLISH_OAK: Taxonomy = Taxonomy {
            kingdom: Kingdom::Plantae,
            phylum: "Tracheophyta",
            class: "Magnoliopsida",
            order: "Fagales",
            family: "Fagaceae",
            genus: "Quercus",
            species: "robur",
            common_name: "English Oak",
        };
        Animal::new(&ENGLISH_OAK);
    }
}
