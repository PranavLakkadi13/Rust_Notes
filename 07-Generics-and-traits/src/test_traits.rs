// here we are creating a Pet struct and the pet value is of type PetType generic (meaning it can be anything u8 to str)
// and as seen after the PetType Generic we add type Animal which restrict the generic to certain type in the Animal trait
// the "+" in the generic type restiction is used to set a type that inherits both the properties of the different traits

// struct Person<PetType: Animal + NotDangerousType> {
struct Person<PetType>
where
    PetType: Animal + NotDangerousType,
{
    name: String,
    pet: PetType,
}

// here we are implementing trait of Animal (Traits in rust are like interfaces in OOP)
// when you know the impl is just going to print something return ()
trait Animal {
    fn make_sound(&self) -> ();
}

trait NotDangerousType {
    fn is_dangerous(&self) -> ();
}

// here is the struct Dog
struct Dog {
    age: u8,
}
// here we are implementing the interface of Animal to Dog and modifying
impl Animal for Dog {
    fn make_sound(&self) {
        std::println!("BARK!!")
    }
}
impl NotDangerousType for Dog {
    fn is_dangerous(&self) -> () {
        std::println!("False");
    }
}

// here we are just implementing methods to Dog type
impl Dog {
    fn breed(&self) -> () {
        std::println!("German Shepard")
    }
}

#[allow(dead_code)]
struct Cat {}
impl Animal for Cat {
    fn make_sound(&self) {
        std::println!("MEOW!!")
    }
}
impl NotDangerousType for Cat {
    fn is_dangerous(&self) -> () {
        std::println!("False");
    }
}

#[allow(dead_code)]
struct Tiger {}
impl Animal for Tiger {
    fn make_sound(&self) {
        std::println!("ROAR!!");
    }
}

pub fn impl_of_traits_and_generics() {
    // let pet1 = Tiger{};
    // let person1 = Person{name:"Pranav".to_string(), pet: pet1}; // u can see the error
    // std::println!("{}",person1.name);
    // person1.pet.make_sound();

    let pet2 = Dog { age: 33 };
    let person2 = Person {
        name: "Pranav".to_string(),
        pet: pet2,
    };
    person2.pet.breed();
    std::println!("{}", person2.name);
    std::println!("{}", person2.pet.age);
    person2.pet.is_dangerous();
    person2.pet.make_sound();
}
