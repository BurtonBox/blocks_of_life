use biology::Nomenclature;
use biology::Anatomy;
use biology::Summarizable;
use biology::regional_defaults;
use biology::Organism;
use biology_animalia::{catalog, Animal, Designation, Mammal, Human, NameParts};
use biology::Sex;

/// Test that demonstrates progressive complexity from Animal → Mammal → Human
#[test]
fn test_progressive_complexity() {
    // Simplest: Generic Animal
    let animal = Animal::new(&catalog::PASSER_DOMESTICUS);
    assert_eq!(animal.display_name(), "House Sparrow");

    // Intermediate: Generic Mammal (adds class-specific traits)
    let mammal = Mammal::new(&catalog::FELIS_CATUS);
    assert_eq!(mammal.display_name(), "Domestic Cat");
    assert_eq!(mammal.has_fur, true);

    // Complex: Detailed Human (full builder, detailed anatomy)
    let human = Human::builder()
        .name(NameParts::from("John Doe"))
        .sex(Sex::Male)
        .with_anatomy_template(&regional_defaults::AVG_NORTH_AMERICAN_MALE)
        .build();
    assert_eq!(human.display_name(), "John Doe");
    assert_eq!(human.appendages.len(), 4);
}

/// Test that all types implement Summarizable trait
#[test]
fn test_all_types_summarizable() {
    let animal = Animal::new(&catalog::SALMO_SALAR);
    let mammal = Mammal::new(&catalog::LOXODONTA_AFRICANA);
    let human = Human::builder()
        .name(Designation::from("Test Human"))
        .sex(Sex::Unknown)
        .with_anatomy_template(&regional_defaults::AVG_NORTH_AMERICAN_MALE)
        .build();

    // All should produce summaries
    assert!(animal.summary().contains("Generic Animal"));
    assert!(mammal.summary().contains("Generic Mammal"));
    assert!(human.summary().contains("Human"));
}

/// Test that convenience re-exports work (Option C)
#[test]
fn test_import_paths() {
    // These imports work because of re-exports in lib.rs
    let _animal: Animal = Animal::new(&catalog::SALMO_SALAR);
    let _mammal: Mammal = Mammal::new(&catalog::LOXODONTA_AFRICANA);
    let _human: Human = Human::builder()
        .name(Designation::from("Test"))
        .sex(Sex::Male)
        .with_anatomy_template(&regional_defaults::AVG_NORTH_AMERICAN_MALE)
        .build();

    // If this compiles, the re-exports work correctly
    assert!(true);
}

/// Test that demonstrates polymorphism through trait objects
#[test]
fn test_polymorphic_collection() {
    let animal = Animal::new(&catalog::SALMO_SALAR);
    let mammal = Mammal::new(&catalog::LOXODONTA_AFRICANA);
    let human = Human::builder()
        .name(Designation::from("Polymorphic Test"))
        .sex(Sex::Male)
        .with_anatomy_template(&regional_defaults::AVG_NORTH_AMERICAN_MALE)
        .build();

    // All can be treated as Summarizable
    let summaries: Vec<&dyn Summarizable> = vec![&animal, &mammal, &human];

    assert_eq!(summaries.len(), 3);
    for entity in summaries {
        let summary = entity.summary();
        assert!(!summary.is_empty());
    }
}

/// Test that different mammals can have different characteristics
#[test]
fn test_mammal_variation() {
    let furry_mammal = Mammal::new(&catalog::CANIS_LUPUS);
    let hairless_mammal = Mammal::new(&catalog::TURSIOPS_TRUNCATUS).with_fur(false);

    assert_eq!(furry_mammal.has_fur, true);
    assert_eq!(hairless_mammal.has_fur, false);

    let furry_anatomy = furry_mammal.describe_anatomy();
    let hairless_anatomy = hairless_mammal.describe_anatomy();

    assert!(furry_anatomy.contains("with fur"));
    assert!(hairless_anatomy.contains("hairless"));
}

/// Test that every organism type shares the same core through the Organism trait
#[test]
fn test_all_types_share_organism_core() {
    let animal = Animal::new(&catalog::PASSER_DOMESTICUS);
    let mammal = Mammal::new(&catalog::CANIS_LUPUS);
    let human = Human::builder()
        .name(Designation::from("Organism Test"))
        .sex(Sex::Female)
        .with_anatomy_template(&regional_defaults::AVG_AFRICAN_FEMALE)
        .build();

    let organisms: Vec<&dyn Organism> = vec![&animal, &mammal, &human];
    let names: Vec<String> = organisms.iter().map(|o| o.scientific_name()).collect();

    assert_eq!(names, ["Passer domesticus", "Canis lupus", "Homo sapiens"]);
    assert!(organisms.iter().all(|o| o.is_alive()));
    assert_ne!(organisms[0].id(), organisms[1].id());
    assert_eq!(human.taxonomy().order, "Primates");
}
