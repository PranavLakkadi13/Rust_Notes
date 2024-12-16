use std::collections::HashSet;

pub fn create_impl_hashset() {
    let mut planet_names = HashSet::from(["Mercury", "Venus"]);

    let planet_names_2 = HashSet::from(["Mercury", "Venus", "Earth", "Mars"]);
    planet_names.insert("jupiter");

    let x = planet_names.get(&"Mercury");
    println!("{} ----- the get method returned value", x.is_some());

    // the .differnece is used just to get the elements in planet_names that are not in planet_names_2
    // this is used to add the elements of planet_names_2 which are not in planet_names to planet_names
    let y = planet_names.symmetric_difference(&planet_names_2);
    for z in y {
        std::println!("{}", z);
    }

    for planet in planet_names {
        std::println!("{}", planet);
    }

    let z = planet_names_2.is_empty();
    std::println!("{}", z);
}

pub fn names() {
    let mut names_of_people = HashSet::<String>::new();
    names_of_people.insert("value".to_string());
    std::println!("{:?}", names_of_people);
}
