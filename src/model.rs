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
#[derive(Clone)] // --> only clone the ARC
pub struct ModelController {
    // we will you the vector number as the ticket id --> works for mock but not in production
    tickets_store: Arc<Mutex<Vec<Option<Ticket>>>>,
}

// Constructor
impl ModelController {
    pub async fn new() -> Result<Self> {
        std::result::Result::Ok(Self {
            tickets_store: Arc::default(),
        })
    }
}

// CRUD imp[lementtion
impl ModelController {
    pub async fn create_ticket(&self, ticket_fc: TicketForCreate) -> Result<Ticket> {
        let mut store = self.tickets_store.lock().ok().unwrap();

        if ticket_fc.title.is_empty() {
            return Err(Error::TicketCreateFailedEmptyTitle);
        }

        let id = store.len() as i64;
        let ticket = Ticket {
            id: id,
            title: ticket_fc.title,
        };
        store.push(Some(ticket.clone()));

        std::result::Result::Ok(ticket)
    }

    pub async fn list_tickets(&self) -> Result<Vec<Ticket>> {
        let store = self.tickets_store.lock().ok().unwrap();

        let tickets = store.iter().filter_map(|t| t.clone()).collect();

        std::result::Result::Ok(tickets)
    }

    pub async fn delete_ticket(&self, id: u64) -> Result<Ticket> {
        let mut store = self.tickets_store.lock().ok().unwrap();

        // here we are geting a returned ticket in Option
        // first we get the store mutex var as a mut ref at the passed index
        // the .and_then() basically if a value is found takes the value and wraps into an Option else returns a None Type
        let ticket = store.get_mut(id as usize).and_then(|t| t.take());

        // here this step now takes the option and converts to a Result type
        ticket.ok_or(Error::TicketDeleteFailIdNotFound { id: id })
    }
}
