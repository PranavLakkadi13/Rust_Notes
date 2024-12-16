// MPSC: Multiple Producer Single Consumer -> many handles can produces messages but only 1 thread can consume it
#![allow(dead_code)]

use std::sync::mpsc;

pub fn test() {
    std::println!("Hello ppl from message passing....");

    // the <u8> helps to determine the type of the data that is passed into the channel and also the amount of data
    // to be sent to asynchronous channle is unlimited(based on memory)
    // let (sender, receiver) = mpsc::channel(); // here the type of data sent in channel is generic
    let (sender, receiver) = mpsc::channel::<u16>();

    // drop(receiver); // if the sender/receiver is dropped the entire channel is invalidated

    let is_sent = sender.send(28);
    let _ = sender.send(23);

    if is_sent.is_ok() {
        std::println!("Message sent to channel successfully");
    }

    // if i do receiver.recv() it completly blocks the thread untill the data is received
    // in the case of the recv_timeout(Duration) the receiver just blocks the thread for a certin duration before
    // letting the thread execute different actions
    let y = receiver.recv_timeout(std::time::Duration::from_secs_f32(22.2));
    std::println!("{:?}", y.ok().unwrap());
}

pub fn test_seperate_thread_channel() {
    let (sender, receiver) = mpsc::channel::<u16>();

    let processor_code = move || {
        std::println!("Starting a processor Thread....");
        let mut failed_count = 0u8;
        loop {
            std::println!("The thread channel is actively listening....");
            let msg_receiver = receiver.recv_timeout(std::time::Duration::from_millis(100));
            if msg_receiver.is_ok() {
                std::println!("{} is the value in the channel", msg_receiver.unwrap());
            } else {
                failed_count += 1;
                if failed_count == 10 {
                    std::println!("no active messages being sent so receiver is terminated channel is not active");
                    drop(receiver);
                    break;
                }
            }
        }
    };

    let new_sender = || {
        let mut i = 300u16;
        loop {
            let _ = sender.send(i);
            i += 1;
            if i == 500 {
                break;
            }
        }
    };

    for i in 0..200 {
        let is_sent = sender.send(i);
        let _ = sender.send(i);
        if is_sent.is_ok() {
            std::println!("Message sent to channel successfully");
        }
    }

    std::thread::scope(|scope| {
        scope.spawn(processor_code);
        scope.spawn(new_sender);
    });
}
