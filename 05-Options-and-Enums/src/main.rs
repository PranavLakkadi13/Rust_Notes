mod option_test;

fn main() {
    std::println!("Hello, world!");
    let result = option_test::test_option_type();
    std::println!("{0}", result.unwrap());

    let result_string = option_test::test_option_string();
    std::println!("{0}", result_string.unwrap());

    // u can also use result_string.is_none()||result_string.is_some() to check if any value if returned before or not

    let result_custom_type = option_test::test_option_charactertype(99);
    // here rust doesn't know how to unwrap the custom enum in return type
    // to deal with it we will create a trait to pass the custom enum and convert to string to be displayed
    if result_custom_type.is_some() {
        std::println!("There is some value ");
        std::println!("{}", result_custom_type.unwrap().to_string());
    } else {
        std::println!("The is a null value!!!!");
    }
}
