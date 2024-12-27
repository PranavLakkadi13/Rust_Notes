use axum::{
    extract::{Path, Query},
    response::{Html, IntoResponse},
    routing::{get, get_service},
    Router,
};
use serde::Deserialize;
use std::net::SocketAddr;
use tower_http::services::ServeDir;
mod error;

#[tokio::main]
async fn main() {
    // here usually the code uses a handler in the get request instead of the writing the async code
    // the below fn was written to dynamically modularise the code
    let routes_all = Router::new()
        .merge(router_hello()) // this is the routes its gonna take first
        .fallback_service(route_static()); // but if no route is found it hits the fallback static route

    // the region where we deal with the server like giving the ip address and port number
    // Start Region ---> Start Server
    // here it was used to set the port and ip_address but was using older version
    let addr = SocketAddr::from(([127, 0, 0, 80], 8080));
    std::println!("The server Listening on port {:?}", &addr);
    // ere it uses the newer version and the bind takes in &str input of the same ip_address and port
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();
    axum::serve(listener, routes_all.into_make_service())
        .await
        .unwrap();
    // End Region --> End Server
}

fn router_hello() -> Router {
    // Router::new().route("/hello", get(|| async { Html("<h1> Hello world </h1> ") }));
    Router::new()
        .route("/hello", get(handler_hello))
        // here you can see that we already assign the variable to name that will be passed in the test as route
        .route("/hello2/:name", get(handler_hello2))
}

// this is for the static file routing.....
fn route_static() -> Router { // currently the code here is going to get the route of the passed fallback path 
    // here the code has been given the access to the root of the dir and the path shared in call will be placed
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}
// this example is when the http://localhost:8080/hello?name=pranav -> here we pass the argument var and assign
// here the in the above line we can see that the value is assigned to a variable
// here as you can see the code is returning the IntoResponse type
// the QueryType is used to take in Query filter from the user and then deserialise it using serde
async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    std::println!("--> {:<32} Hello   {params:?}", "Handler"); // this is recorded in the server side code (logs)
    let name = params.name.as_deref().unwrap_or("World");
    Html(format!("Hello <strong> World Peeps.... {name} </strong>"))
}

// /hello2?Pranav
// how to get the code to work like above since the value is directly passed instead of a query param filter
async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
    std::println!("->> {:<32} Hello   {name}", "Handler"); // this is recorded in the server side code (logs)
    Html(format!("Hello <strong> World Peeps.... {name} </strong>"))
}
