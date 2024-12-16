#![allow(dead_code)]

use futures::join;

fn main() {
    let returned_value1 = smol::block_on(num1());
    let returned_value2 = smol::block_on(num2());
    let returned_value3 = smol::block_on(num3());

    let result = smol::block_on(
        async {
            join!(returned_value1, returned_value2, returned_value3);
        }
    );
}

async fn num1() -> u8 {
    std::println!("The future is never read!!");
    return 121;
}

async fn num2() -> u8 {
    std::thread::sleep(std::time::Duration::from_millis(20000));
    return 10
}

async fn num3() -> u8 {
    std::thread::sleep(std::time::Duration::from_millis(200));
    return 121
}
