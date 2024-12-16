#![allow(dead_code)]

struct Person {
    first_name: String,
}

pub fn test_scoping_threads() {
    let age = 23;
    let p1 = Person {
        first_name: "Pranav".to_string(),
    };

    // here as u can see the variable age is defined in the parent scope and to access it in the closure scope we
    // need to borrow it and to acheive it we use the move keyword that helps in borrowing the variable from the
    // parent scope to the child scope -> this closure has to take ownership when used to spawn a thread
    // let scope_cloure = move || {
    let scope_closure = || {
        // here we are not moving the ownership
        std::println!("The returned value for age is the  {age}");
        std::println!("Your name is the {0}", &p1.first_name);
    };

    let scope_closure2 = || {
        // here we are not moving the ownership
        std::println!("The returned value for age is {age}");
        std::println!("Your name is {0}", &p1.first_name);
        for i in 1..10 {
            std::println!("{i}")
        }
    };

    // prefiexed the var name with "_" bcoz the rust compiler knows that value will not be used and treated like dead_code
    // let _result = std::thread::spawn(scope_closure).join();

    // now to better deal with the borrowing model and to get since when we moved the ownership by borrowing for a stack
    // varaible is easier than that of the heap variable so we will be using a "scope" thread for it
    std::thread::scope(|scope| {
        scope.spawn(scope_closure); // this thread created here is given the lifetime of this call in the main thread
        scope.spawn(scope_closure2);
    });
}
