use biology::Nomenclature;
use biology::characteristics::structural::Anatomy;
use biology::characteristics::presentation::Summarizable;
use biology::templates::regional_defaults;
use biology_animalia::{Animal, Mammal, Human, NameParts};
use biology::Sex;

/// Test that demonstrates progressive complexity from Animal → Mammal → Human
#[test]
fn test_progressive_complexity() {
    // Simplest: Generic Animal
    let animal = Animal::new("Generic Bird".to_string());
    assert_eq!(animal.display_name(), "Generic Bird");

    // Intermediate: Generic Mammal (adds class-specific traits)
    let mammal = Mammal::new("Generic Cat".to_string());
    assert_eq!(mammal.display_name(), "Generic Cat");
    assert_eq!(mammal.has_fur, true);

    // Complex: Detailed Human (full builder, detailed anatomy)
    let human = Human::builder()
        .name(NameParts::from_full_name("John Doe"))
        .sex(Sex::Male)
        .using_template(&regional_defaults::AVG_NORTH_AMERICAN_MALE)
        .build();
    assert_eq!(human.display_name(), "John Doe");
    assert_eq!(human.appendages.len(), 4);
}

/// Test that all types implement Summarizable trait
#[test]
fn test_all_types_summarizable() {
    let animal = Animal::new("Test Animal".to_string());
    let mammal = Mammal::new("Test Mammal".to_string());
    let human = Human::builder()
        .designation("Test Human".to_string())
        .sex(Sex::Unknown)
        .using_template(&regional_defaults::AVG_NORTH_AMERICAN_MALE)
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
    let _animal: Animal = Animal::new("Test".to_string());
    let _mammal: Mammal = Mammal::new("Test".to_string());
    let _human: Human = Human::builder()
        .designation("Test".to_string())
        .sex(Sex::Male)
        .using_template(&regional_defaults::AVG_NORTH_AMERICAN_MALE)
        .build();

    // If this compiles, the re-exports work correctly
    assert!(true);
}

/// Test that demonstrates polymorphism through trait objects
#[test]
fn test_polymorphic_collection() {
    let animal = Animal::new("Polymorphic Test".to_string());
    let mammal = Mammal::new("Polymorphic Test".to_string());
    let human = Human::builder()
        .designation("Polymorphic Test".to_string())
        .sex(Sex::Male)
        .using_template(&regional_defaults::AVG_NORTH_AMERICAN_MALE)
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
    let furry_mammal = Mammal::new("Wolf".to_string());
    let hairless_mammal = Mammal::new("Dolphin".to_string()).with_fur(false);

    assert_eq!(furry_mammal.has_fur, true);
    assert_eq!(hairless_mammal.has_fur, false);

    let furry_anatomy = furry_mammal.describe_anatomy();
    let hairless_anatomy = hairless_mammal.describe_anatomy();

    assert!(furry_anatomy.contains("with fur"));
    assert!(hairless_anatomy.contains("hairless"));
}
