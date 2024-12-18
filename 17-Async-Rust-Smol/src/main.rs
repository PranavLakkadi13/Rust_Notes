#![allow(dead_code)]

use futures::future::FutureExt;
use futures::{join, pin_mut, select};

fn main() {
    // let returned_value1 = smol::block_on(num1());
    // let returned_value2 = smol::block_on(num2());
    // let returned_value3 = smol::block_on(num3());

    // let returned_value1 = num1(); // these are output of the async functions (in Futures) when using join! macro
    // let returned_value2 = num2();
    // let returned_value3 = num3();

    let returned_value1 = num1().fuse(); // when using the select statement
    let returned_value2 = num2().fuse();
    let returned_value3 = num3().fuse();

    let _ = smol::block_on(async {
    //  here the join is basically used to join and make all the async call once and join the results into a tuple()
            join!(num1(), num2(), num3())
        }
    );

    pin_mut!(returned_value1, returned_value2, returned_value3);

    let results = smol::block_on(async {
        // this is more like a switch statment to handle async calls 
        loop {
            select! {
                _ = returned_value1 => std::println!("Hello"),
                _ = returned_value2 => std::println!("Hello"),
                _ = returned_value3 => std::println!("Hello"),
            complete => break,
            default => std::println!("Hello"),
            };
        }
    });

    std::println!("{:#?} is the final value ", results);
}

async fn num1() -> u8 {
    std::println!("The future is never read!!");
    std::println!("{:#?} is the current time", std::time::Instant::now());
    return 69;
}

async fn num2() -> u8 {
    std::thread::sleep(std::time::Duration::from_millis(2000));
    std::println!("num 2 executed now at {:#?}", std::time::Instant::now());
    return 10;
}

async fn num3() -> u8 {
    std::thread::sleep(std::time::Duration::from_millis(200));
    std::println!(
        "The num3 fn is executed now at {:#?}",
        std::time::Instant::now()
    );
    return 221;
}
