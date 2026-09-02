use std::borrow::Cow;
use std::convert::{From, TryFrom, TryInto};
use std::num::NonZeroU8;

use mediasoup::prelude::{
    MimeTypeAudio as MediasoupMimeTypeAudio,
    MimeTypeVideo as MediasoupMimeTypeVideo,
    RtcpFeedback as MediasoupRtcpFeedback,
    RtpCapabilities as MediasoupRtpCapabilities,
    RtpCapabilitiesFinalized as MediasoupRtpCapabilitiesFinalized,
    RtpCodecCapability as MediasoupRtpCodecCapability,
    RtpCodecParametersParameters as MediasoupRtpCodecParametersParameters,
    RtpParameters as MediasoupRtpParameters,
    RtpCodecParameters as MediasoupRtpCodecParameters,
    RtpEncodingParameters as MediasoupRtpEncodingParameters,
    RtpHeaderExtensionParameters as MediasoupRtpHeaderExtensionParameters,
};
use mediasoup::types::rtp_parameters::{
    RtcpParameters as MediasoupRtcpParameters,
    RtpCodecCapabilityFinalized as MediasoupRtpCodecCapabilityFinalized,
    RtpCodecParametersParametersValue as MediasoupRtpCodecParametersParametersValue,
    RtpEncodingParametersRtx as MediasoupRtpEncodingParametersRtx,
    RtpHeaderExtension as MediasoupRtpHeaderExtension,
    RtpHeaderExtensionDirection as MediasoupRtpHeaderExtensionDirection,
    RtpHeaderExtensionUri as MediasoupRtpHeaderExtensionUri,
};
use mediasoup::types::scalability_modes::ScalabilityMode as MediasoupScalabilityMode;

use crate::shared::{
    rtcp_parameters,
    rtp_codec_capability,
    rtp_codec_capability_finalized,
    rtp_codec_parameters,
    rtp_codec_parameters_parameters,
    rtp_encoding_parameters,
    rtp_parameters,
    scalability_mode,
    MediaKind,
    MimeTypeAudio,
    MimeTypeVideo,
    RtcpFeedback,
    RtcpParameters,
    RtpCapabilities,
    RtpCapabilitiesFinalized,
    RtpCodecCapability,
    RtpCodecCapabilityFinalized,
    RtpCodecParameters,
    RtpCodecParametersParameters,
    RtpEncodingParameters,
    RtpEncodingParametersRtx,
    RtpHeaderExtension,
    RtpHeaderExtensionDirection,
    RtpHeaderExtensionParameters,
    RtpHeaderExtensionUri,
    RtpParameters,
    ScalabilityMode,
    ScalabilityModeCustom,
    ScalabilityModeEnum,
};
use super::error::ConversionError;

impl TryFrom<RtpHeaderExtensionUri> for MediasoupRtpHeaderExtensionUri {
    type Error = ConversionError;

    fn try_from(uri: RtpHeaderExtensionUri) -> Result<Self, Self::Error> {
        match uri {
            RtpHeaderExtensionUri::Mid => Ok(Self::Mid),
            RtpHeaderExtensionUri::RtpStreamId => Ok(Self::RtpStreamId),
            RtpHeaderExtensionUri::RepairRtpStreamId => Ok(Self::RepairRtpStreamId),
            RtpHeaderExtensionUri::AudioLevel => Ok(Self::AudioLevel),
            RtpHeaderExtensionUri::VideoOrientation => Ok(Self::VideoOrientation),
            RtpHeaderExtensionUri::TimeOffset => Ok(Self::TimeOffset),
            RtpHeaderExtensionUri::TransportWideCcDraft01 => Ok(Self::TransportWideCcDraft01),
            RtpHeaderExtensionUri::AbsSendTime => Ok(Self::AbsSendTime),
            RtpHeaderExtensionUri::AbsCaptureTime => Ok(Self::AbsCaptureTime),
            RtpHeaderExtensionUri::PlayoutDelay => Ok(Self::PlayoutDelay),
            RtpHeaderExtensionUri::DependencyDescriptor => Ok(Self::DependencyDescriptor),
            RtpHeaderExtensionUri::Unspecified => Err(ConversionError::InvalidHeaderExtensionUri),
        }
    }
}

impl From<MediasoupRtpHeaderExtensionUri> for RtpHeaderExtensionUri {
    fn from(uri: MediasoupRtpHeaderExtensionUri) -> Self {
        match uri {
            MediasoupRtpHeaderExtensionUri::Mid => Self::Mid,
            MediasoupRtpHeaderExtensionUri::RtpStreamId => Self::RtpStreamId,
            MediasoupRtpHeaderExtensionUri::RepairRtpStreamId => Self::RepairRtpStreamId,
            MediasoupRtpHeaderExtensionUri::AudioLevel => Self::AudioLevel,
            MediasoupRtpHeaderExtensionUri::VideoOrientation => Self::VideoOrientation,
            MediasoupRtpHeaderExtensionUri::TimeOffset => Self::TimeOffset,
            MediasoupRtpHeaderExtensionUri::TransportWideCcDraft01 => Self::TransportWideCcDraft01,
            MediasoupRtpHeaderExtensionUri::AbsSendTime => Self::AbsSendTime,
            MediasoupRtpHeaderExtensionUri::AbsCaptureTime => Self::AbsCaptureTime,
            MediasoupRtpHeaderExtensionUri::PlayoutDelay => Self::PlayoutDelay,
            MediasoupRtpHeaderExtensionUri::DependencyDescriptor => Self::DependencyDescriptor,
            MediasoupRtpHeaderExtensionUri::Unsupported => Self::Unspecified,
        }
    }
}

impl TryFrom<RtpHeaderExtensionDirection> for MediasoupRtpHeaderExtensionDirection {
    type Error = ConversionError;

    fn try_from(direction: RtpHeaderExtensionDirection) -> Result<Self, Self::Error> {
        match direction {
            RtpHeaderExtensionDirection::SendRecv => Ok(Self::SendRecv),
            RtpHeaderExtensionDirection::SendOnly => Ok(Self::SendOnly),
            RtpHeaderExtensionDirection::RecvOnly => Ok(Self::RecvOnly),
            RtpHeaderExtensionDirection::Inactive => Ok(Self::Inactive),
            RtpHeaderExtensionDirection::Unspecified => Err(ConversionError::InvalidHeaderDirection),
        }
    }
}

impl From<MediasoupRtpHeaderExtensionDirection> for RtpHeaderExtensionDirection {
    fn from(direction: MediasoupRtpHeaderExtensionDirection) -> Self {
        match direction {
            MediasoupRtpHeaderExtensionDirection::SendRecv => Self::SendRecv,
            MediasoupRtpHeaderExtensionDirection::SendOnly => Self::SendOnly,
            MediasoupRtpHeaderExtensionDirection::RecvOnly => Self::RecvOnly,
            MediasoupRtpHeaderExtensionDirection::Inactive => Self::Inactive,
        }
    }
}

impl TryFrom<RtpHeaderExtension> for MediasoupRtpHeaderExtension {
    type Error = ConversionError;

    fn try_from(header_ext: RtpHeaderExtension) -> Result<Self, Self::Error> {
        let preferred_id = u16::try_from(header_ext.preferred_id)
            .map_err(|_| ConversionError::InvalidPreferredId(header_ext.preferred_id))?;

        let uri = RtpHeaderExtensionUri::try_from(header_ext.uri)
            .map_err(|_| ConversionError::InvalidHeaderExtensionUri)?
            .try_into()?;

        let kind = MediaKind::try_from(header_ext.kind)
            .map_err(|_| ConversionError::InvalidMediaKind(header_ext.kind))?
            .try_into()?;

        let direction = RtpHeaderExtensionDirection::try_from(header_ext.direction)
            .map_err(|_| ConversionError::InvalidHeaderDirection)?
            .try_into()?;

        Ok(Self {
            preferred_id,
            uri,
            kind,
            direction,
            preferred_encrypt: header_ext.preferred_encrypt,
        })
    }
}

impl From<MediasoupRtpHeaderExtension> for RtpHeaderExtension {
    fn from(header_ext: MediasoupRtpHeaderExtension) -> Self {
        Self {
            uri: RtpHeaderExtensionUri::from(header_ext.uri) as i32,
            preferred_id: u32::from(header_ext.preferred_id),
            preferred_encrypt: header_ext.preferred_encrypt,
            kind: MediaKind::from(header_ext.kind) as i32,
            direction: RtpHeaderExtensionDirection::from(header_ext.direction) as i32,
        }
    }
}

impl TryFrom<MimeTypeAudio> for MediasoupMimeTypeAudio {
    type Error = ConversionError;

    fn try_from(value: MimeTypeAudio) -> Result<Self, Self::Error> {
        match value {
            MimeTypeAudio::Opus => Ok(Self::Opus),
            MimeTypeAudio::MultiChannelOpus => Ok(Self::MultiChannelOpus),
            MimeTypeAudio::Pcmu => Ok(Self::Pcmu),
            MimeTypeAudio::Pcma => Ok(Self::Pcma),
            MimeTypeAudio::Isac => Ok(Self::Isac),
            MimeTypeAudio::G722 => Ok(Self::G722),
            MimeTypeAudio::Ilbc => Ok(Self::Ilbc),
            MimeTypeAudio::Silk => Ok(Self::Silk),
            MimeTypeAudio::Cn => Ok(Self::Cn),
            MimeTypeAudio::TelephoneEvent => Ok(Self::TelephoneEvent),
            MimeTypeAudio::Rtx => Ok(Self::Rtx),
            MimeTypeAudio::Red => Ok(Self::Red),
            MimeTypeAudio::Unspecified => Err(ConversionError::InvalidMimeTypeAudio),
        }
    }
}

impl From<MediasoupMimeTypeAudio> for MimeTypeAudio {
    fn from(value: MediasoupMimeTypeAudio) -> Self {
        match value {
            MediasoupMimeTypeAudio::Opus => Self::Opus,
            MediasoupMimeTypeAudio::MultiChannelOpus => Self::MultiChannelOpus,
            MediasoupMimeTypeAudio::Pcmu => Self::Pcmu,
            MediasoupMimeTypeAudio::Pcma => Self::Pcma,
            MediasoupMimeTypeAudio::Isac => Self::Isac,
            MediasoupMimeTypeAudio::G722 => Self::G722,
            MediasoupMimeTypeAudio::Ilbc => Self::Ilbc,
            MediasoupMimeTypeAudio::Silk => Self::Silk,
            MediasoupMimeTypeAudio::Cn => Self::Cn,
            MediasoupMimeTypeAudio::TelephoneEvent => Self::TelephoneEvent,
            MediasoupMimeTypeAudio::Rtx => Self::Rtx,
            MediasoupMimeTypeAudio::Red => Self::Red,
        }
    }
}

impl TryFrom<MimeTypeVideo> for MediasoupMimeTypeVideo {
    type Error = ConversionError;

    fn try_from(value: MimeTypeVideo) -> Result<Self, Self::Error> {
        match value {
            MimeTypeVideo::Vp8 => Ok(Self::Vp8),
            MimeTypeVideo::Vp9 => Ok(Self::Vp9),
            MimeTypeVideo::H264 => Ok(Self::H264),
            MimeTypeVideo::Av1 => Ok(Self::AV1),
            MimeTypeVideo::Rtx => Ok(Self::Rtx),
            MimeTypeVideo::Red => Ok(Self::Red),
            MimeTypeVideo::Ulpfec => Ok(Self::Ulpfec),
            MimeTypeVideo::Unspecified => Err(ConversionError::InvalidMimeTypeVideo),
        }
    }
}

impl From<MediasoupMimeTypeVideo> for MimeTypeVideo {
    fn from(value: MediasoupMimeTypeVideo) -> Self {
        match value {
            MediasoupMimeTypeVideo::Vp8 => Self::Vp8,
            MediasoupMimeTypeVideo::Vp9 => Self::Vp9,
            MediasoupMimeTypeVideo::H264 => Self::H264,
            MediasoupMimeTypeVideo::AV1 => Self::Av1,
            MediasoupMimeTypeVideo::Rtx => Self::Rtx,
            MediasoupMimeTypeVideo::Red => Self::Red,
            MediasoupMimeTypeVideo::Ulpfec => Self::Ulpfec,
        }
    }
}

pub struct MediasoupRtpCodecParametersParametersWrapper(pub MediasoupRtpCodecParametersParameters);

impl TryFrom<Vec<RtpCodecParametersParameters>> for MediasoupRtpCodecParametersParametersWrapper {
    type Error = ConversionError;

    fn try_from(entries: Vec<RtpCodecParametersParameters>) -> Result<Self, Self::Error> {
        let map = entries
            .into_iter()
            .map(|entry| {
                let key = Cow::Owned(entry.key);
                let value = match entry.value {
                    Some(rtp_codec_parameters_parameters::Value::StringValue(s)) => {
                        MediasoupRtpCodecParametersParametersValue::String(Cow::Owned(s))
                    }
                    Some(rtp_codec_parameters_parameters::Value::NumberValue(n)) => {
                        MediasoupRtpCodecParametersParametersValue::Number(n)
                    }
                    None => return Err(ConversionError::MissingCodecParameterValue),
                };
                Ok((key, value))
            })
            .collect::<Result<MediasoupRtpCodecParametersParameters, _>>()?;

        Ok(Self(map))
    }
}

impl From<MediasoupRtpCodecParametersParametersWrapper> for Vec<RtpCodecParametersParameters> {
    fn from(wrapper: MediasoupRtpCodecParametersParametersWrapper) -> Self {
        wrapper.0
            .iter()
            .map(|(key, value)| {
                let value = match value {
                    MediasoupRtpCodecParametersParametersValue::String(s) => {
                        Some(rtp_codec_parameters_parameters::Value::StringValue(s.to_string()))
                    }
                    MediasoupRtpCodecParametersParametersValue::Number(n) => {
                        Some(rtp_codec_parameters_parameters::Value::NumberValue(*n))
                    }
                };

                RtpCodecParametersParameters {
                    key: key.to_string(),
                    value,
                }
            })
            .collect()
    }
}

impl TryFrom<RtcpFeedback> for MediasoupRtcpFeedback {
    type Error = ConversionError;

    fn try_from(value: RtcpFeedback) -> Result<Self, Self::Error> {
        match (value.r#type.as_str(), value.parameter.as_str()) {
            ("nack", "") => Ok(Self::Nack),
            ("nack", "pli") => Ok(Self::NackPli),
            ("ccm", "fir") => Ok(Self::CcmFir),
            ("goog-remb", "") => Ok(Self::GoogRemb),
            ("transport-cc", "") => Ok(Self::TransportCc),
            _ => Err(ConversionError::InvalidRtcpFeedbackType),
        }
    }
}

impl From<MediasoupRtcpFeedback> for RtcpFeedback {
    fn from(value: MediasoupRtcpFeedback) -> Self {
        match value {
            MediasoupRtcpFeedback::Nack => Self {
                r#type: "nack".to_string(),
                parameter: String::new(),
            },
            MediasoupRtcpFeedback::NackPli => Self {
                r#type: "nack".to_string(),
                parameter: "pli".to_string(),
            },
            MediasoupRtcpFeedback::CcmFir => Self {
                r#type: "ccm".to_string(),
                parameter: "fir".to_string(),
            },
            MediasoupRtcpFeedback::GoogRemb => Self {
                r#type: "goog-remb".to_string(),
                parameter: String::new(),
            },
            MediasoupRtcpFeedback::TransportCc => Self {
                r#type: "transport-cc".to_string(),
                parameter: String::new(),
            },
            MediasoupRtcpFeedback::Unsupported => Self {
                r#type: "unsupported".to_string(),
                parameter: String::new(),
            },
        }
    }
}

impl TryFrom<RtpCodecCapabilityFinalized> for MediasoupRtpCodecCapabilityFinalized {
    type Error = ConversionError;

    fn try_from(codec: RtpCodecCapabilityFinalized) -> Result<Self, Self::Error> {
        match codec.capability {
            Some(rtp_codec_capability_finalized::Capability::Audio(audio)) => {
                audio.try_into()
            }
            Some(rtp_codec_capability_finalized::Capability::Video(video)) => {
                video.try_into()
            }
            None => Err(ConversionError::MissingCodecCapability),
        }
    }
}

impl TryFrom<MediasoupRtpCodecCapabilityFinalized> for RtpCodecCapabilityFinalized {
    type Error = ConversionError;

    fn try_from(codec: MediasoupRtpCodecCapabilityFinalized) -> Result<Self, Self::Error> {
        let capability = match codec {
            MediasoupRtpCodecCapabilityFinalized::Audio { .. } => {
                rtp_codec_capability_finalized::Capability::Audio(codec.try_into()?)
            }
            MediasoupRtpCodecCapabilityFinalized::Video { .. } => {
                rtp_codec_capability_finalized::Capability::Video(codec.try_into()?)
            }
            _ => return Err(ConversionError::UnsupportedCodecType),
        };

        Ok(Self {
            capability: Some(capability),
        })
    }
}

impl TryFrom<RtpCodecCapability> for MediasoupRtpCodecCapability {
    type Error = ConversionError;

    fn try_from(codec: RtpCodecCapability) -> Result<Self, Self::Error> {
        match codec.capability {
            Some(rtp_codec_capability::Capability::Audio(audio)) => {
                audio.try_into()
            }
            Some(rtp_codec_capability::Capability::Video(video)) => {
                video.try_into()
            }
            None => Err(ConversionError::MissingCodecCapability),
        }
    }
}

impl TryFrom<MediasoupRtpCodecCapability> for RtpCodecCapability {
    type Error = ConversionError;

    fn try_from(codec: MediasoupRtpCodecCapability) -> Result<Self, Self::Error> {
        let capability = match codec {
            MediasoupRtpCodecCapability::Audio { .. } => {
                rtp_codec_capability::Capability::Audio(codec.try_into()?)
            }
            MediasoupRtpCodecCapability::Video { .. } => {
                rtp_codec_capability::Capability::Video(codec.try_into()?)
            }
        };

        Ok(Self {
            capability: Some(capability),
        })
    }
}

impl TryFrom<MediasoupRtpCapabilitiesFinalized> for RtpCapabilitiesFinalized {
    type Error = ConversionError;
    
    fn try_from(capabilities: MediasoupRtpCapabilitiesFinalized) -> Result<Self, Self::Error> {
        let codecs = capabilities
            .codecs
            .into_iter()
            .map(RtpCodecCapabilityFinalized::try_from)
            .collect::<Result<Vec<_>, _>>()?;

        let header_extensions = capabilities
            .header_extensions
            .into_iter()
            .map(RtpHeaderExtension::from)
            .collect();

        Ok(Self {
            codecs,
            header_extensions,
        })
    }
}

impl TryFrom<RtpCapabilities> for MediasoupRtpCapabilities {
    type Error = ConversionError;

    fn try_from(capabilities: RtpCapabilities) -> Result<Self, Self::Error> {
        let codecs = capabilities
            .codecs
            .into_iter()
            .map(MediasoupRtpCodecCapability::try_from)
            .collect::<Result<Vec<_>, _>>()?;

        let header_extensions = capabilities
            .header_extensions
            .into_iter()
            .map(MediasoupRtpHeaderExtension::try_from)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            codecs,
            header_extensions,
        })
    }
}

impl TryFrom<MediasoupRtpCapabilities> for RtpCapabilities {
    type Error = ConversionError;
    
    fn try_from(capabilities: MediasoupRtpCapabilities) -> Result<Self, Self::Error> {
        let codecs = capabilities
            .codecs
            .into_iter()
            .map(RtpCodecCapability::try_from)
            .collect::<Result<Vec<_>, _>>()?;

        let header_extensions = capabilities
            .header_extensions
            .into_iter()
            .map(RtpHeaderExtension::from)
            .collect();

        Ok(Self {
            codecs,
            header_extensions,
        })
    }
}

impl TryFrom<RtpCodecParameters> for MediasoupRtpCodecParameters {
    type Error = ConversionError;

    fn try_from(codec: RtpCodecParameters) -> Result<Self, Self::Error> {
        match codec.parameters {
            Some(rtp_codec_parameters::Parameters::Audio(audio)) => {
                audio.try_into()
            }
            Some(rtp_codec_parameters::Parameters::Video(video)) => {
                video.try_into()
            }
            None => Err(ConversionError::MissingCodecCapability),
        }
    }
}

impl TryFrom<MediasoupRtpCodecParameters> for RtpCodecParameters {
    type Error = ConversionError;

    fn try_from(codec: MediasoupRtpCodecParameters) -> Result<Self, Self::Error> {
        let parameters = match codec {
            MediasoupRtpCodecParameters::Audio { .. } => {
                rtp_codec_parameters::Parameters::Audio(codec.try_into()?)
            }
            MediasoupRtpCodecParameters::Video { .. } => {
                rtp_codec_parameters::Parameters::Video(codec.try_into()?)
            }
        };

        Ok(Self {
            parameters: Some(parameters),
        })
    }
}

impl TryFrom<RtpHeaderExtensionParameters> for MediasoupRtpHeaderExtensionParameters {
    type Error = ConversionError;

    fn try_from(params: RtpHeaderExtensionParameters) -> Result<Self, Self::Error> {
        let uri = RtpHeaderExtensionUri::try_from(params.uri)
            .map_err(|_| ConversionError::InvalidHeaderExtensionUri)?
            .try_into()?;

        let id = u16::try_from(params.id)
            .map_err(|_| ConversionError::InvalidHeaderExtensionId(params.id))?;

        Ok(Self {
            uri,
            id,
            encrypt: params.encrypt,
        })
    }
}

impl From<MediasoupRtpHeaderExtensionParameters> for RtpHeaderExtensionParameters {
    fn from(params: MediasoupRtpHeaderExtensionParameters) -> Self {
        Self {
            uri: RtpHeaderExtensionUri::from(params.uri) as i32,
            id: u32::from(params.id),
            encrypt: params.encrypt,
        }
    }
}

impl From<RtpEncodingParametersRtx> for MediasoupRtpEncodingParametersRtx {
    fn from(proto: RtpEncodingParametersRtx) -> Self {
        Self { ssrc: proto.ssrc }
    }
}

impl From<MediasoupRtpEncodingParametersRtx> for RtpEncodingParametersRtx {
    fn from(ms: MediasoupRtpEncodingParametersRtx) -> Self {
        Self { ssrc: ms.ssrc }
    }
}

impl TryFrom<ScalabilityModeEnum> for MediasoupScalabilityMode {
    type Error = ConversionError;

    fn try_from(value: ScalabilityModeEnum) -> Result<Self, Self::Error> {
        match value {
            ScalabilityModeEnum::None => Ok(Self::None),
            ScalabilityModeEnum::L1t2 => Ok(Self::L1T2),
            ScalabilityModeEnum::L1t2h => Ok(Self::L1T2h),
            ScalabilityModeEnum::L1t3 => Ok(Self::L1T3),
            ScalabilityModeEnum::L1t3h => Ok(Self::L1T3h),
            ScalabilityModeEnum::L2t1 => Ok(Self::L2T1),
            ScalabilityModeEnum::L2t1h => Ok(Self::L2T1h),
            ScalabilityModeEnum::L2t1Key => Ok(Self::L2T1Key),
            ScalabilityModeEnum::L2t2 => Ok(Self::L2T2),
            ScalabilityModeEnum::L2t2h => Ok(Self::L2T2h),
            ScalabilityModeEnum::L2t2Key => Ok(Self::L2T2Key),
            ScalabilityModeEnum::L2t2KeyShift => Ok(Self::L2T2KeyShift),
            ScalabilityModeEnum::L2t3 => Ok(Self::L2T3),
            ScalabilityModeEnum::L2t3h => Ok(Self::L2T3h),
            ScalabilityModeEnum::L2t3Key => Ok(Self::L2T3Key),
            ScalabilityModeEnum::L2t3KeyShift => Ok(Self::L2T3KeyShift),
            ScalabilityModeEnum::L3t1 => Ok(Self::L3T1),
            ScalabilityModeEnum::L3t1h => Ok(Self::L3T1h),
            ScalabilityModeEnum::L3t1Key => Ok(Self::L3T1Key),
            ScalabilityModeEnum::L3t2 => Ok(Self::L3T2),
            ScalabilityModeEnum::L3t2h => Ok(Self::L3T2h),
            ScalabilityModeEnum::L3t2Key => Ok(Self::L3T2Key),
            ScalabilityModeEnum::L3t2KeyShift => Ok(Self::L3T2KeyShift),
            ScalabilityModeEnum::L3t3 => Ok(Self::L3T3),
            ScalabilityModeEnum::L3t3h => Ok(Self::L3T3h),
            ScalabilityModeEnum::L3t3Key => Ok(Self::L3T3Key),
            ScalabilityModeEnum::L3t3KeyShift => Ok(Self::L3T3KeyShift),
            ScalabilityModeEnum::S2t1 => Ok(Self::S2T1),
            ScalabilityModeEnum::S2t1h => Ok(Self::S2T1h),
            ScalabilityModeEnum::S2t2 => Ok(Self::S2T2),
            ScalabilityModeEnum::S2t2h => Ok(Self::S2T2h),
            ScalabilityModeEnum::S2t3 => Ok(Self::S2T3),
            ScalabilityModeEnum::S2t3h => Ok(Self::S2T3h),
            ScalabilityModeEnum::S3t1 => Ok(Self::S3T1),
            ScalabilityModeEnum::S3t1h => Ok(Self::S3T1h),
            ScalabilityModeEnum::S3t2 => Ok(Self::S3T2),
            ScalabilityModeEnum::S3t2h => Ok(Self::S3T2h),
            ScalabilityModeEnum::S3t3 => Ok(Self::S3T3),
            ScalabilityModeEnum::S3t3h => Ok(Self::S3T3h),
            ScalabilityModeEnum::Unspecified => Err(ConversionError::InvalidScalabilityModeEnum),
        }
    }
}

impl TryFrom<MediasoupScalabilityMode> for ScalabilityModeEnum {
    type Error = ConversionError;

    fn try_from(mode: MediasoupScalabilityMode) -> Result<Self, Self::Error> {
        match mode {
            MediasoupScalabilityMode::None => Ok(Self::None),
            MediasoupScalabilityMode::L1T2 => Ok(Self::L1t2),
            MediasoupScalabilityMode::L1T2h => Ok(Self::L1t2h),
            MediasoupScalabilityMode::L1T3 => Ok(Self::L1t3),
            MediasoupScalabilityMode::L1T3h => Ok(Self::L1t3h),
            MediasoupScalabilityMode::L2T1 => Ok(Self::L2t1),
            MediasoupScalabilityMode::L2T1h => Ok(Self::L2t1h),
            MediasoupScalabilityMode::L2T1Key => Ok(Self::L2t1Key),
            MediasoupScalabilityMode::L2T2 => Ok(Self::L2t2),
            MediasoupScalabilityMode::L2T2h => Ok(Self::L2t2h),
            MediasoupScalabilityMode::L2T2Key => Ok(Self::L2t2Key),
            MediasoupScalabilityMode::L2T2KeyShift => Ok(Self::L2t2KeyShift),
            MediasoupScalabilityMode::L2T3 => Ok(Self::L2t3),
            MediasoupScalabilityMode::L2T3h => Ok(Self::L2t3h),
            MediasoupScalabilityMode::L2T3Key => Ok(Self::L2t3Key),
            MediasoupScalabilityMode::L2T3KeyShift => Ok(Self::L2t3KeyShift),
            MediasoupScalabilityMode::L3T1 => Ok(Self::L3t1),
            MediasoupScalabilityMode::L3T1h => Ok(Self::L3t1h),
            MediasoupScalabilityMode::L3T1Key => Ok(Self::L3t1Key),
            MediasoupScalabilityMode::L3T2 => Ok(Self::L3t2),
            MediasoupScalabilityMode::L3T2h => Ok(Self::L3t2h),
            MediasoupScalabilityMode::L3T2Key => Ok(Self::L3t2Key),
            MediasoupScalabilityMode::L3T2KeyShift => Ok(Self::L3t2KeyShift),
            MediasoupScalabilityMode::L3T3 => Ok(Self::L3t3),
            MediasoupScalabilityMode::L3T3h => Ok(Self::L3t3h),
            MediasoupScalabilityMode::L3T3Key => Ok(Self::L3t3Key),
            MediasoupScalabilityMode::L3T3KeyShift => Ok(Self::L3t3KeyShift),
            MediasoupScalabilityMode::S2T1 => Ok(Self::S2t1),
            MediasoupScalabilityMode::S2T1h => Ok(Self::S2t1h),
            MediasoupScalabilityMode::S2T2 => Ok(Self::S2t2),
            MediasoupScalabilityMode::S2T2h => Ok(Self::S2t2h),
            MediasoupScalabilityMode::S2T3 => Ok(Self::S2t3),
            MediasoupScalabilityMode::S2T3h => Ok(Self::S2t3h),
            MediasoupScalabilityMode::S3T1 => Ok(Self::S3t1),
            MediasoupScalabilityMode::S3T1h => Ok(Self::S3t1h),
            MediasoupScalabilityMode::S3T2 => Ok(Self::S3t2),
            MediasoupScalabilityMode::S3T2h => Ok(Self::S3t2h),
            MediasoupScalabilityMode::S3T3 => Ok(Self::S3t3),
            MediasoupScalabilityMode::S3T3h => Ok(Self::S3t3h),
            MediasoupScalabilityMode::Custom { .. } => {
                Err(ConversionError::CustomScalabilityModeUnsupported)
            },
        }
    }
}

impl TryFrom<ScalabilityModeCustom> for MediasoupScalabilityMode {
    type Error = ConversionError;

    fn try_from(value: ScalabilityModeCustom) -> Result<Self, Self::Error> {
        let spatial_layers = u8::try_from(value.spatial_layers)
            .ok()
            .and_then(NonZeroU8::new)
            .ok_or(ConversionError::InvalidSpatialLayer(value.spatial_layers))?;

        let temporal_layers = u8::try_from(value.temporal_layers)
            .ok()
            .and_then(NonZeroU8::new)
            .ok_or(ConversionError::InvalidTemporalLayer(value.temporal_layers))?;

        Ok(Self::Custom {
            spatial_layers,
            temporal_layers,
            ksvc: value.ksvc,
            scalability_mode: value.scalability_mode,
        })
    }
}

impl TryFrom<MediasoupScalabilityMode> for ScalabilityModeCustom {
    type Error = ConversionError;

    fn try_from(mode: MediasoupScalabilityMode) -> Result<Self, Self::Error> {
        match mode {
            MediasoupScalabilityMode::Custom {
                scalability_mode,
                ksvc,
                spatial_layers,
                temporal_layers,
            } => Ok(Self {
                scalability_mode,
                ksvc,
                spatial_layers: u32::from(spatial_layers.get()),
                temporal_layers: u32::from(temporal_layers.get()),
            }),
            _ => Err(ConversionError::ExpectedCustomScalabilityMode),
        }
    }
}

impl TryFrom<ScalabilityMode> for MediasoupScalabilityMode {
    type Error = ConversionError;

    fn try_from(value: ScalabilityMode) -> Result<Self, Self::Error> {
        match value.mode {
            Some(scalability_mode::Mode::Predefined(predefined)) => {
                let enum_val = ScalabilityModeEnum::try_from(predefined)
                    .map_err(|_| ConversionError::InvalidScalabilityModeEnum)?;

                Self::try_from(enum_val)
            }
            Some(scalability_mode::Mode::Custom(custom_val)) => {
                Self::try_from(custom_val)
            }
            None => Err(ConversionError::MissingScalabilityMode),
        }
    }
}

impl TryFrom<MediasoupScalabilityMode> for ScalabilityMode {
    type Error = ConversionError;

    fn try_from(value: MediasoupScalabilityMode) -> Result<Self, Self::Error> {
        let mode = if let MediasoupScalabilityMode::Custom { .. } = value {
            let custom = ScalabilityModeCustom::try_from(value)?;
            scalability_mode::Mode::Custom(custom)
        } else {
            let predefined = ScalabilityModeEnum::try_from(value)?;
            scalability_mode::Mode::Predefined(predefined.into())
        };

        Ok(Self { mode: Some(mode) })
    }
}

impl TryFrom<RtpEncodingParameters> for MediasoupRtpEncodingParameters {
    type Error = ConversionError;

    fn try_from(value: RtpEncodingParameters) -> Result<Self, Self::Error> {
        let ssrc = value.ssrc_opt.map(|rtp_encoding_parameters::SsrcOpt::Ssrc(v)| v);

        let rid = match value.rid_opt {
            Some(rtp_encoding_parameters::RidOpt::Rid(r)) if !r.is_empty() => Some(r),
            _ => None,
        };

        let codec_payload_type = match value.codec_payload_type_opt {
            Some(rtp_encoding_parameters::CodecPayloadTypeOpt::CodecPayloadType(v)) => {
                let codec_payload_type_u8 = u8::try_from(v)
                    .map_err(|_| ConversionError::InvalidPayloadType(v))?;
                Some(codec_payload_type_u8)
            }
            None => None,
        };

        let rtx = value.rtx.map(MediasoupRtpEncodingParametersRtx::from);

        let dtx = value.dtx_opt.map(|rtp_encoding_parameters::DtxOpt::Dtx(b)| b);

        let scalability_mode = match value.scalability_mode {
            Some(mode) => MediasoupScalabilityMode::try_from(mode)?,
            None => MediasoupScalabilityMode::None,
        };

        let max_bitrate = value.max_bitrate_opt.map(|rtp_encoding_parameters::MaxBitrateOpt::MaxBitrate(v)| v);

        Ok(Self {
            ssrc,
            rid,
            codec_payload_type,
            rtx,
            dtx,
            scalability_mode,
            max_bitrate,
        })
    }
}

impl TryFrom<MediasoupRtpEncodingParameters> for RtpEncodingParameters {
    type Error = ConversionError;

    fn try_from(value: MediasoupRtpEncodingParameters) -> Result<Self, Self::Error> {
        Ok(Self {
            ssrc_opt: value.ssrc.map(rtp_encoding_parameters::SsrcOpt::Ssrc),
            rid_opt: value.rid.map(rtp_encoding_parameters::RidOpt::Rid),
            codec_payload_type_opt: value.codec_payload_type.map(|v| {
                rtp_encoding_parameters::CodecPayloadTypeOpt::CodecPayloadType(u32::from(v))
            }),
            dtx_opt: value.dtx.map(rtp_encoding_parameters::DtxOpt::Dtx),
            max_bitrate_opt: value.max_bitrate.map(
                rtp_encoding_parameters::MaxBitrateOpt::MaxBitrate,
            ),
            rtx: value.rtx.map(RtpEncodingParametersRtx::from),
            scalability_mode: Some(ScalabilityMode::try_from(value.scalability_mode)?),
        })
    }
}

impl TryFrom<RtcpParameters> for MediasoupRtcpParameters {
    type Error = ConversionError;

    fn try_from(value: RtcpParameters) -> Result<Self, Self::Error> {
        let cname = match value.cname_opt {
            Some(rtcp_parameters::CnameOpt::Cname(c)) if !c.is_empty() => Some(c),
            _ => None,
        };

        Ok(Self {
            cname,
            reduced_size: value.reduced_size,
        })
    }
}

impl From<MediasoupRtcpParameters> for RtcpParameters {
    fn from(value: MediasoupRtcpParameters) -> Self {
        Self {
            cname_opt: value.cname.map(rtcp_parameters::CnameOpt::Cname),
            reduced_size: value.reduced_size,
        }
    }
}

impl TryFrom<RtpParameters> for MediasoupRtpParameters {
    type Error = ConversionError;

    fn try_from(value: RtpParameters) -> Result<Self, Self::Error> {
        let mid = match value.mid_opt {
            Some(rtp_parameters::MidOpt::Mid(m)) if !m.is_empty() => Some(m),
            _ => None,
        };

        let codecs = value
            .codecs
            .into_iter()
            .map(MediasoupRtpCodecParameters::try_from)
            .collect::<Result<Vec<_>, _>>()?;

        let header_extensions = value
            .header_extensions
            .into_iter()
            .map(MediasoupRtpHeaderExtensionParameters::try_from)
            .collect::<Result<Vec<_>, _>>()?;

        let encodings = value
            .encodings
            .into_iter()
            .map(MediasoupRtpEncodingParameters::try_from)
            .collect::<Result<Vec<_>, _>>()?;

        let rtcp = match value.rtcp {
            Some(r) => MediasoupRtcpParameters::try_from(r)?,
            None => return Err(ConversionError::MissingRtcp),
        };

        Ok(Self {
            mid,
            codecs,
            header_extensions,
            encodings,
            rtcp,
        })
    }
}

impl TryFrom<MediasoupRtpParameters> for RtpParameters {
    type Error = ConversionError;

    fn try_from(value: MediasoupRtpParameters) -> Result<Self, Self::Error> {
        let codecs = value
            .codecs
            .into_iter()
            .map(RtpCodecParameters::try_from)
            .collect::<Result<Vec<_>, _>>()?;

        let header_extensions = value
            .header_extensions
            .into_iter()
            .map(RtpHeaderExtensionParameters::from)
            .collect();

        let encodings = value
            .encodings
            .into_iter()
            .map(RtpEncodingParameters::try_from)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            mid_opt: value.mid.map(rtp_parameters::MidOpt::Mid),
            codecs,
            header_extensions,
            encodings,
            rtcp: Some(RtcpParameters::from(value.rtcp)),
        })
    }
}
