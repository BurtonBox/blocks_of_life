use biology::{Nomenclature, Organism, Sex};
use biology::Anatomy;
use biology::Mobility;
use biology::Summarizable;
use biology::regional_defaults;
use biology::BloodPressure;
use biology::LiveVitals;
use biology::VitalStatus;
use biology::{Celsius, BPM};
use biology_animalia::{Designation, Human, NameParts};

#[test]
fn test_human_creation_with_builder() {
    let human = Human::builder()
        .name(NameParts::from("John Doe"))
        .sex(Sex::Male)
        .with_anatomy_template(&regional_defaults::AVG_NORTH_AMERICAN_MALE)
        .build();

    assert!(matches!(human.sex, Sex::Male));
    assert_eq!(human.display_name(), "John Doe");
}

#[test]
fn test_human_with_designation() {
    let human = Human::builder()
        .name(Designation::from("Case #12345"))
        .sex(Sex::Female)
        .with_anatomy_template(&regional_defaults::AVG_EAST_ASIAN_FEMALE)
        .build();

    assert_eq!(human.display_name(), "Case #12345");
}

#[test]
fn test_human_name_parts_parsing() {
    let name = NameParts::from("Jane Smith");
    let human = Human::builder()
        .name(name)
        .sex(Sex::Female)
        .with_anatomy_template(&regional_defaults::AVG_SOUTH_ASIAN_FEMALE)
        .build();

    let display = human.display_name();
    assert!(display.contains("Jane"));
    assert!(display.contains("Smith"));
}

#[test]
fn test_human_with_vitals() {
    let vitals = VitalStatus::Alive(LiveVitals::new(
        BloodPressure(120, 80),
        Celsius(37),
        BPM(72),
        16,
    ));

    let human = Human::builder()
        .name(Designation::from("Test Subject"))
        .sex(Sex::Unknown)
        .with_vitals(vitals)
        .with_anatomy_template(&regional_defaults::AVG_NORTH_AMERICAN_MALE)
        .build();

    assert!(matches!(human.vitals(), VitalStatus::Alive(_)));
}

#[test]
fn test_human_anatomy_trait() {
    let human = Human::builder()
        .name(Designation::from("Test"))
        .sex(Sex::Male)
        .with_anatomy_template(&regional_defaults::AVG_NORTH_AMERICAN_MALE)
        .build();

    let anatomy = human.describe_anatomy();
    assert!(anatomy.contains("bipedal"));
    assert!(anatomy.contains("arms"));
    assert!(anatomy.contains("legs"));
}

#[test]
fn test_human_mobility_trait() {
    let human = Human::builder()
        .name(Designation::from("Test"))
        .sex(Sex::Male)
        .with_anatomy_template(&regional_defaults::AVG_NORTH_AMERICAN_MALE)
        .build();

    let mobility = human.describe_locomotion();
    assert!(mobility.contains("two legs"));
}

#[test]
fn test_human_summarizable_trait() {
    let human = Human::builder()
        .name(NameParts::from("Test Person"))
        .sex(Sex::Male)
        .with_anatomy_template(&regional_defaults::AVG_NORTH_AMERICAN_MALE)
        .build();

    let summary = human.summary();
    assert!(summary.contains("Human"));
    assert!(summary.contains("Test Person"));
    assert!(summary.contains("Anatomy:"));
    assert!(summary.contains("Mobility:"));
}

#[test]
fn test_human_builder_uses_template() {
    let human = Human::builder()
        .name(Designation::from("Template Test"))
        .sex(Sex::Male)
        .with_anatomy_template(&regional_defaults::AVG_NORTH_AMERICAN_MALE)
        .build();

    // Verify appendages were created
    assert_eq!(human.appendages.len(), 4); // 2 arms + 2 legs
}

#[test]
fn test_human_unique_identifiers() {
    let human1 = Human::builder()
        .name(Designation::from("Human 1"))
        .sex(Sex::Male)
        .with_anatomy_template(&regional_defaults::AVG_NORTH_AMERICAN_MALE)
        .build();

    let human2 = Human::builder()
        .name(Designation::from("Human 2"))
        .sex(Sex::Female)
        .with_anatomy_template(&regional_defaults::AVG_NORTH_AMERICAN_MALE)
        .build();

    // IDs should be unique (just verify they're different via string representation)
    assert_ne!(format!("{}", human1.id()), format!("{}", human2.id()));
}
