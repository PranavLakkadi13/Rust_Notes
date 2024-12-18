#![allow(dead_code)]

use std::process::Output;

// the flavor is used to say if the main function is used in a single threaded app or multi threaded
// by default it is mult threaded , -> in flavor u can try "worker_thread" to choose number of threads
// #[tokio::main(flavor = "current_thread")]
#[tokio::main]
async fn main() {
    let x = F1racer::new();
}

struct F1racer {
    name: String,
    completed_laps: u8,
    no_laps: u8,
    best_lap_time: u8,
    lap_times: Vec<u8>
}

impl F1racer {
    fn new() -> F1racer {
        return F1racer {
            name: "Lewis Hamilton".to_string(),
            completed_laps: 0,
            no_laps: 5,
            best_lap_time: 255,
            lap_times: vec![111,82,99,34,221]
        };
    }
}

impl std::future::Future for F1racer {
    // the expected output of the resolved future 
    type Output = u8;

    // this is the function the runtime executor calls continously untill a result is returned
    // here Poll if of type ENUM that has 2 values Ready and Pending 
    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        std::task::Poll::Ready(self.best_lap_time)
    }
}
