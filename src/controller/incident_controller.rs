use crate::error::errors::IncidentError;
use crate::models::incident::{Incident, SubmitIncidentRequest};
use crate::repository::app_state::AppState;
use actix_web::{
    Result, post,
    web::{Data, Json},
};

#[post("/incident/add")]
pub async fn add_incident(repo: Data<AppState>, incident_request: Json<SubmitIncidentRequest>) -> Result<Json<Incident>, IncidentError> {
    let new_incident = Incident::new_from_request(&incident_request);

    if !new_incident.validate() {
        return Err(IncidentError::InvalidIncident);
    }

    match repo.incident_repo.create_incident(&new_incident).await {
        Ok(inserted_incident) => Ok(Json(inserted_incident)),
        Err(e) => { eprintln!("Error adding incident: {}", e);Err(IncidentError::FailedToCreate) }
    }
}
