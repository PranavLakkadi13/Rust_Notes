use axum::routing::{get, post};
use axum::{Router, serve};
use tokio::net::TcpListener;

mod vehicle;
use vehicle::{get_vehicle, post_vehicle, post_vehicle_query};

use crate::vehicle::post_vehicle_and_customer;

#[tokio::main]
async fn main() {
    let router1 = Router::new()
        // this allows the same route have 2 different route handlers (without needing to import)
        // since it works directly on the return type of the get handler
        .route("/vehicle", get(get_vehicle).post(post_vehicle))
        .route("/vehicle_query", post(post_vehicle_query))
        .route("/vehicle_customer", post(post_vehicle_and_customer));

    let address = "0.0.0.0:3000";
    let listener = TcpListener::bind(address).await.unwrap();
    serve(listener, router1).await.unwrap();
}
