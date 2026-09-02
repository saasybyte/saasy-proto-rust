use std::convert::From;

use mediasoup::prelude::ConsumerId as MediasoupConsumerId;

use crate::shared::ConsumerId;

impl From<MediasoupConsumerId> for ConsumerId {
    fn from(id: MediasoupConsumerId) -> Self {
        Self { id: id.to_string() }
    }
}
