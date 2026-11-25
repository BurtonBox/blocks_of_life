use biology::{Nomenclature, Organism, Sex};
use biology::Body;
use biology::{Climate, Environment, Terrain};
use biology::Interaction;
use biology::Population;
use biology::regional_defaults;
use biology::BloodPressure;
use biology::LiveVitals;
use biology::VitalStatus;
use biology::{Celsius, BPM};
use biology_animalia::{Designation, Human, NameParts};
use core_shared::Centimeters;

fn main() {
    println!();

    // sample human with a designation instead of name:
    let human_test_case = Human::builder()
        .name(Designation::from("Case #12345"))
        .sex(Sex::Female)
        .with_vitals(VitalStatus::Alive(LiveVitals::new(
            BloodPressure(110, 35),
            Celsius(30),
            BPM(21),
            25,
        )))
        .with_anatomy_template(&regional_defaults::AVG_EAST_ASIAN_FEMALE)
        .build();

    // sample human with a name:
    let human = Human::builder()
        .name(NameParts::from("Stephen Burton"))
        .sex(Sex::Male)
        .with_vitals(VitalStatus::Alive(LiveVitals::new(
            BloodPressure(130, 40),
            Celsius(31),
            BPM(25),
            25,
        )))
        .with_anatomy_template(&regional_defaults::AVG_NORTH_AMERICAN_MALE)
        .build();

    println!("{} : {} ({})", human_test_case.id(), human_test_case.display_name(),human_test_case.sex);
    println!("{} : {} ({})", human.id(), human.display_name(), human.sex);
    println!();
    println!("{}", human_test_case.get_vitals());
    println!("{}", human.get_vitals());
    println!();

    // Create an ecosystem with environmental context
    let desert_environment = Environment {
        climate: Climate::Desert,
        terrain: Terrain::Plains,
        altitude: Centimeters(42800),
        humidity: 0.14,
    };

    let mut desert_ecosystem = Population::new(desert_environment);

    // Add members to the ecosystem
    desert_ecosystem.add_member(Body::new(&human));

    // Add some ecological interactions
    desert_ecosystem.add_interaction(Interaction::Predation {
        predator_id: 0,
        prey_id: 10,
    });

    desert_ecosystem.add_interaction(Interaction::Competition {
        competitor1_id: 0,
        competitor2_id: 1,
        resource: "Food resources".to_string(),
    });

    println!("Forest Ecosystem:");
    println!("Environment: {:?}", desert_ecosystem.environment);
    println!("Population size: {}", desert_ecosystem.size());
    println!("Interactions: {} recorded", desert_ecosystem.interactions.len());
    println!("");

    for body in &desert_ecosystem.members {
        body.display_summary();
        println!("");
    }

    println!();

}
