#[derive(Debug)]
#[allow(dead_code)]
pub enum CarColor {
    Silver,
    Black,
    Red,
    White,
    Yellow,
    Blue,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Car {
    pub no_of_seats: u8,
    pub engine: String,
    pub top_speed: u16,
    pub color: CarColor,
}

// Impl is basically an implementation of methods on a particular datatype(custom)
// since we know that structs are like objects the impl feature is like giving structs the ability to have
// functions just like a normal oop call
impl Car {
    // here since we are tryping to update a variable location it has to be a mutable reference
    // here since struct is used as a variable type we need to pass the first
    // since this is also updating the variable so when used the variable has to be mutable also in the func we marked
    // it to be mutable
    // this is an example of the instance method ->usually used in the case of mutable instance of a variable in the obj
    fn paint(&mut self, new_color: CarColor) {
        self.color = new_color;
    }

    // here is the example of a static method
    fn create_new_car() -> Car {
        let new_car = Car {
            engine: "default".to_string(),
            color: CarColor::default(),
            top_speed: 0,
            no_of_seats: 0,
        };
        new_car
    }
}

// this is the case when u use an impl on an enum when u want to return an current type back
impl CarColor {
    fn default() -> CarColor {
        CarColor::White
    }
}

pub fn run_impl() {
    let mut car1 = Car {
        no_of_seats: 4,
        engine: "v6".to_string(),
        top_speed: 200,
        color: CarColor::Red,
    };
    std::println!("{:?}", car1.color);
    car1.paint(CarColor::Black);
    std::println!("{:?}", car1.color);

    let mut car2 = Car {
        no_of_seats: 4,
        engine: "v6".to_string(),
        top_speed: 200,
        color: CarColor::default(),
    };
    std::println!("{:?}", car2.color);
    car2.color = CarColor::Black;
    std::println!("{:?}", car2.color);

    let mut car3 = Car::create_new_car();
    std::println!("{}", car3.engine);
    car3.paint(CarColor::Black);
}
