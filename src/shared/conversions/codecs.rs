use std::convert::TryFrom;
use std::num::{NonZeroU32, NonZeroU8};

use mediasoup::prelude::{
    RtcpFeedback as MediasoupRtcpFeedback,
    RtpCodecCapability as MediasoupRtpCodecCapability,
    RtpCodecParameters as MediasoupRtpCodecParameters,
};
use mediasoup::types::rtp_parameters::RtpCodecCapabilityFinalized as MediasoupRtpCodecCapabilityFinalized;

use crate::shared::{
    audio_rtp_codec_capability,
    video_rtp_codec_capability,
    AudioRtpCodecCapability,
    AudioRtpCodecCapabilityFinalized,
    AudioRtpCodecParameters,
    MimeTypeAudio,
    MimeTypeVideo,
    RtcpFeedback,
    VideoRtpCodecCapability,
    VideoRtpCodecCapabilityFinalized,
    VideoRtpCodecParameters,
};
use super::error::ConversionError;
use super::rtp::MediasoupRtpCodecParametersParametersWrapper;

impl TryFrom<AudioRtpCodecCapabilityFinalized> for MediasoupRtpCodecCapabilityFinalized {
    type Error = ConversionError;

    fn try_from(codec: AudioRtpCodecCapabilityFinalized) -> Result<Self, Self::Error> {
        let mime_type = MimeTypeAudio::try_from(codec.mime_type)
            .map_err(|_| ConversionError::InvalidMimeType(codec.mime_type.to_string()))?
            .try_into()?;

        let preferred_payload_type = u8::try_from(codec.preferred_payload_type)
            .map_err(|_| ConversionError::InvalidPayloadType(codec.preferred_payload_type))?;

        let clock_rate = NonZeroU32::new(codec.clock_rate)
            .ok_or(ConversionError::InvalidClockRate(codec.clock_rate))?;

        let channels = NonZeroU8::new(u8::try_from(codec.channels)
            .map_err(|_| ConversionError::InvalidChannels(codec.channels))?)
            .ok_or(ConversionError::InvalidChannels(codec.channels))?;

        let parameters = MediasoupRtpCodecParametersParametersWrapper::try_from(codec.parameters)?
            .0;

        let rtcp_feedback = codec.rtcp_feedback
            .into_iter()
            .map(MediasoupRtcpFeedback::try_from)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self::Audio {
            mime_type,
            preferred_payload_type,
            clock_rate,
            channels,
            parameters,
            rtcp_feedback,
        })
    }
}

impl TryFrom<MediasoupRtpCodecCapabilityFinalized> for AudioRtpCodecCapabilityFinalized {
    type Error = ConversionError;

    fn try_from(codec: MediasoupRtpCodecCapabilityFinalized) -> Result<Self, Self::Error> {
        match codec {
            MediasoupRtpCodecCapabilityFinalized::Audio {
                mime_type,
                preferred_payload_type,
                clock_rate,
                channels,
                parameters,
                rtcp_feedback,
            } => {
                let rtcp_feedback: Vec<RtcpFeedback> = rtcp_feedback
                    .into_iter()
                    .map(RtcpFeedback::from)
                    .collect();

                let parameters = MediasoupRtpCodecParametersParametersWrapper(parameters).into();

                Ok(Self {
                    mime_type: MimeTypeAudio::from(mime_type) as i32,
                    preferred_payload_type: u32::from(preferred_payload_type),
                    clock_rate: clock_rate.get(),
                    channels: u32::from(channels.get()),
                    parameters,
                    rtcp_feedback,
                })
            },
            _ => Err(ConversionError::ExpectedAudioCodec),
        }
    }
}

impl TryFrom<VideoRtpCodecCapabilityFinalized> for MediasoupRtpCodecCapabilityFinalized {
    type Error = ConversionError;

    fn try_from(codec: VideoRtpCodecCapabilityFinalized) -> Result<Self, Self::Error> {
        let mime_type = MimeTypeVideo::try_from(codec.mime_type)
            .map_err(|_| ConversionError::InvalidMimeType(codec.mime_type.to_string()))?
            .try_into()?;

        let preferred_payload_type = u8::try_from(codec.preferred_payload_type)
            .map_err(|_| ConversionError::InvalidPayloadType(codec.preferred_payload_type))?;

        let clock_rate = NonZeroU32::new(codec.clock_rate)
            .ok_or(ConversionError::InvalidClockRate(codec.clock_rate))?;

        let parameters = MediasoupRtpCodecParametersParametersWrapper::try_from(codec.parameters)?
            .0;

        let rtcp_feedback = codec.rtcp_feedback
            .into_iter()
            .map(MediasoupRtcpFeedback::try_from)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self::Video {
            mime_type,
            preferred_payload_type,
            clock_rate,
            parameters,
            rtcp_feedback,
        })
    }
}

impl TryFrom<MediasoupRtpCodecCapabilityFinalized> for VideoRtpCodecCapabilityFinalized {
    type Error = ConversionError;

    fn try_from(codec: MediasoupRtpCodecCapabilityFinalized) -> Result<Self, Self::Error> {
        match codec {
            MediasoupRtpCodecCapabilityFinalized::Video {
                mime_type,
                preferred_payload_type,
                clock_rate,
                parameters,
                rtcp_feedback,
            } => {
                let rtcp_feedback: Vec<RtcpFeedback> = rtcp_feedback
                    .into_iter()
                    .map(RtcpFeedback::from)
                    .collect();

                let parameters = MediasoupRtpCodecParametersParametersWrapper(parameters).into();

                Ok(Self {
                    mime_type: MimeTypeVideo::from(mime_type) as i32,
                    preferred_payload_type: u32::from(preferred_payload_type),
                    clock_rate: clock_rate.get(),
                    parameters,
                    rtcp_feedback,
                })
            },
            _ => Err(ConversionError::ExpectedVideoCodec),
        }
    }
}

impl TryFrom<AudioRtpCodecCapability> for MediasoupRtpCodecCapability {
    type Error = ConversionError;

    fn try_from(codec: AudioRtpCodecCapability) -> Result<Self, Self::Error> {
        let mime_type = MimeTypeAudio::try_from(codec.mime_type)
            .map_err(|_| ConversionError::InvalidMimeType(codec.mime_type.to_string()))?
            .try_into()?;

        let preferred_payload_type = match codec.preferred_payload_type_opt {
            Some(audio_rtp_codec_capability::PreferredPayloadTypeOpt::PreferredPayloadType(v)) => {
                let preferred_payload_type_u8 = u8::try_from(v)
                    .map_err(|_| ConversionError::InvalidPayloadType(v))?;
                Some(preferred_payload_type_u8)
            }
            None => None,
        };

        let clock_rate = NonZeroU32::new(codec.clock_rate)
            .ok_or(ConversionError::InvalidClockRate(codec.clock_rate))?;

        let channels = NonZeroU8::new(u8::try_from(codec.channels)
            .map_err(|_| ConversionError::InvalidChannels(codec.channels))?)
            .ok_or(ConversionError::InvalidChannels(codec.channels))?;

        let parameters = MediasoupRtpCodecParametersParametersWrapper::try_from(codec.parameters)?
            .0;

        let rtcp_feedback = codec.rtcp_feedback
            .into_iter()
            .map(MediasoupRtcpFeedback::try_from)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self::Audio {
            mime_type,
            clock_rate,
            channels,
            parameters,
            rtcp_feedback,
            preferred_payload_type,
        })
    }
}

impl TryFrom<MediasoupRtpCodecCapability> for AudioRtpCodecCapability {
    type Error = ConversionError;

    fn try_from(codec: MediasoupRtpCodecCapability) -> Result<Self, Self::Error> {
        match codec {
            MediasoupRtpCodecCapability::Audio {
                mime_type,
                preferred_payload_type,
                clock_rate,
                channels,
                parameters,
                rtcp_feedback,
            } => {
                let preferred_payload_type_opt = preferred_payload_type.map(|ppt| {
                    audio_rtp_codec_capability::PreferredPayloadTypeOpt::PreferredPayloadType(
                        ppt.into(),
                    )
                });
                
                let rtcp_feedback: Vec<RtcpFeedback> = rtcp_feedback
                    .into_iter()
                    .map(RtcpFeedback::from)
                    .collect();

                let parameters = MediasoupRtpCodecParametersParametersWrapper(parameters).into();

                Ok(Self {
                    preferred_payload_type_opt,
                    parameters,
                    rtcp_feedback,
                    mime_type: MimeTypeAudio::from(mime_type) as i32,
                    clock_rate: clock_rate.get(),
                    channels: u32::from(channels.get()),
                })
            },
            MediasoupRtpCodecCapability::Video { .. } => Err(ConversionError::ExpectedAudioCodec),
        }
    }
}

impl TryFrom<VideoRtpCodecCapability> for MediasoupRtpCodecCapability {
    type Error = ConversionError;

    fn try_from(codec: VideoRtpCodecCapability) -> Result<Self, Self::Error> {
        let mime_type = MimeTypeVideo::try_from(codec.mime_type)
            .map_err(|_| ConversionError::InvalidMimeType(codec.mime_type.to_string()))?
            .try_into()?;

        let preferred_payload_type = match codec.preferred_payload_type_opt {
            Some(video_rtp_codec_capability::PreferredPayloadTypeOpt::PreferredPayloadType(v)) => {
                let preferred_payload_type_u8 = u8::try_from(v)
                    .map_err(|_| ConversionError::InvalidPayloadType(v))?;
                Some(preferred_payload_type_u8)
            }
            None => None,
        };

        let clock_rate = NonZeroU32::new(codec.clock_rate)
            .ok_or(ConversionError::InvalidClockRate(codec.clock_rate))?;

        let parameters = MediasoupRtpCodecParametersParametersWrapper::try_from(codec.parameters)?
            .0;

        let rtcp_feedback = codec.rtcp_feedback
            .into_iter()
            .map(MediasoupRtcpFeedback::try_from)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self::Video {
            mime_type,
            clock_rate,
            parameters,
            rtcp_feedback,
            preferred_payload_type,
        })
    }
}

impl TryFrom<MediasoupRtpCodecCapability> for VideoRtpCodecCapability {
    type Error = ConversionError;

    fn try_from(codec: MediasoupRtpCodecCapability) -> Result<Self, Self::Error> {
        match codec {
            MediasoupRtpCodecCapability::Video {
                mime_type,
                preferred_payload_type,
                clock_rate,
                parameters,
                rtcp_feedback,
            } => {
                let preferred_payload_type_opt = preferred_payload_type.map(|ppt| {
                    video_rtp_codec_capability::PreferredPayloadTypeOpt::PreferredPayloadType(
                        ppt.into(),
                    )
                });

                let rtcp_feedback: Vec<RtcpFeedback> = rtcp_feedback
                    .into_iter()
                    .map(RtcpFeedback::from)
                    .collect();

                let parameters = MediasoupRtpCodecParametersParametersWrapper(parameters).into();

                Ok(Self {
                    preferred_payload_type_opt,
                    parameters,
                    rtcp_feedback,
                    mime_type: MimeTypeVideo::from(mime_type) as i32,
                    clock_rate: clock_rate.get(),
                })
            },
            MediasoupRtpCodecCapability::Audio{ .. } => Err(ConversionError::ExpectedVideoCodec),
        }
    }
}

impl TryFrom<AudioRtpCodecParameters> for MediasoupRtpCodecParameters {
    type Error = ConversionError;

    fn try_from(codec: AudioRtpCodecParameters) -> Result<Self, Self::Error> {
        let mime_type = MimeTypeAudio::try_from(codec.mime_type)
            .map_err(|_| ConversionError::InvalidMimeType(codec.mime_type.to_string()))?
            .try_into()?;

        let payload_type = u8::try_from(codec.payload_type)
            .map_err(|_| ConversionError::InvalidPayloadType(codec.payload_type))?;

        let clock_rate = NonZeroU32::new(codec.clock_rate)
            .ok_or(ConversionError::InvalidClockRate(codec.clock_rate))?;

        let channels = NonZeroU8::new(u8::try_from(codec.channels)
            .map_err(|_| ConversionError::InvalidChannels(codec.channels))?)
            .ok_or(ConversionError::InvalidChannels(codec.channels))?;

        let parameters = MediasoupRtpCodecParametersParametersWrapper::try_from(codec.parameters)?
            .0;

        let rtcp_feedback = codec.rtcp_feedback
            .into_iter()
            .map(MediasoupRtcpFeedback::try_from)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self::Audio {
            mime_type,
            payload_type,
            clock_rate,
            channels,
            parameters,
            rtcp_feedback,
        })
    }
}

impl TryFrom<MediasoupRtpCodecParameters> for AudioRtpCodecParameters {
    type Error = ConversionError;

    fn try_from(codec: MediasoupRtpCodecParameters) -> Result<Self, Self::Error> {
        match codec {
            MediasoupRtpCodecParameters::Audio {
                mime_type,
                payload_type,
                clock_rate,
                channels,
                parameters,
                rtcp_feedback,
            } => {
                let rtcp_feedback: Vec<RtcpFeedback> = rtcp_feedback
                    .into_iter()
                    .map(RtcpFeedback::from)
                    .collect();

                let parameters = MediasoupRtpCodecParametersParametersWrapper(parameters).into();

                Ok(Self {
                    mime_type: MimeTypeAudio::from(mime_type) as i32,
                    payload_type: u32::from(payload_type),
                    clock_rate: clock_rate.get(),
                    channels: u32::from(channels.get()),
                    parameters,
                    rtcp_feedback,
                })
            },
            MediasoupRtpCodecParameters::Video{ .. } => Err(ConversionError::ExpectedAudioCodec),
        }
    }
}

impl TryFrom<VideoRtpCodecParameters> for MediasoupRtpCodecParameters {
    type Error = ConversionError;

    fn try_from(codec: VideoRtpCodecParameters) -> Result<Self, Self::Error> {
        let mime_type = MimeTypeVideo::try_from(codec.mime_type)
            .map_err(|_| ConversionError::InvalidMimeType(codec.mime_type.to_string()))?
            .try_into()?;

        let payload_type = u8::try_from(codec.payload_type)
            .map_err(|_| ConversionError::InvalidPayloadType(codec.payload_type))?;

        let clock_rate = NonZeroU32::new(codec.clock_rate)
            .ok_or(ConversionError::InvalidClockRate(codec.clock_rate))?;

        let parameters = MediasoupRtpCodecParametersParametersWrapper::try_from(codec.parameters)?
            .0;

        let rtcp_feedback = codec.rtcp_feedback
            .into_iter()
            .map(MediasoupRtcpFeedback::try_from)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self::Video {
            mime_type,
            payload_type,
            clock_rate,
            parameters,
            rtcp_feedback,
        })
    }
}

impl TryFrom<MediasoupRtpCodecParameters> for VideoRtpCodecParameters {
    type Error = ConversionError;

    fn try_from(codec: MediasoupRtpCodecParameters) -> Result<Self, Self::Error> {
        match codec {
            MediasoupRtpCodecParameters::Video {
                mime_type,
                payload_type,
                clock_rate,
                parameters,
                rtcp_feedback,
            } => {
                let rtcp_feedback: Vec<RtcpFeedback> = rtcp_feedback
                    .into_iter()
                    .map(RtcpFeedback::from)
                    .collect();

                let parameters = MediasoupRtpCodecParametersParametersWrapper(parameters).into();

                Ok(Self {
                    mime_type: MimeTypeVideo::from(mime_type) as i32,
                    payload_type: u32::from(payload_type),
                    clock_rate: clock_rate.get(),
                    parameters,
                    rtcp_feedback,
                })
            },
            MediasoupRtpCodecParameters::Audio{ .. } => Err(ConversionError::ExpectedVideoCodec),
        }
    }
}
