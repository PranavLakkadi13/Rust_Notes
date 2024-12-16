use std::cell::Cell;

#[derive(Debug)]
#[allow(dead_code)]
pub struct Person {
    pub first_name: String,
    pub last_name: String,
    pub age: u8,
}

#[allow(dead_code)]
pub struct NewPerson {
    // here when using a String it has an issue that causes an error during compilation
    // the issue is due to code copy error

    // pub first_name: Cell<String>,
    pub first_name: Cell<String>,
    pub last_name: Cell<String>,
    pub age: Cell<u8>,
    pub gender: String,
}

pub fn create_person() {
    // here though we were able to add the mutability it is more of the the netire variable being mutable
    let mut person1 = Person {
        first_name: "Pranav".to_string(),
        last_name: "Reddy".to_string(),
        age: 21,
    };
    person1.first_name = "King".to_string();
    person1.last_name = "Khan".to_string();
    std::println!("{:?}", person1);
}

pub fn create_new_person_mut() {
    // this works because the variable is also being muted so doenst have issues and updates the code
    let mut person1 = NewPerson {
        first_name: Cell::new("Pranav".to_string()),
        last_name: Cell::new("Reddy".to_string()),
        age: Cell::new(21),
        gender: "Male".to_string(),
    };
    person1.first_name = Cell::new("King".to_string());
    person1.last_name = Cell::from("Khan".to_string());
    person1.last_name = Cell::from("Nitrogen".to_string());
    std::println!("{:?}", person1.first_name.get_mut());
    std::println!("{:?}", person1.last_name.get_mut());
}

#[allow(dead_code)]
pub fn create_new_person_non_mut_error() {
    #[allow(unused_variables)]
    let person1 = NewPerson {
        first_name: Cell::new("Pranav".to_string()),
        last_name: Cell::new("Reddy".to_string()),
        age: Cell::new(21),
        gender: "Male".to_string(),
    };
    // person1.first_name = Cell::new("King".to_string());
    // person1.last_name = Cell::from("Khan".to_string());
    // person1.last_name = Cell::from("Nitrogen".to_string());

    // std::println!("{:?}", person1.first_name.get()); // here we are getting the error due to String copy issue
    // std::println!("{:?}", person1.last_name.get());
}

#[allow(dead_code)]
pub struct PersonLifeTimes<'p> {
    // the <'p > is called lifetime which is sent during variable initialisation
    // can be static (will be as long as the variable is in use or exists)
    pub first_name: Cell<&'p str>,
    pub last_name: String,
    pub age: u8,
}

// the static lifetime means the life of the program
pub fn create_new_person_non_mut() -> PersonLifeTimes<'static> {
    let str = "Pranav";
    let person1 = PersonLifeTimes {
        first_name: Cell::from(str),
        last_name: "Reddy".to_string(),
        age: 21,
    };
    println!("{}", str);
    println!("{:?}", person1.first_name);
    person1.first_name.set("val updated"); // -> this is how u can update the value

    return person1;
}
