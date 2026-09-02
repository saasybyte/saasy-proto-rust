use std::convert::{From, TryFrom};

use mediasoup::prelude::MediaKind as MediasoupMediaKind;

use crate::shared::MediaKind;
use super::error::ConversionError;

impl TryFrom<MediaKind> for MediasoupMediaKind {
    type Error = ConversionError;

    fn try_from(kind: MediaKind) -> Result<Self, Self::Error> {
        match kind {
            MediaKind::Audio => Ok(Self::Audio),
            MediaKind::Video => Ok(Self::Video),
            MediaKind::Unspecified => Err(ConversionError::InvalidMediaKind(0)),
        }
    }
}

impl From<MediasoupMediaKind> for MediaKind {
    fn from(kind: MediasoupMediaKind) -> Self {
        match kind {
            MediasoupMediaKind::Audio => Self::Audio,
            MediasoupMediaKind::Video => Self::Video,
        }
    }
}
