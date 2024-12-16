// structs and debug using #[derive(Debug)]
// also covers about struct_tuple (short hands struct)
// how to make a struct mutable
// about struct methods (Impl for a custom data type)

mod test_mut_structs;
mod test_structs;
mod test_structs_methods;

fn main() {
    std::println!("Hello, world!");
    test_structs::test_basic_struct_set_up();
    test_structs::test_basic_struct_user_input();

    // here i am create a new struct using the variable in the other file
    // and the refer the struct in the other file to learn about to make it visible everywhere
    let new_car = test_structs::Car {
        no_of_seats: 2,
        engine: "Turbo_Hybrid".to_string(),
        top_speed: 300,
        color: test_structs::CarColor::Black,
    };
    std::println!("The Top speed of the car is {0} kmph", new_car.top_speed);

    let tuple_car = test_structs::create_car_tuple();
    std::println!("{:#?}", tuple_car);
    std::println!("{:?}", tuple_car.0);
    std::println!("{}", tuple_car.2);

    test_mut_structs::create_person();
    test_mut_structs::create_new_person_mut();

    let person1 = test_mut_structs::create_new_person_non_mut();
    // person1.first_name = Cell::set(&self,"value");
    std::println!("{}", person1.first_name.get());

    test_structs_methods::run_impl();
}
