mod test_hashmaps;
mod test_hashset;

fn main() {
    println!("Hello, world!");

    test_hashmaps::creat_hashmap_basic();
    test_hashset::create_impl_hashset();
    test_hashset::names();
}
