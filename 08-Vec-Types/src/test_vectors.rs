use std::{ops::Mul, vec};

pub fn test_vec_int() {
    let mut arr: Vec<u32> = Vec::new();
    arr.push(22);

    let mut x = 10u32;
    while arr.len() < 10 {
        arr.push(x.mul(x));
        x += 1;
    }

    let indexedvalue = arr.get(30);
    std::println!("{}", indexedvalue.is_some()); // this returns a bool

    std::println!("{}", arr.capacity()); //this is to give the memory allocated for the array
    std::println!("{:?}", arr);

    std::println!("{:?}", &(&arr).as_slice()[2..=4]); // here we are slicing the array so need to know the location
}

pub fn test_vec_strings() {
    let mut str_arr = vec!["Hello"];
    str_arr.push("value");

    // if this is run it will cause issues as the value is being and based on the ownership model since the varible
    // after the use through iteration will be deallocated from memory
    // for i in str_arr {
    for i in str_arr.as_slice() {
        // here we are created a copy to be iterated
        std::println!("{} is the name being processed.....", i);
    }

    std::println!("{:?}", str_arr);
}

#[allow(dead_code)]
#[derive(Debug)]
struct Car {
    manufacturer: String,
    model: String,
}

pub fn car_vec() {
    // here using the below logic we create a vec of size 10 and all the indexes will be filled with the value passed
    // let arr_cars = vec!["Pranav"; 10];  --> done below for custom data types

    let mut arr_cars = Vec::new();

    let mut arr_cars2 = Vec::new();

    arr_cars2.push(Car {
        manufacturer: "Mercedes".to_string(),
        model: "AMG".to_string(),
    });

    for _ in 1..100u8 {
        arr_cars.push(Car {
            manufacturer: "BMW".to_string(),
            model: "x6".to_string(),
        });
    }

    for _ in 1..100u8 {
        arr_cars2.push(Car {
            manufacturer: "Mercedes".to_string(),
            model: "AMG".to_string(),
        });
    }

    std::println!("{:?}", arr_cars);
    std::println!("{:?}", arr_cars2);

    // this to append others arrays data into to current array and should be a pointer address and it copies the data
    arr_cars.append(&mut arr_cars2);
    std::println!("{:?}", arr_cars);
    std::println!("{:?}", arr_cars2); // the values are wiped out since the data was copied to the other vec

    arr_cars.insert(
        0,
        Car {
            manufacturer: "Audi".to_string(),
            model: "A8".to_string(),
        },
    );

    std::println!("{:?}", arr_cars);

    // he we will use the retain method and as teh name suggests it will retain the values based on the condition
    // retain is a vec methods that takes arguments as an anonymous function and returns a value based on the return type
    let keep = |car: &Car| {
        if car.manufacturer == "Audi".to_string() {
            return true;
        } else {
            return false;
        }
    };
    arr_cars.retain(keep);
    std::println!("{:?}", arr_cars);
    std::println!("{:?}", arr_cars.len());

    // this is method is used to additional reserve space for the vec data
    std::println!("{:?}", arr_cars.capacity());
    arr_cars.reserve(1000);
    std::println!("{:?}", arr_cars.capacity());
}
