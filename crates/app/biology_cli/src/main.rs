use biology::{templates, Nomenclature, Sex};
use biology::anatomy::body::Body;
use biology::ecosystem::environment::{Climate, Environment, Terrain};
use biology::ecosystem::interactions::Interaction;
use biology::ecosystem::population::Population;
use biology::vitals::blood_pressure::BloodPressure;
use biology::vitals::live_vitals::LiveVitals;
use biology::vitals::vital_status::VitalStatus;
use biology::vitals::vital_types::{Celsius, BPM};
use biology_animalia::{Human, NameParts};
use core_shared::Centimeters;

fn main() {
    let stephen = Human::builder()
        .designation(String::from("Case #10288377"))
        .sex(Sex::Male)
        .with_vitals(VitalStatus::Alive(LiveVitals::new(
            BloodPressure(130, 40),
            Celsius(31),
            BPM(25),
            25,
        )))
        .using_template(&templates::regional_defaults::AVG_NORTH_AMERICAN_MALE)
        .build();

    println!("{}", stephen.get_vitals());

    let john = Human::builder()
        .name(NameParts::from_full_name("John Hicks"))
        .sex(Sex::Male)
        .with_vitals(VitalStatus::Alive(LiveVitals::new(
            BloodPressure(130, 40),
            Celsius(31),
            BPM(25),
            25,
        )))
        .using_template(&templates::regional_defaults::AVG_NORTH_AMERICAN_MALE)
        .build();

    // Create an ecosystem with environmental context
    let forest_environment = Environment {
        climate: Climate::Temperate,
        terrain: Terrain::Forest,
        altitude: Centimeters(500),
        humidity: 0.7,
    };

    let mut forest_ecosystem = Population::new(forest_environment);

    // Add members to the ecosystem
    forest_ecosystem.add_member(Body::new(&stephen));
    forest_ecosystem.add_member(Body::new(&john));

    // Add some ecological interactions
    forest_ecosystem.add_interaction(Interaction::Mutualism {
        participant1_id: 0, // Stephen
        participant2_id: 4, // Yucca plant
        benefit: "Humans provide care, plants provide oxygen".to_string(),
    });

    forest_ecosystem.add_interaction(Interaction::Competition {
        competitor1_id: 0, // Stephen
        competitor2_id: 1, // Shuyun
        resource: "Food resources".to_string(),
    });

    println!("Forest Ecosystem:");
    println!("Environment: {:?}", forest_ecosystem.environment);
    println!("Population size: {}", forest_ecosystem.size());
    println!("Interactions: {} recorded", forest_ecosystem.interactions.len());
    println!("");

    for body in &forest_ecosystem.members {
        body.display_summary();
        println!("");
    }

    println!(
        "{} : {} ({})",
        stephen.id,
        stephen.display_name(),
        stephen.sex
    );
    println!("{} : {} ({})", john.id, john.display_name(), john.sex);
}
