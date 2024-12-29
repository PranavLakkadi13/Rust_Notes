// Simplictic Model Layer
// Mock Model Layer

use crate::{Error, Result};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

// Region --> Ticket Type  -> Basically like a simple CRUD,Todo Task etc
// using the Clone to send a copy to the client and debu to be able to debug the code
#[derive(Debug, Clone, Serialize)]
pub struct Ticket {
    pub id: i64,
    pub title: String,
}

// this is like a payload send to the Create API
#[derive(Deserialize)]
pub struct TicketForCreate {
    pub title: String,
}

// --> You can have more tickets as in for updates and partial updates to be dealt with
// End Region  --> Ticket Type

// --------------------------------------------------------------------------------------------------------

// Region --> Model Controller Type
// this is more of a region to connect the database and SQLX but for now we will use a embedded data in memory
pub struct ModelController {
    // we will you the vector number as the ticket id 
    tickets_store: Arc<Mutex<Vec<Option<Ticket>>>>,
}
