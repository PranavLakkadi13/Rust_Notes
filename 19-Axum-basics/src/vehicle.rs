use axum::{Json, extract::Query};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Vehicle {
    pub manufacturer: String,
    pub model: String,
    pub year: u16,
    pub id: Option<String>,
}

pub async fn get_vehicle() -> Json<Vehicle> {
    println!("the get vehicle route hit!!!");
    Vehicle {
        manufacturer: "Dodge".to_string(),
        model: "RAM 1500".to_string(),
        year: 2021,
        id: Some(uuid::Uuid::new_v4().to_string()),
    }
    .into()
}

/**
 * http://localhost:3000/vehicle
 *
 * body = {
    "manufacturer": "Dodge",
    "model": "RAM 1500",
    "year": 2021,
    "id": "ecbf9c86-81fc-4dd7-97ea-5c441fbd4a90"
}
 */
pub async fn post_vehicle(Json(mut user_vehicle): Json<Vehicle>) -> Json<Vehicle> {
    println!("the post vehicle route hit!!!");
    println!(
        "the user vehicle is manufactured by {:?}, model {:?}, in the year {:?}",
        user_vehicle.manufacturer, user_vehicle.model, user_vehicle.year
    );
    user_vehicle.id = Some(uuid::Uuid::new_v4().to_string());
    Json::from(user_vehicle)
}

// trying to pass struct as a query
// http://localhost:3000/vehicle_query?manufacturer=bmw&model=x1&year=2025
pub async fn post_vehicle_query(Query(mut user_vehicle): Query<Vehicle>) -> Json<Vehicle> {
    println!("hit the query route!!!");
    println!(
        "the user vehicle is manufactured by {:?}, model {:?}, in the year {:?}",
        user_vehicle.manufacturer, user_vehicle.model, user_vehicle.year
    );
    user_vehicle.id = Some(uuid::Uuid::new_v4().to_string());
    Json::from(user_vehicle)
}

#[derive(Deserialize)]
pub struct Customer {
    pub first_name: String,
    pub last_name: String,
}

#[derive(Serialize)]
pub struct VehicleOwner {
    pub manufacturer: String,
    pub model: String,
    pub year: u16,
    pub id: Option<String>,
    pub first_name: String,
    pub last_name: String,
}

// here u can see i passed 2 structs and deserialised them seperately
// http://localhost:3000/vehicle_customer?manufacturer=bmw&model=x1&year=2025&first_name=Alice&last_name=bob
pub async fn post_vehicle_and_customer(
    Query(user_vehicle): Query<Vehicle>,
    Query(user): Query<Customer>,
) -> Json<VehicleOwner> {
    println!("hit the query and cutomer route hit!!!");
    println!(
        "The user first_name: {:?}, last name is {:?}",
        user.first_name, user.last_name
    );
    println!(
        "the user vehicle is manufactured by {:?}, model {:?}, in the year {:?}",
        user_vehicle.manufacturer, user_vehicle.model, user_vehicle.year
    );
    Json::from(VehicleOwner {
        manufacturer: user_vehicle.manufacturer,
        model: user_vehicle.model,
        year: user_vehicle.year,
        id: Some(uuid::Uuid::new_v4().to_string()),
        first_name: user.first_name,
        last_name: user.last_name,
    })
}
