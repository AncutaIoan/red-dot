use std::sync::Arc;
use crate::error::errors::IncidentError;
use crate::models::incident::{Incident, SubmitIncidentRequest};
use crate::repository::incident_repository::IncidentRepository;

pub struct IncidentService {
    repo: Arc<dyn IncidentRepository + Send + Sync>,
}

impl IncidentService {
    pub fn new(repo: Arc<dyn IncidentRepository + Send + Sync>) -> Self {
        Self { repo }
    }

    pub async fn add_incident_from_request(
        &self,
        req: &SubmitIncidentRequest,
    ) -> Result<Incident, IncidentError> {
        let new_incident = Incident::new_from_request(req);

        if !new_incident.validate() {
            return Err(IncidentError::InvalidIncident);
        }

        self.repo
            .create_incident(&new_incident)
            .await
            .map_err(|e| {
                eprintln!("Error adding incident: {}", e);
                IncidentError::FailedToCreate
            })
    }
}