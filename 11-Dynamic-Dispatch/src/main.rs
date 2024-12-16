mod test_dyn_traits;

// dynamic dispatch is like dynamically using a interfaces and make function calls based on dynamic instances
fn main() {
    println!("Hello, world!");

    test_dyn_traits::run_test_traits();
}
