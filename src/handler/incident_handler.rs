use crate::error::errors::IncidentError;
use crate::models::incident::{Incident, SubmitIncidentRequest};
use crate::state::app_state::IncidentState;
use actix_web::web::Data;
use actix_web::{post, web::Json, Result};

#[post("/add")]
pub async fn add_incident(incident_state: Data<IncidentState>, Json(incident_request): Json<SubmitIncidentRequest>) -> Result<Json<Incident>, IncidentError> {
    let incident = incident_state
                                .incident_service
                                .add_incident_from_request(&incident_request)
                                .await?;

    Ok(Json(incident))
}
