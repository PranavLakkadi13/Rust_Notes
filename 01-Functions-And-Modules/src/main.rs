// this is called a module more like a local import from a different file (mod (file_name))
// this is module internal functions are by default private and need to be marked pub explicitly
mod helpers;

fn main() {
    println!("Hello, world!");
    let fullname:String = helpers::namehelpers::get_first_last_name("pranav", "reddy");
    println!("{0}", fullname);
    let age = helpers::agefunction::get_age_and_add_5(22);
    println!("the age of the person {0}", age)
}

// this warning attribute applys directly on the next line of usage (outer attribute)
#[allow(dead_code)] // this attribute is used to tell the compiler that ik its not used and dont complain 
fn dead_code() -> u8 {
    let x = 43 as u8;
    return x;
}