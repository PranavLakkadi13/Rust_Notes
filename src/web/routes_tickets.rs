use crate::model::{ModelController, Ticket, TicketForCreate};
use crate::{Error, Result};
use axum::extract::Path;
use axum::routing::{delete, get, post};
use axum::Router;
use axum::{extract::State, Json};

#[allow(unused_variables, dead_code)] // this is an example of having and dealing with multiple routes
struct AppState {
    mc: ModelController, // this is a sub state u can have many substates
                         // other substates can be using and dealing with s3 and redis
}

// Routes for the api to handle
pub fn routes(mc: ModelController) -> Router {
    // let app_state = AppState{mc}; // -> this is how u construct it and use multiple states
    Router::new()
        .route("/tickets", post(create_ticket).get(list_tickets))
        .route("/tickets/:id", delete(delete_tickets))
        .route("/gettickets", get(list_tickets))
        // .with_state(app_state) this is the case when used with multiple state handlers
        .with_state(mc) // normally used  in a single state used for this example
}

// Region --- REST Handlers
// State is like a extractor but at the application level
async fn create_ticket(
    State(mc): State<ModelController>,
    Json(ticket_fc): Json<TicketForCreate>,
) -> Result<Json<Ticket>> {
    std::println!("->> {:>32} - Create-Ticket", "Handler");

    let ticket = mc.create_ticket(ticket_fc).await?;

    Ok(Json(ticket))
}

async fn list_tickets(State(mc): State<ModelController>) -> Result<Json<Vec<Ticket>>> {
    std::println!("->> {:>32} - Listing-Ticket", "Handler");

    let result_ticket = mc.list_tickets().await?;

    Ok(Json(result_ticket))
}

async fn delete_tickets(
    State(mc): State<ModelController>,
    Path(id): Path<i64>,
) -> Result<Json<Ticket>> {
    std::println!("->> {:>32} - Delete-Ticket", "Handler");

    let return_val = mc.delete_ticket(id as u64).await?;

    Ok(Json(return_val))
}
// End Region --- REST Handlers
