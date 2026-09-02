pub mod v1 {
    tonic::include_proto!("saasy.speaking_engine.v1");
}

pub use v1::{
    engine_to_orchestrator_event,
    speaking_engine_control_message,
    speaking_engine_media_ack,
    speaking_engine_media_payload,
    CloseSessionRequest,
    CloseSessionResponse,
    CreateTransportRequest,
    CreateTransportResponse,
    DirectionEnum,
    EngineToOrchestratorEvent,
    FlushAudioRequest,
    FlushAudioResponse,
    GetDeviceRtpCapabilitiesRequest,
    GetDeviceRtpCapabilitiesResponse,
    GetRouterProducerIdRequest,
    GetRouterProducerIdResponse,
    HealthCheckRequest,
    HealthCheckResponse,
    LoadDeviceRequest,
    LoadDeviceResponse,
    MediaFrame,
    OnConnect,
    OnPlaybackComplete,
    OrchestratorToEngineEvent,
    SpeakingEngineControlMessage,
    SpeakingEngineMediaAck,
    SpeakingEngineMediaPayload,
    SpeechGenerationCompleteRequest,
    SpeechGenerationCompleteResponse,
    StartProductionRequest,
    StartProductionResponse,
};
pub use v1::speaking_engine_service_server::{
    SpeakingEngineService,
    SpeakingEngineServiceServer,
};
pub use v1::speaking_engine_service_client::SpeakingEngineServiceClient;
