use utoipa::OpenApi;
use super::{HealthResponse};

#[derive(OpenApi)]
#[openapi(
    paths(super::health),
    components(schemas(HealthResponse)),
    tags((name = "Health", description = "Health check endpoints"))
)]
pub struct ApiDoc;

