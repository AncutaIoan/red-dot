use actix_web::{ResponseError, HttpResponse};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum IncidentError {
    #[error("Invalid incident data")]
    InvalidIncident,
    #[error("Failed to create incident")]
    FailedToCreate,
}

impl ResponseError for IncidentError {
    fn error_response(&self) -> HttpResponse {
        match self {
            IncidentError::InvalidIncident => HttpResponse::BadRequest().body(self.to_string()),
            IncidentError::FailedToCreate => HttpResponse::InternalServerError().body(self.to_string()),
        }
    }
}
