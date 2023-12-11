use axum::extract::{FromRef, Path, State};
use axum::routing::{delete, post};
use axum::{Json, Router};

use crate::model::{ModelController, Ticket, TicketForCreate};
use crate::Result;

// #[derive(Clone, FromRef)]
// struct AppState {
//     mc: ModelController,
// }

pub fn routes(mc: ModelController) -> Router {
    // let app_state = AppState { mc };

    Router::new()
        .route("/tickets", post(create_ticket).get(list_ticket))
        .route("/tickets/:id", delete(delete_ticket))
        // .with_state(app_state)
        .with_state(mc)
}

// region: --- REST handlers
async fn create_ticket(
    State(mc): State<ModelController>,
    Json(ticket_fc): Json<TicketForCreate>,
) -> Result<Json<Ticket>> {
    println!("->> {:<15} - create_ticket", "HANDLER");

    let ticket = mc.create_ticket(ticket_fc).await?;

    Ok(Json(ticket))
}

async fn list_ticket(State(mc): State<ModelController>) -> Result<Json<Vec<Ticket>>> {
    println!("->> {:<15} - list_ticket", "HANDLER");

    let tickets = mc.list_tickets().await?;

    Ok(Json(tickets))
}

async fn delete_ticket(
    State(mc): State<ModelController>,
    Path(id): Path<u64>,
) -> Result<Json<Ticket>> {
    println!("->> {:<15} - delete_ticket", "HANDLER");

    let ticket = mc.delete_ticket(id).await?;

    Ok(Json(ticket))
}
// endregion: --- REST handlers