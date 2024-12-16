
-
// #[derive(Default,Debug)] // this is going to derive the default values for all the fields
#[derive(Debug)]
struct Person {
    first_name: String,
    last_name: String,
    age: u8,
    loction: String,
}

impl Default for Person {
    fn default() -> Self {
        return Person {
            first_name: String::from("lorem"),
            last_name: String::from("ipsum"),
            age: 231u8,
            loction: String::from("value"),
        };
    }
}

fn main() {
    println!("Hello, world!");

    let person1 = Person::default();
    std::println!("{:#?}", person1);
    std::println!("{}",person1.first_name);
    std::println!("{}",person1.last_name);
    std::println!("{}",person1.age);
    std::println!("{}",person1.loction);
    std::println!("{:#?}", Person::default());


}
