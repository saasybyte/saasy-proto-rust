use std::convert::From;

use mediasoup::prelude::ProducerId as MediasoupProducerId;

use crate::shared::ProducerId;

impl From<MediasoupProducerId> for ProducerId {
    fn from(id: MediasoupProducerId) -> Self {
        Self { id: id.to_string() }
    }
}
