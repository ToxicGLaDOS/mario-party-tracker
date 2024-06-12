use serde::Serialize;

#[derive(Serialize)]
pub struct MessageResponse {
    pub message: String,
    pub success: bool
}
