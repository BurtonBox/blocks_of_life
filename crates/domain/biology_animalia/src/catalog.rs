//! Species definitions for the Animalia kingdom.
//!
//! Each species is a `Taxonomy` constant: classification is data, not a
//! folder hierarchy. Add new species here; only species that need detailed
//! modeling (like `Human`) get their own type.

use biology::{Kingdom, Taxonomy};

// ============================================================================
// MAMMALIA
// ============================================================================

pub const HOMO_SAPIENS: Taxonomy = Taxonomy {
    kingdom: Kingdom::Animalia,
    phylum: "Chordata",
    class: "Mammalia",
    order: "Primates",
    family: "Hominidae",
    genus: "Homo",
    species: "sapiens",
    common_name: "Human",
};

pub const CANIS_LUPUS: Taxonomy = Taxonomy {
    kingdom: Kingdom::Animalia,
    phylum: "Chordata",
    class: "Mammalia",
    order: "Carnivora",
    family: "Canidae",
    genus: "Canis",
    species: "lupus",
    common_name: "Gray Wolf",
};

pub const FELIS_CATUS: Taxonomy = Taxonomy {
    kingdom: Kingdom::Animalia,
    phylum: "Chordata",
    class: "Mammalia",
    order: "Carnivora",
    family: "Felidae",
    genus: "Felis",
    species: "catus",
    common_name: "Domestic Cat",
};

pub const PANTHERA_LEO: Taxonomy = Taxonomy {
    kingdom: Kingdom::Animalia,
    phylum: "Chordata",
    class: "Mammalia",
    order: "Carnivora",
    family: "Felidae",
    genus: "Panthera",
    species: "leo",
    common_name: "Lion",
};

pub const HETEROCEPHALUS_GLABER: Taxonomy = Taxonomy {
    kingdom: Kingdom::Animalia,
    phylum: "Chordata",
    class: "Mammalia",
    order: "Rodentia",
    family: "Heterocephalidae",
    genus: "Heterocephalus",
    species: "glaber",
    common_name: "Naked Mole-Rat",
};

/// Cetaceans are classified within Artiodactyla (even-toed ungulates).
pub const TURSIOPS_TRUNCATUS: Taxonomy = Taxonomy {
    kingdom: Kingdom::Animalia,
    phylum: "Chordata",
    class: "Mammalia",
    order: "Artiodactyla",
    family: "Delphinidae",
    genus: "Tursiops",
    species: "truncatus",
    common_name: "Common Bottlenose Dolphin",
};

pub const LOXODONTA_AFRICANA: Taxonomy = Taxonomy {
    kingdom: Kingdom::Animalia,
    phylum: "Chordata",
    class: "Mammalia",
    order: "Proboscidea",
    family: "Elephantidae",
    genus: "Loxodonta",
    species: "africana",
    common_name: "African Bush Elephant",
};

// ============================================================================
// AVES
// ============================================================================

pub const PASSER_DOMESTICUS: Taxonomy = Taxonomy {
    kingdom: Kingdom::Animalia,
    phylum: "Chordata",
    class: "Aves",
    order: "Passeriformes",
    family: "Passeridae",
    genus: "Passer",
    species: "domesticus",
    common_name: "House Sparrow",
};

// ============================================================================
// ACTINOPTERYGII (ray-finned fishes)
// ============================================================================

pub const SALMO_SALAR: Taxonomy = Taxonomy {
    kingdom: Kingdom::Animalia,
    phylum: "Chordata",
    class: "Actinopterygii",
    order: "Salmoniformes",
    family: "Salmonidae",
    genus: "Salmo",
    species: "salar",
    common_name: "Atlantic Salmon",
};

#[cfg(test)]
mod tests {
    use super::*;

    const ALL: [&Taxonomy; 9] = [
        &HOMO_SAPIENS,
        &CANIS_LUPUS,
        &FELIS_CATUS,
        &PANTHERA_LEO,
        &HETEROCEPHALUS_GLABER,
        &TURSIOPS_TRUNCATUS,
        &LOXODONTA_AFRICANA,
        &PASSER_DOMESTICUS,
        &SALMO_SALAR,
    ];

    #[test]
    fn test_catalog_species_are_all_animalia() {
        assert!(ALL.iter().all(|t| t.kingdom == Kingdom::Animalia));
    }

    #[test]
    fn test_catalog_scientific_names_are_unique() {
        let mut names: Vec<String> = ALL.iter().map(|t| t.scientific_name()).collect();
        names.sort();
        names.dedup();
        assert_eq!(names.len(), ALL.len());
    }
}
