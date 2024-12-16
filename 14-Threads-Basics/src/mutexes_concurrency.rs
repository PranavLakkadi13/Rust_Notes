#![allow(dead_code)]

use std::ops::AddAssign;

// mutexes are a method make a a piece of data or resource mutually exculsive when trying to do multithreading
// this is similar to the concept in OS when mutex is a resource that can only be accessed by 1 thread at a time
pub fn mutex_share() {
    let score = std::sync::Mutex::new(21u16);
    let unlocked_score = score.lock();
    let mut unwrapped_data = unlocked_score.unwrap();
    unwrapped_data.add_assign(33);

    // the above value are all in scope and are held by the process/thread and the shareable resource score cannot
    // be shared since it is still held by the main process and to release it so that other processes can use it is
    // it has to make sure the MutexGuard<> has to go out of scope and as you know it is a Cutom type u need to
    // drop it from heap

    drop(unwrapped_data);
}

pub fn multi_thread() {
    let score = std::sync::Mutex::new(0u32);

    let my_closure = || {
        std::println!("closure1 waiting to get the access to the shared resource... lock ");
        let mut data = score.lock().unwrap();
        for i in 1..10 {
            data.add_assign(i);
            std::println!("{} is the value being added to resource in closure 1", i);
        }
    };

    // we are doing this again to use it for the multi-threading process
    let my_closure2 = || {
        std::println!("closure2 waiting to get the access to the shared resource... lock ");

        #[allow(unused_mut)]
        let mut data = score.lock().unwrap();
        // if u know its gonna throw error its better to handle it in a way it drops the mutex lock so others can use
        drop(data);
        std::panic!("Hellow"); // wanted introduced a panic to handle it

        #[allow(unreachable_code)]
        for i in 1..10 {
            data.add_assign(i);
            std::println!("{} is the value being added to resource in closure 2", i);
        }
    };

    _ = std::thread::scope(|scope| {
        // here though in multi-threading as you know if a error occurs it should just effect 1 core rather than the
        // system so we need a method to handle the possible errors that can occur
        // but in scope function if a error occus it crashes the whole system so need to handle the error
        _ = scope.spawn(my_closure).join();
        let handle2 = scope.spawn(my_closure2).join();
        if handle2.is_err() {
            std::println!("It is being handled here!!");
        }
    });

    std::println!("{}", score.lock().unwrap());
}

// the main aim of the code is to spawn 2 treads and then see the which thread accesses the shared resource first
// and the other thread which instead of waiting rather sleep the thread until the resource is free and when free
// the thread will be spawned
pub fn multi_thread_sleep_when_notreceived_mutex() {
    let score = std::sync::Mutex::new(0u32);

    let my_closure = || {
        std::println!("closure1 waiting to get the access to the shared resource... lock ");
        let mut data = score.lock().unwrap();
        for i in 1..=10 {
            data.add_assign(i);
            std::println!("{} is the value being added to resource in closure 1", i);
        }
    };

    // we are doing this again to use it for the multi-threading process
    let my_closure2 = || loop {
        std::println!("closure2 waiting to get the access to the shared resource... lock ");
        let guard = score.lock();
        if guard.is_ok() {
            let mut data = guard.unwrap();
            for i in 1..=10 {
                data.add_assign(i);
                std::println!("{} is the value being added to resource in closure 2", i);
            }
            break;
        }
        let time = std::time::Duration::from_secs_f32(100.0);
        std::thread::sleep(time);
        println!("Gotta wait ")
    };

    _ = std::thread::scope(|scope| {
        scope.spawn(my_closure);
        scope.spawn(my_closure2);
        // removing this error handing as i am trying to handle the error and save resources in the closure itself
        // let handle2 = scope.spawn(my_closure2).join();
        // if handle2.is_err() {
        //     std::println!("It is being handled here!!");
        // }
    });

    std::println!("{}", score.lock().unwrap());
}
