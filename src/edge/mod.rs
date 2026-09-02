pub mod v1 {
    tonic::include_proto!("saasy.edge.v1");
}

pub use v1::{
    ListProviderModelsRequest,
    ListProviderModelsResponse,
    LlmProviderModel,
    SttProviderModel,
    TtsProviderModel,
};
pub use v1::edge_service_server::{EdgeService, EdgeServiceServer};
pub use v1::edge_service_client::EdgeServiceClient;
