use core::str;

pub struct Person {
    first_name: String,
    last_name: String,
}

pub fn test_closures() {
    // closures in rust are like annonymous functions
    // if there is | pipe symbol when no comparision is made it's very likely that it's a closure
    // the || is used to take arguments as input just like a normal function with ()
    let add = || std::println!("This is my first closure");
    add();

    // eg for a parametrised closure is
    // example on how a closure takes in parameters and how to use a closure
    // here u can also see that no need to define a type of the variable in the closure it is infered
    // based on the argumnet passed to it
    let sum = |x| std::println!("The value entered is {0}", x + 8);

    let user_input = &mut String::from("");

    std::io::stdin().read_line(user_input).unwrap();

    let input = user_input.trim().parse::<i8>().unwrap();
    sum(input);

    // also a fun fact is that the when declaring a closure you dont have keep any return type
    // also the types for the arguments inputs need not be given just like the return type
    let no_return_closure = |x, y| {
        // because this is the last value of the closure u should not keep ';'
        x + y
    };
    let returned_val = no_return_closure(2, 4);

    // here we can see that a closure can read the variables of the parent scope
    // i.e the variable declared in the `pub fn test_closures()` can be used internally in closure
    // its like the parent class variables are inherited into the closure to be accessed
    let print_result = || std::println!("The result is {0}", returned_val);
    print_result();

    // now to understand how the variables in parent scope can borrowed and mutated eg below
    let mut p1 = Person {
        first_name: "Pranav".to_string(),
        last_name: "Reddy".to_string(),
    };

    // here this lines gives you an error because when u are mutating a mutable variable using a closure
    // the closure itself has to also be mutable
    // let change_name = |x:&str| { p1.last_name = x.to_string()};
    // change_name("lakkadi");

    let mut change_name = |x: &str| {
        // here the variable is being modified so as per core rules the variable has to be owned to be modified
        // here the closure has to borrow the ownership of mutable variable to modify it
        p1.last_name = x.to_string()
    };

    let user_input_str = &mut String::new();
    std::io::stdin().read_line(user_input_str).unwrap();
    let input_str = user_input_str.as_str();

    change_name(input_str);
    // here the variable is still owned so in the future by the closure so it cant be taken by parent
    // only after the closure has completely done with its borrowing even in the parent scope
    // the parent scope variable cant be accessed

    // std::println!("{0} {1}", p1.first_name, p1.last_name);
    // change_name("lakkadi");

    std::println!("{0} {1}", p1.first_name, p1.last_name);
}
