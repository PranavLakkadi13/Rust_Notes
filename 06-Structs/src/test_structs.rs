#[derive(Debug)]
pub enum CarColor {
    Silver,
    Black,
    Red,
    White,
    Yellow,
    Blue,
}

#[derive(Debug)]
pub struct Car {
    // we need the pub to add visibility to the variables inside the sturct (called from other modules)
    // the pub struct just gives the visibility to the struct but not the internal variables
    // see below for an example to see the code doesn't compile if the toString isn't implemented
    pub no_of_seats: u8,
    pub engine: String,
    pub top_speed: u16,
    pub color: CarColor,
}

// this is more of a consise way of defining a struct and not having to write the struct variables
// also to access the variables in other file you u need to declare them as public
// this is more of a short hand struct that helps to use the "." to access the variables like a tuple
// bcoz like a tuple it allows multiple types and easy indexing using the "." and use indexes [0..]
#[derive(Debug)]
#[allow(dead_code)]
pub struct CarTuple(pub u8, pub String, pub u16, pub CarColor);

pub fn create_car_tuple() -> CarTuple {
    let car1 = CarTuple(5, "hybrid".to_string(), 180, CarColor::Black);
    return car1;
}

// this method is more custom but to do it in a defualt debug way like a hashmap we can use #[dereive(debug)]
// this is very helper to make the code printable and the compiler doesnt complain
impl ToString for CarColor {
    fn to_string(&self) -> String {
        match self {
            CarColor::Silver => "Silver".to_string(),
            CarColor::Black => "Black".to_string(),
            CarColor::Red => "Red".to_string(),
            CarColor::White => String::from("White"),
            CarColor::Yellow => String::from("Yellow"),
            CarColor::Blue => String::from("Blue"),
        }
    }
}

pub fn test_basic_struct_set_up() {
    let mut car1 = Car {
        no_of_seats: 2,
        engine: String::from("V6"),
        top_speed: 300,
        color: CarColor::Blue,
    };
    std::println!("{:?}", car1.engine);
    std::println!("{}", car1.top_speed);
    std::println!("{}", car1.no_of_seats);
    std::println!("{:?}", car1.color.to_string());

    car1.engine = String::from("V8-hybrid");
    std::println!("{:?}", car1.engine);
}

#[allow(dead_code)]
pub fn test_basic_struct_user_input() {
    let input_seats = &mut String::new();

    std::println!("Enter number of seats in car: ");
    std::io::stdin().read_line(input_seats).unwrap();
    let input_converted_seats = input_seats.trim().parse::<u8>().unwrap();

    let input_engine = &mut String::new();
    std::println!("Enter the engine type");
    std::io::stdin().read_line(input_engine).unwrap();

    let input_converted_engine = input_engine.trim().parse::<String>().unwrap();

    let input_top_speed = &mut String::new();
    std::println!("Enter the top speed");
    std::io::stdin().read_line(input_top_speed).unwrap();

    let input_converted_top_speed = input_top_speed.trim().parse::<u16>().unwrap();

    let input_color = &mut String::new();
    std::println!("Enter the Color of car option in number");
    std::io::stdin().read_line(input_color).unwrap();
    let input_color_value = input_color.trim().parse::<u8>().unwrap();

    let chosen_color = match input_color_value {
        0 => CarColor::Silver,
        1 => CarColor::Black,
        2 => CarColor::Red,
        3 => CarColor::White,
        4 => CarColor::Yellow,
        5 => CarColor::Blue,
        _ => CarColor::White,
    };

    let car1 = Car {
        no_of_seats: input_converted_seats,
        engine: input_converted_engine,
        top_speed: input_converted_top_speed,
        color: chosen_color,
    };
    std::println!("The desired featured of the car are::");
    std::println!("{:?}", car1.engine);
    std::println!("{}", car1.top_speed);
    std::println!("{}", car1.no_of_seats);
    std::println!("{}", car1.color.to_string());
    // this would have given an error due it bad debug of the custom type
    // std::println!("{:#?}", car1.color);
    // the custom debug type at the println time is to use [:?]
    std::println!("{:?}", car1.color);
    std::println!("{:?}", car1);
}
