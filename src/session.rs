use std::sync::Arc;

use actix_ws::{CloseReason, Session};
use tokio::{sync::{oneshot, Mutex}, time::timeout};
use uuid::Uuid;

use crate::{error::RelayerError, relayer::RelayerShared};

pub struct RelayerSession {
    id: Uuid,
    server: RelayerShared,
    inner: Mutex<Option<Session>>,
    notify: Mutex<Option<oneshot::Sender<()>>>,
}

pub type RelayerSessionShared = Arc<RelayerSession>;

impl RelayerSession {
    pub fn new(session: Session, server: RelayerShared) -> RelayerSessionShared {
        Arc::new(Self {
            id: Uuid::new_v4(),
            inner: Mutex::new(Some(session)),
            server,
            notify: Mutex::new(None),
        })
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    // Send a ping message to the session
    // this must be called from the task handling the session only
    pub async fn ping(&self) -> Result<(), RelayerError> {
        let mut inner = self.inner.lock().await;
        let session = inner.as_mut()
            .ok_or(RelayerError::Closed)?;

        timeout(self.server.session_message_timeout(), session.ping(b"")).await??;
        Ok(())
    }

    // Send a pong message to the session
    // this must be called from the task handling the session only
    pub async fn pong(&self) -> Result<(), RelayerError> {
        let mut inner = self.inner.lock().await;
        let session = inner.as_mut()
            .ok_or(RelayerError::Closed)?;

        timeout(self.server.session_message_timeout(), session.pong(b"")).await??;
        Ok(())
    }

    // this must be called from the task handling the session only
    pub async fn text<S: ToString>(&self, value: S) -> Result<(), RelayerError> {
        let mut inner = self.inner.lock().await;
        let session = inner.as_mut()
            .ok_or(RelayerError::Closed)?;

        timeout(self.server.session_message_timeout(), session.text(value.to_string())).await??;
        Ok(())
    }

    // Close the session
    pub async fn close(&self) -> Result<(), RelayerError> {
        let mut notify = self.notify.lock().await;
        let sender = notify.take()
            .ok_or(RelayerError::SessionNotify)?;

        sender.send(()).map_err(|_| RelayerError::ChannelUnavailable)
    }

    pub async fn close_internal(&self, reason: Option<CloseReason>) -> Result<(), RelayerError> {
        let mut inner = self.inner.lock().await;
        let session = inner.take()
            .ok_or(RelayerError::Closed)?;

        session.close(reason).await.map_err(|_| RelayerError::Closed)
    }

    pub async fn set_notify(&self, sender: oneshot::Sender<()>) -> Result<(), RelayerError> {
        let mut notify = self.notify.lock().await;
        if notify.is_some() {
            return Err(RelayerError::SessionNotify)
        }

        *notify = Some(sender);

        Ok(())
    }
}