
pub fn iter_array() {
    let arr1 = [2,3,4,5,6,22];
    std::println!("{:?}",arr1.iter().all(|&x| x > 10));

    // if .iter is used it is typically used to get iterateable slice of the array
    let iterate = arr1.iter();

    for i in iterate {
        std::println!("{}",i);
    }
}

pub fn iter_vec() {
    let mut vec_arr = vec!["Straberry"];
    vec_arr.push("Blueberry");
    vec_arr.push("Mango");
    vec_arr.push("Apple");
    vec_arr.push("Banana");

    let vec_arr2 = vec!["Almonds","Pista","Kaju","Kishmis"];

    let fruit_nut_list = vec_arr2.iter().chain(&vec_arr);
    std::println!("{:?}",fruit_nut_list);

    // for i in fruit_nut_list {
    //     std::println!("{}", i);
    // }

    let new_final_list: Vec<&&str> = fruit_nut_list.clone().collect(); // this is helpful to concat different vec! into 1 
    std::println!("{:?}", new_final_list);

    let iterate_value = vec_arr.iter().next();

    std::println!("{}", iterate_value.is_some());
    std::println!("{:?}",vec_arr.iter());
}