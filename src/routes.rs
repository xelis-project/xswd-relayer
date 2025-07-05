
use actix_web::web::Payload;
use actix_web::{get, web, HttpRequest, HttpResponse, Responder};
use uuid::Uuid;

use crate::relayer::Relayer;
use crate::session::RelayerSession;

/// Upgrade the connection to a WebSocket
/// And send directly the channel id to be shared to the client
#[get("/ws")]
async fn create_channel(relayer: web::Data<Relayer>, request: HttpRequest, body: Payload) -> Result<impl Responder, actix_web::Error> {
    let (response, session, stream) = actix_ws::handle(&request, body)?;

    let relayer = relayer.into_inner();
    let session = RelayerSession::new(session, relayer.clone());

    if let Err(e) = relayer.create_new_channel(session, stream).await {
        return Ok(HttpResponse::InternalServerError().body(e.to_string()))
    }

    Ok(response)
}

#[get("/ws/{id}")]
async fn join_channel(
    relayer: web::Data<Relayer>,
    request: HttpRequest,
    body: Payload,
    path: web::Path<Uuid>,
) -> Result<impl Responder, actix_web::Error> {
    let id = path.into_inner();
    if !relayer.has_channel(&id) {
        return Ok(HttpResponse::NotFound().body("Channel not found"))
    }

    let (response, session, stream) = actix_ws::handle(&request, body)?;
    let relayer = relayer.into_inner();
    let session = RelayerSession::new(session, relayer.clone());

    if let Err(e) = relayer.join_channel(id, session, stream).await {
        return Ok(HttpResponse::InternalServerError().body(e.to_string()))
    }

    Ok(response)
}
