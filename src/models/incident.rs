use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Incident {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub category: String,
    pub latitude: f64,
    pub longitude: f64,
    pub submitted_at: DateTime<Utc>,
    pub verified: bool,
}

#[derive(Deserialize)]
pub struct SubmitIncidentRequest {
    pub title: String,
    pub description: String,
    pub category: String,
    pub latitude: f64,
    pub longitude: f64,
}

impl Incident {
    pub fn new_from_request(req: &SubmitIncidentRequest) -> Self {
        Self {
            id: 0,
            title: req.title.clone(),
            description: req.description.clone(),
            category: req.category.clone(),
            latitude: req.latitude,
            longitude: req.longitude,
            submitted_at: Utc::now(),
            verified: false,
        }
    }

    pub fn validate(&self) -> bool {
        !self.title.is_empty() && self.latitude.abs() <= 90.0 && self.longitude.abs() <= 180.0
    }
}
