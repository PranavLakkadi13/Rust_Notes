#![allow(dead_code)]

use std::thread;

pub fn public_call_explore() {
    let x = thread::spawn(|| {
        println!("Hello people");
        for i in 0..20 {
            std::println!("{0}", i);
        }
    });
    let z = thread::spawn(|| {
        println!("Hello people");
        for i in 0..20 {
            std::println!("{0}", i);
        }
    });
    let zz = thread::spawn(|| {
        println!("Hello people");
        for i in 0..20 {
            std::println!("{0}", i);
        }
    });
    let xx = thread::spawn(|| {
        println!("Hello people");
        for i in 0..20 {
            std::println!("{0}", i);
        }
    });
    let y = x.thread();
    let w = z.thread();
    let q = zz.thread();
    let e = xx.thread();
    std::println!("{:#?} {:#?} {:#?} {:#?}", y, w, q, e);
}

pub fn test_my_thread() {
    let mut x = 0u128;
    for i in 0..500_000_000 {
        x += i;
        std::println!("{0}", x);
    }
}

pub fn spawing_my_thread() {
    let thread_closure = || {
        let mut x = 0u128;
        for i in 0..500_000_000 {
            x += i;
        }
        std::println!("the value of x = {0}", x);
    };

    // so here the main thread assignes tasks to the worker thread to be executed but as soon as the main thread is
    // done executing the worker threads job will be pre-emptied
    std::println!("New Thread has been spawned!!");
    let handler = thread::spawn(thread_closure); // when spawning a new thread it returns a handle
    let handler2 = thread::spawn(thread_closure); // when spawning a new thread it returns a handle
    std::println!("The thread has been assigned work..");

    test_my_thread();

    // you can use this approach to manage the scope of the main thread instead of just using the join handle
    loop {
        if handler.is_finished() && handler2.is_finished() {
            std::println!("The main thread and the worker threads are done executing.....");
            break;
        }
    }
}
