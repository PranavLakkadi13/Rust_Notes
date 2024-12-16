fn main() {
    println!("Hello, world!");
    test_func();
    test_func_int();
    test_func_unsigned_int();
    loop_();
    test_type_cast();
    test_bool();
    test_string_types();
    test_tuple();
    test_array()
}

fn test_func() {
    // this is a unique type in rust called the unit type (its like null/none)
    let x: () = ();
    println!("{:?}", x);
}

fn test_func_int() {
    let x = -22132;
    print!("The {} is a type float", x);
}

fn test_func_unsigned_int() {
    let x: u32 = 32;
    print!("The {} is a type float", x);
}

fn loop_() {
    // the compiler can catch only compiler time error runtime errors still difficult
    // let mut x: u32 = 4294967295;
    for i in 0..12 {
        println!("{}", i);
        // x+=i;
    }
    // println!("{}",x);
}

fn test_type_cast() {
    let y: f32 = 22.221;
    let x: u16 = ((y as u8) + 5) as u16;
    println!("The value of x {}", x);
}

fn test_bool() {
    let booltest: bool = true;
    // gives error bcoz the variable is not mutable
    // booltest = false;
    println!("{}", booltest);
}

fn test_string_types() {
    // type is char and can be only 1 char length
    // also meant that is 4 bit long
    let charvar = 'A';
    println!("{}", charvar);

    // this s of type &str
    let strvar = "A";
    println!("{}", strvar);

    // this is for the mutablity of the string
    let mut first_name = "Pranav";
    println!("{}", first_name);
    first_name = "lakkadi";
    println!("{}", first_name);
}

fn test_tuple() {
    // tuple can store different data types unlike arrays and start with curve bracket
    let tuplevar = ("Pranav", "Reddy", 21 as u8);

    println!("{}", tuplevar.0);

    // println!("{}", tuplevar); //this gives an error because output is of 1 var but tuple has multiple
    println!("{:?}", tuplevar); // this gets printed as the ":?" uses debugger and prints the variable accordingly
}

fn test_array() {
    // array takes variables of the same type ;
    let arrayvar = [12, 23, 432, 45, 345];

    println!("{:?}", arrayvar);

    println!("{}", arrayvar[0]);

    // the variable sliced array is basically an array that borrows its values from the owner but cant on
    let /* mut */ sliced_array = &arrayvar[2..4];
    println!("{:?}", sliced_array);

    // this is an error since the varaible uses another variables reference so it doesn't own it
    // even though it was mutable it can write to the variables
    // sliced_array[1] = 23;
}
