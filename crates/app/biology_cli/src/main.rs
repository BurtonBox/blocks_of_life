use biology::{Sex};
use biology_shared::Nomenclature;
use biology_animalia::{Human, NameParts};

fn main() {

    let stephen = Human::builder()
        .designation(String::from("Case #10288377"))
        .sex(Sex::Male)
        .build();

    let john = Human::builder()
        .name(NameParts::from_full_name("John Hicks"))
        .sex(Sex::Male)
        .build();

    println!("{} : {} ({})", stephen.id, stephen.display_name(), stephen.sex);
    println!("{} : {} ({})", john.id, john.display_name(), john.sex);

}
