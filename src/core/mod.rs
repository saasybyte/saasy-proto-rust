pub mod v1 {
    tonic::include_proto!("saasy.core.v1");
}

pub use v1::{
    RecordUsageRequest,
    RecordUsageResponse,
};
pub use v1::core_service_server::{CoreService, CoreServiceServer};
pub use v1::core_service_client::CoreServiceClient;
