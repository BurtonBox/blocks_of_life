//! The shared core every organism is composed from.
//!
//! `OrganismCore` is the smallest "block of life": identity, classification
//! and vital state. Concrete organisms (`Animal`, `Mammal`, `Human`, ...)
//! hold one as a field instead of re-declaring those fields, and implement
//! the `Organism` trait with a single accessor to gain the shared behaviour.
//! This is composition, not inheritance: there is no base type.

use core_shared::Identifier;

use crate::classification::Taxonomy;
use crate::vitals::LiveVitals;
use crate::vitals::VitalStatus;

/// Identity, classification and vital state shared by every organism.
#[derive(Debug)]
pub struct OrganismCore {
    pub id: Identifier,
    pub taxonomy: &'static Taxonomy,
    pub vitals: VitalStatus,
}

impl OrganismCore {
    /// Creates a living organism of the given species with a fresh identifier
    /// and default vitals.
    pub fn new(taxonomy: &'static Taxonomy) -> Self {
        Self {
            id: Identifier::new(),
            taxonomy,
            vitals: VitalStatus::Alive(LiveVitals::default()),
        }
    }

    pub fn with_vitals(mut self, vitals: VitalStatus) -> Self {
        self.vitals = vitals;
        self
    }
}

/// Anything that is a living (or once-living) organism.
///
/// Implementors provide only `core()`; everything else comes from default
/// methods, following the same pattern as `AnatomyAnalyzer`.
///
/// # Examples
///
/// ```rust,ignore
/// use biology::{Organism, OrganismCore};
///
/// impl Organism for Animal {
///     fn core(&self) -> &OrganismCore { &self.core }
/// }
///
/// let wolf = Animal::new(&catalog::CANIS_LUPUS);
/// assert_eq!(wolf.scientific_name(), "Canis lupus");
/// ```
pub trait Organism {
    fn core(&self) -> &OrganismCore;

    fn id(&self) -> Identifier {
        self.core().id
    }

    fn taxonomy(&self) -> &'static Taxonomy {
        self.core().taxonomy
    }

    fn vitals(&self) -> &VitalStatus {
        &self.core().vitals
    }

    fn scientific_name(&self) -> String {
        self.taxonomy().scientific_name()
    }

    fn is_alive(&self) -> bool {
        matches!(self.vitals(), VitalStatus::Alive(_))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::classification::Kingdom;
    use crate::vitals::PostMortemReport;

    const TEST_SPECIES: Taxonomy = Taxonomy {
        kingdom: Kingdom::Animalia,
        phylum: "Chordata",
        class: "Mammalia",
        order: "Carnivora",
        family: "Canidae",
        genus: "Canis",
        species: "lupus",
        common_name: "Gray Wolf",
    };

    struct Specimen {
        core: OrganismCore,
    }

    impl Organism for Specimen {
        fn core(&self) -> &OrganismCore {
            &self.core
        }
    }

    #[test]
    fn test_new_core_is_alive_with_given_taxonomy() {
        let core = OrganismCore::new(&TEST_SPECIES);
        assert!(matches!(core.vitals, VitalStatus::Alive(_)));
        assert_eq!(core.taxonomy, &TEST_SPECIES);
    }

    #[test]
    fn test_each_core_gets_a_unique_id() {
        assert_ne!(OrganismCore::new(&TEST_SPECIES).id, OrganismCore::new(&TEST_SPECIES).id);
    }

    #[test]
    fn test_organism_defaults_delegate_to_core() {
        let specimen = Specimen { core: OrganismCore::new(&TEST_SPECIES) };
        assert_eq!(specimen.id(), specimen.core.id);
        assert_eq!(specimen.scientific_name(), "Canis lupus");
        assert!(specimen.is_alive());
    }

    #[test]
    fn test_deceased_organism_is_not_alive() {
        let specimen = Specimen {
            core: OrganismCore::new(&TEST_SPECIES)
                .with_vitals(VitalStatus::Deceased(PostMortemReport::default())),
        };
        assert!(!specimen.is_alive());
    }
}
