fn main() {
    // test_if();
    // test_while();
    // test_loop();
    test_for();
}

fn test_for() {
    let ages: [u32; 8] = [12, 34, 56, 78, 90, 65, 32, 9];

    let age_to_drive = 16u32;

    let mut count = 0u8;

    // here we are directly iteration over the array and unlike the counter i, i stands for ages[i] value
    for i in ages {
        if i > age_to_drive {
            count += 1;
        }
    }

    println!("{:?}", ages);

    std::println!("The eligible people are {0}", count);
}

#[allow(dead_code)]
fn test_loop() {
    // the difference w.r.t to while is that it doesn't expect a conditional rather just a {}
    // this is more like the  `while true {}` case
    loop {
        // it must be explicitely broken else leads to an infinite loop
        std::println!("I am Rustecian: 🦀🦀")
    }
}

#[allow(dead_code)]
fn test_while() {
    let age_to_drive = 16u8;

    let mut current_age = 0u8;

    // here as usual the while loop expects a condition
    while current_age < age_to_drive {
        std::println!("Waiting for the chance to drive!!");
        current_age += 1;
        if current_age == 8 {
            break;
        }
    }
}

#[allow(dead_code)]
fn test_if() {
    let age_to_drive = 16u8;

    std::println!("Enter the age of the person");

    let person_name_input = &mut String::from("");

    std::io::stdin()
        .read_line(person_name_input) // when u use this it add the new_line like ln at end
        .unwrap(); //the read_line returns a type result but we override that using unwrap that we dont mind return value

    // here we are not dealing with new_line generated by the io
    // let age = person_name_input.parse::<u8>().unwrap();

    // this function here deals with the whitespaces at the end and beginning make the input parse-able
    // the parse helps in type conversion and here we are explicitely saying to convert to u8 type
    let age = person_name_input.trim().parse::<u8>().unwrap();

    if age > age_to_drive {
        std::println!("The person is old enough to drive!!!!")
    } else if age == age_to_drive {
        std::println!("You are on the verge wait another year")
    } else {
        std::println!("You are not Old Enough Sorry :( ")
    }
}
