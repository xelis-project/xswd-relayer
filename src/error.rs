use actix_ws::Closed;
use thiserror::Error;
use tokio::time::error::Elapsed;

#[derive(Error, Debug)]
pub enum RelayerError {
    #[error("unexpected message")]
    UnexpectedMessage,
    #[error("notify host")]
    NotifyHost,
    #[error("waiting peer")]
    WaitingPeer,
    #[error("channel unavailable")]
    ChannelUnavailable,
    #[error("channel not found")]
    ChannelNotFound,
    #[error(transparent)]
    DeadlineElapsed(#[from] Elapsed),
    #[error(transparent)]
    SessionClosed(#[from] Closed),
    #[error("relayer session is closed")]
    Closed,
    #[error("session notify channel")]
    SessionNotify,
}
