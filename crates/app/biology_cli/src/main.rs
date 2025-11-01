use biology::{templates, Nomenclature, Sex};
use biology::anatomy::body::Body;
use biology::ecosystem::environment::{Climate, Environment, Terrain};
use biology::ecosystem::interactions::Interaction;
use biology::ecosystem::population::Population;
use biology::nomenclature::nomenclature::NomenclatureComponents;
use biology::vitals::blood_pressure::BloodPressure;
use biology::vitals::live_vitals::LiveVitals;
use biology::vitals::vital_status::VitalStatus;
use biology::vitals::vital_types::{Celsius, BPM};
use biology_animalia::{Human, NameParts};
use core_shared::Centimeters;

fn main() {

    let stephen = Human::builder()
        .name(NameParts::from("Stephen Burton"))
        .sex(Sex::Male)
        .with_vitals(VitalStatus::Alive(LiveVitals::new(
            BloodPressure(130, 40),
            Celsius(31),
            BPM(25),
            25,
        )))
        .with_anatomy_template(&templates::regional_defaults::AVG_NORTH_AMERICAN_MALE)
        .build();

    println!("{}", stephen.get_vitals());
    println!("{} {}", stephen.first_name().unwrap(), stephen.last_name().unwrap());

    // Create an ecosystem with environmental context
    let desert_environment = Environment {
        climate: Climate::Desert,
        terrain: Terrain::Plains,
        altitude: Centimeters(42800),
        humidity: 0.14,
    };

    let mut desert_ecosystem = Population::new(desert_environment);

    // Add members to the ecosystem
    desert_ecosystem.add_member(Body::new(&stephen));

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

    println!(
        "{} : {} ({})",
        stephen.id,
        stephen.display_name(),
        stephen.sex
    );

}
