//! Taxonomic classification as data.
//!
//! Taxonomy is carried by each organism as a value rather than encoded in the
//! module/folder hierarchy. Classifications change as the science is revised,
//! and the number of species is far too large to model as one type per rank,
//! so ranks live here as plain data that kingdom crates define as constants.

use std::fmt::{Display, Formatter};

use crate::classification::kingdom::Kingdom;

/// The Linnaean classification of a species, from kingdom down to species.
///
/// Kingdom crates define their species as `const` values (see
/// `biology_animalia::catalog`), in the same data-driven style as the
/// anatomy templates.
///
/// # Examples
///
/// ```rust
/// use biology::{Kingdom, Taxonomy};
///
/// const CANIS_LUPUS: Taxonomy = Taxonomy {
///     kingdom: Kingdom::Animalia,
///     phylum: "Chordata",
///     class: "Mammalia",
///     order: "Carnivora",
///     family: "Canidae",
///     genus: "Canis",
///     species: "lupus",
///     common_name: "Gray Wolf",
/// };
///
/// assert_eq!(CANIS_LUPUS.scientific_name(), "Canis lupus");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Taxonomy {
    pub kingdom: Kingdom,
    pub phylum: &'static str,
    pub class: &'static str,
    pub order: &'static str,
    pub family: &'static str,
    pub genus: &'static str,
    /// The specific epithet, e.g. `"sapiens"` in *Homo sapiens*.
    pub species: &'static str,
    pub common_name: &'static str,
}

impl Taxonomy {
    /// The binomial name, e.g. `"Homo sapiens"`.
    pub fn scientific_name(&self) -> String {
        format!("{} {}", self.genus, self.species)
    }
}

impl Display for Taxonomy {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({} {})", self.common_name, self.genus, self.species)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const HOMO_SAPIENS: Taxonomy = Taxonomy {
        kingdom: Kingdom::Animalia,
        phylum: "Chordata",
        class: "Mammalia",
        order: "Primates",
        family: "Hominidae",
        genus: "Homo",
        species: "sapiens",
        common_name: "Human",
    };

    #[test]
    fn test_scientific_name_is_binomial() {
        assert_eq!(HOMO_SAPIENS.scientific_name(), "Homo sapiens");
    }

    #[test]
    fn test_display_includes_common_and_scientific_name() {
        assert_eq!(HOMO_SAPIENS.to_string(), "Human (Homo sapiens)");
    }
}
