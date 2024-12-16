#[allow(dead_code)]
pub struct Person {
    first_name: String,
    last_name: String,
    age_range: (u8, u8),
}

pub fn test_match_on_int() {
    let myage: u16 = 15;

    // KEY NOTE -> the match expression must handle all possible cases not just if ==
    // also make sure that the matches depend on precedence that is a if the value is in 2 different branches
    // the branch on top is chosen
    match myage {
        // this is called the literal pattern since the matching is done with a literal
        35 => std::println!("The value is my age is 35"),
        // this is called the range pattern since a range is given
        1..35 => {
            std::println!("your age is just short of the age var");
        }
        // here the _ is used to represent all the possible other cases or the DEFAULT path to take if none matched
        // the default can be removed if all the possible cases are covered
        _ => {
            std::println!("I dont care about other cases");
        }
    }
}

pub fn test_match_on_int_armguard() {
    let myage: u16 = 15;

    let y: u8 = 5;

    match myage {
        35 => std::println!("The value is my age is 35"),
        1..35 if y == 5 => std::println!("the value of x is 35 and y is defined"),
        1..35 if y != 5 => std::println!("The value of y is not defined"),
        1..35 => {
            std::println!("your age is just short of the age var");
        }
        _ => {
            std::println!("I dont care about other cases");
        }
    }
}

pub fn test_match_on_string() {
    let car_manufacturer = "porche";

    match car_manufacturer {
        "Hyundai" => std::println!("Hyundai it is !!"),
        _ => std::println!("manufacturer not supported"),
    }
}

pub fn test_match_handle_return() -> u32 {
    let car_manufacturer = "Porche";

    // this match expression still returns a value
    // if there is a `;` at the end of the statment then that is it doesnt return aything at the fn scope
    // also handling the return is optional
    match car_manufacturer {
        "Hyundai" => 30000,
        "Porche" => 100000,
        _ => 0,
    } // here the `;` determines if the value will be returned at the function scope or not
}

pub fn test_match_array() {
    let prices: [u32; 4] = [30000, 50000, 90000, 120000];

    match prices[0..=2] {
        [30000, 50000] => std::println!("You have cars in budget"),
        [.., 90000] => std::println!("you have some choices in the range"),
        _ => std::println!("cars out of budget"),
    }
}

pub fn test_match_struct() {
    let person1 = Person {
        first_name: "Pranav".to_string(),
        last_name: "Lakkadi".to_string(),
        age_range: (2, 3),
    };
    
    #[allow(unused_variables)]
    let first_name_string: String = String::from("Pranav");
    
    #[allow(unused_variables)]
    match person1 {
        Person {
            first_name: first_name_string,
            ..
        } => {
            std::println!("matched first name");
        }
    }
}
