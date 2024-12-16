pub fn run_test_traits() {
    let dog1 = Dog {};

    // make_some_noise(dog1); -> when the argument was generic
    make_some_noise(&dog1); // here since the trit was implemented by dog when the address is passed it will be able to run

    let dog2: &dyn Animal = &Dog {};
    make_some_noise(dog2);

    // -> if passing a struct or custom data type that implements the required trait always pass in as a reference
    // else it throws an error or look up at the above method ⬆️
    eat_some_food(&dog1);

    let antelope1 = Antelope {};
    eat_some_food(&antelope1);

    let bear1: &dyn AnimalEating = &Bear {};
    eat_some_food(bear1);

    let returned_animal = get_animal();
    returned_animal.eat_food();

    let supertrait_return_type = get_animal_animaltrait();
    supertrait_return_type.eat_food();
    supertrait_return_type.make_sound();
}

// this is the generic approach where u have a generic type "Animal" but constrainted to AnimalSound Type
// fn make_some_noise <Animal: AnimalSound>(a: Animal) {
//     a.make_sound();
// }

// Here we are using the dynamic dispatch as args and determining the type at runtime
fn make_some_noise(a: &dyn Animal) {
    a.make_sound();
}

fn eat_some_food(a: &dyn AnimalEating) {
    a.eat_food();
}

// the return type as the dynamic dispatch
// here u can see the value is returned as Box on the heap and
fn get_animal() -> Box<dyn AnimalEating> {
    // returns the type of dyn trait implemented value
    let animal = Antelope {}; // the initialised type of animal determies the  print statement
    return Box::from(animal);
}

fn get_animal_animaltrait() -> Box<dyn Animal> {
    let animal = Dog {};
    Box::from(animal)
}

// ------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------
// Below is all the code for the logic of the of the datatypes etc...

struct Dog {}

struct Antelope {}

struct Bear {}

// super trait -> implements the features of the child traits
trait Animal: AnimalEating + AnimalSound {}

trait AnimalEating {
    fn eat_food(&self);
}

trait AnimalSound {
    fn make_sound(&self);
}

impl AnimalEating for Dog {
    fn eat_food(&self) {
        std::println!("The Dog eats dog food!!");
    }
}

impl AnimalEating for Antelope {
    fn eat_food(&self) {
        std::println!("The Antelope eats plants!!");
    }
}

impl AnimalEating for Bear {
    fn eat_food(&self) {
        std::println!("The Bear eats Both Animals and Plants in the wild");
    }
}

impl AnimalSound for Dog {
    fn make_sound(&self) {
        std::println!("The Dog Barks");
    }
}

impl AnimalSound for Antelope {
    fn make_sound(&self) {
        std::println!("The Antelope Bleats");
    }
}

impl AnimalSound for Bear {
    fn make_sound(&self) {
        std::println!("The Bear Roars");
    }
}

// need to impl Animal Trait for all the structs to make it compilant to the Dynamic Dispatch
impl Animal for Dog {}

impl Animal for Bear {}

impl Animal for Antelope {}
