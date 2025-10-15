use biology::{Nomenclature, Sex};
use biology::characteristics::structural::Anatomy;
use biology::characteristics::locomotion::Mobility;
use biology::characteristics::presentation::Summarizable;
use biology::templates::regional_defaults;
use biology::vitals::blood_pressure::BloodPressure;
use biology::vitals::live_vitals::LiveVitals;
use biology::vitals::vital_status::VitalStatus;
use biology::vitals::vital_types::{Celsius, BPM};
use biology_animalia::{Human, NameParts};

#[test]
fn test_human_creation_with_builder() {
    let human = Human::builder()
        .name(NameParts::from_full_name("John Doe"))
        .sex(Sex::Male)
        .using_template(&regional_defaults::AVG_NORTH_AMERICAN_MALE)
        .build();

    assert!(matches!(human.sex, Sex::Male));
    assert_eq!(human.display_name(), "John Doe");
}

#[test]
fn test_human_with_designation() {
    let human = Human::builder()
        .designation("Case #12345".to_string())
        .sex(Sex::Female)
        .using_template(&regional_defaults::AVG_NORTH_AMERICAN_MALE)
        .build();

    assert_eq!(human.display_name(), "Case #12345");
}

#[test]
fn test_human_name_parts_parsing() {
    let name = NameParts::from_full_name("Jane Smith");
    let human = Human::builder()
        .name(name)
        .sex(Sex::Female)
        .using_template(&regional_defaults::AVG_NORTH_AMERICAN_MALE)
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
        .designation("Test Subject".to_string())
        .sex(Sex::Unknown)
        .with_vitals(vitals)
        .using_template(&regional_defaults::AVG_NORTH_AMERICAN_MALE)
        .build();

    assert!(matches!(human.vitals, VitalStatus::Alive(_)));
}

#[test]
fn test_human_anatomy_trait() {
    let human = Human::builder()
        .designation("Test".to_string())
        .sex(Sex::Male)
        .using_template(&regional_defaults::AVG_NORTH_AMERICAN_MALE)
        .build();

    let anatomy = human.describe_anatomy();
    assert!(anatomy.contains("bipedal"));
    assert!(anatomy.contains("arms"));
    assert!(anatomy.contains("legs"));
}

#[test]
fn test_human_mobility_trait() {
    let human = Human::builder()
        .designation("Test".to_string())
        .sex(Sex::Male)
        .using_template(&regional_defaults::AVG_NORTH_AMERICAN_MALE)
        .build();

    let mobility = human.describe_locomotion();
    assert!(mobility.contains("two legs"));
}

#[test]
fn test_human_summarizable_trait() {
    let human = Human::builder()
        .name(NameParts::from_full_name("Test Person"))
        .sex(Sex::Male)
        .using_template(&regional_defaults::AVG_NORTH_AMERICAN_MALE)
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
        .designation("Template Test".to_string())
        .sex(Sex::Male)
        .using_template(&regional_defaults::AVG_NORTH_AMERICAN_MALE)
        .build();

    // Verify appendages were created
    assert_eq!(human.appendages.len(), 4); // 2 arms + 2 legs
}

#[test]
fn test_human_unique_identifiers() {
    let human1 = Human::builder()
        .designation("Human 1".to_string())
        .sex(Sex::Male)
        .using_template(&regional_defaults::AVG_NORTH_AMERICAN_MALE)
        .build();

    let human2 = Human::builder()
        .designation("Human 2".to_string())
        .sex(Sex::Female)
        .using_template(&regional_defaults::AVG_NORTH_AMERICAN_MALE)
        .build();

    // IDs should be unique (just verify they're different via string representation)
    assert_ne!(format!("{}", human1.id), format!("{}", human2.id));
}
