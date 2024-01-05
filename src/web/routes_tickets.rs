use axum::extract::Path;
use axum::routing::{delete, get};
use axum::Router;
use axum::{extract::State, Json};

use crate::ctx::Ctx;
use crate::model::{ModelController, Ticket, TicketForCreate};
use crate::{Result};

async fn create_ticket(
    State(mc): State<ModelController>,
    ctx: Ctx,
    Json(ticket_fc): Json<TicketForCreate>,
) -> Result<Json<Ticket>> {
    dbg!(">>>>>> CREATE_TICKET HANDLER");

    let ticket = mc.create_ticket(ctx, ticket_fc).await?;

    Ok(Json(ticket))
}

async fn list_tickets(State(mc): State<ModelController>, ctx:Ctx) -> Result<Json<Vec<Ticket>>> {
    dbg!(">>>>>> LIST_TICKETS HANDLER");

    let tickets = mc.list_tickets(ctx).await?;
    Ok(Json(tickets))
}

async fn delete_ticket(
    State(mc): State<ModelController>,
    ctx: Ctx,
    Path(id): Path<u64>,
) -> Result<Json<Ticket>> {
    dbg!(">>>>>> DELETE_TICKET HANDLER");

    let ticket = mc.delete_ticket(ctx, id).await?;
    Ok(Json(ticket))
}

pub fn routes(mc: ModelController) -> Router {
    Router::new()
        .route("/tickets", get(list_tickets).post(create_ticket))
        .route("/tickets/:id", delete(delete_ticket))
        .with_state(mc)
}
