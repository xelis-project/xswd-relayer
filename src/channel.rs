use std::{mem, sync::Arc};

use dashmap::DashMap;
use tokio::sync::oneshot;
use uuid::Uuid;

use crate::{error::RelayerError, session::RelayerSessionShared};

#[derive(Default)]
pub enum ChannelState {
    Waiting {
        host: RelayerSessionShared,
        // used to send the Session to the host
        notify: oneshot::Sender<RelayerSessionShared>
    },
    #[default]
    Ready
}

pub struct Channel {
    state: ChannelState,
}

pub type Channels = Arc<DashMap<Uuid, Channel>>;

impl Channel {
    pub fn new(host: RelayerSessionShared, notify: oneshot::Sender<RelayerSessionShared>) -> Self {
        Self {
            state: ChannelState::Waiting { host, notify }
        }
    }

    // Join the channel by notifying the host that we joined it
    // and start handling it
    pub fn join(&mut self, client: &RelayerSessionShared) -> Result<RelayerSessionShared, RelayerError> {
        if !matches!(self.state, ChannelState::Waiting { .. }) {
            return Err(RelayerError::ChannelUnavailable)
        }

        let ChannelState::Waiting { host, notify } = mem::take(&mut self.state) else {
            unreachable!()
        };

        // Change its state
        self.state = ChannelState::Ready;

        // Notify the host that we've joined
        notify.send(client.clone())
            .map_err(|_| RelayerError::NotifyHost)?;

        Ok(host)
    }
}