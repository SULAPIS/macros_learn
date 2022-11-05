pub mod generate {
    use basic::generate;

    generate!("basic/fixtures/person.json");
}

use generate::*;

fn main() {
    let person = Person {
        first_name: "lapis".into(),
        last_name: "lazuli".into(),
        skill: Skill {
            name: "water".into(),
        },
    };
    println!("{:#?}", person);
}
