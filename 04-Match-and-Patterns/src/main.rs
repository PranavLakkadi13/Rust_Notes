mod match_code;

fn main() {
    println!("Hello, world!");

    match_code::test_match_on_int();
    match_code::test_match_on_int_armguard();
    match_code::test_match_on_string();
    // match_code::test_match_handle_return();
    let x = match_code::test_match_handle_return();
    println!("The value is {}", x);

    match_code::test_match_array();
    match_code::test_match_struct();
}
