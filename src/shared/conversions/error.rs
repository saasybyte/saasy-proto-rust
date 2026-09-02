#[derive(Debug, thiserror::Error)]
pub enum ConversionError {
    #[error("Missing codec capability (neither audio nor video provided)")]
    MissingCodecCapability,

    #[error("Missing codec parameters (neither audio nor video provided)")]
    MissingCodecParameters,

    #[error("Missing codec parameter values")]
    MissingCodecParameterValue,

    #[error("Expected an audio codec")]
    ExpectedAudioCodec,

    #[error("Expected a video codec")]
    ExpectedVideoCodec,

    #[error("Invalid RTP header extension direction")]
    InvalidHeaderDirection,

    #[error("Invalid RTP header extension uri")]
    InvalidHeaderExtensionUri,

    #[error("Invalid mime type audio")]
    InvalidMimeTypeAudio,

    #[error("Invalid mime type video")]
    InvalidMimeTypeVideo,

    #[error("Invalid rtcp feedback type")]
    InvalidRtcpFeedbackType,

    #[error("Invalid URI: {0}")]
    InvalidUri(String),

    #[error("Invalid media kind: {0}")]
    InvalidMediaKind(i32),
    
    #[error("Invalid DTLS role: {0}")]
    InvalidDtlsRole(i32),
    
    #[error("Invalid fingerprint algorithm: {0}")]
    InvalidFingerprintAlgorithm(i32),

    #[error("Invalid fingerprint algorithm length: {0}")]
    InvalidFingerprintAlgorithmLength(usize),
    
    #[error("Invalid MIME type: {0}")]
    InvalidMimeType(String),

    #[error("Invalid clock rate: {0}")]
    InvalidClockRate(u32),

    #[error("Invalid channels: {0}")]
    InvalidChannels(u32),

    #[error("Invalid fingerprint length: expected {0} bytes, got {1} bytes")]
    InvalidFingerprintLength(usize, usize),

    #[error("Invalid payload type: {0} (expected 0–255)")]
    InvalidPayloadType(u32),

    #[error("Invalid preferred ID: {0} (must be ≤ 65535)")]
    InvalidPreferredId(u32),

    #[error("Invalid header extension ID: {0} (must be ≤ 65535)")]
    InvalidHeaderExtensionId(u32),

    #[error("Invalid scalability mode")]
    InvalidScalabilityModeEnum,

    #[error("Invalid DTLS algorithm")]
    InvalidDtlsAlgorithm,

    #[error("Unsupported Rtcp feedback type: {0}")]
    UnsupportedRtcpFeedbackType(String, String),

    #[error("Invalid ICE protocol")]
    InvalidIceProtocol,

    #[error("Invalid ICE candidate type")]
    InvalidIceCandidateType,

    #[error("Invalid ICE candidate TCP type")]
    InvalidIceCandidateTcpType,

    #[error("Missing scalability mode")]
    MissingScalabilityMode,

    #[error("Missing rtcp")]
    MissingRtcp,

    #[error("Custom scalability mode is unsupported")]
    CustomScalabilityModeUnsupported,

    #[error("Expected custom scalability mode")]
    ExpectedCustomScalabilityMode,

    #[error("Invalid spatial layer: {0}")]
    InvalidSpatialLayer(u32),

    #[error("Invalid temporal layer: {0}")]
    InvalidTemporalLayer(u32),

    #[error("Invalid port value: {0}")]
    InvalidPort(u32),
    
    #[error("Missing required field: {0}")]
    MissingField(String),

    #[error("Missing preferred payload type")]
    MissingPreferredPayloadType,
    
    #[error("Invalid value: {0}")]
    InvalidValue(String),

    #[error("Invalid UUID: {0}")]
    InvalidUuid(String),

    #[error("Invalid username fragment: {0}")]
    InvalidUsernameFragment(String),

    #[error("Invalid password: {0}")]
    InvalidPassword(String),

    #[error("Unsupported codec type")]
    UnsupportedCodecType,
}
