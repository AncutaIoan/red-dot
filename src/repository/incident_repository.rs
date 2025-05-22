use sqlx::Error;
use crate::repository::repository::PostgresRepository;
use crate::models::incident::Incident;

#[async_trait::async_trait]
pub trait IncidentRepository: Send + Sync {
    async fn create_incident(&self, incident: &Incident) -> Result<Incident, Error>;
    async fn get_incidents_nearby(&self, lat: f64, lng: f64, radius_meters: f64) -> Result<Vec<Incident>, Error>;
    async fn get_incident_by_id(&self, id: i32) -> Result<Option<Incident>, Error>;
}


#[async_trait::async_trait]
impl IncidentRepository for PostgresRepository {
    async fn create_incident(&self, incident: &Incident) -> Result<Incident, Error> {
        let query = r#"
                            INSERT INTO incidents (title, description, category, latitude, longitude, submitted_at, verified)
                            VALUES ($1, $2, $3, $4, $5, $6, $7)
                            RETURNING id, title, description, category, latitude, longitude, submitted_at, verified
                        "#;

        match sqlx::query_as::<_, Incident>(query)
            .bind(&incident.title)
            .bind(&incident.description)
            .bind(&incident.category)
            .bind(incident.latitude)
            .bind(incident.longitude)
            .bind(incident.submitted_at)
            .bind(incident.verified)
            .fetch_one(&self.pool)
            .await
        {
            Ok(result) => Ok(result),
            Err(e) => {
                eprintln!("Error inserting incident: {}", e);
                Err(e)
            }
        }
    }

    async fn get_incidents_nearby(&self, lat: f64, lng: f64, radius_meters: f64) -> Result<Vec<Incident>, Error> {
        let query = r#"
                                SELECT id, title, description, category, latitude, longitude, submitted_at, verified
                                FROM incidents
                                WHERE ST_DWithin(
                                    ST_SetSRID(ST_MakePoint(longitude, latitude), 4326)::geography,
                                    ST_SetSRID(ST_MakePoint($1, $2), 4326)::geography,
                                    $3
                                )
                            "#;

        match sqlx::query_as::<_, Incident>(query)
            .bind(lng)
            .bind(lat)
            .bind(radius_meters)
            .fetch_all(&self.pool)
            .await
        {
            Ok(results) => Ok(results),
            Err(e) => {
                eprintln!("Error fetching nearby incidents: {}", e);
                Err(e)
            }
        }
    }

    async fn get_incident_by_id(&self, id: i32) -> Result<Option<Incident>, Error> {
        let query = r#"
                            SELECT id, title, description, category, latitude, longitude, submitted_at, verified
                            FROM incidents
                            WHERE id = $1
                        "#;

        match sqlx::query_as::<_, Incident>(query)
            .bind(id)
            .fetch_optional(&self.pool)
            .await
        {
            Ok(result) => Ok(result),
            Err(e) => {
                eprintln!("Error fetching incident by id: {}", e);
                Err(e)
            }
        }
    }

}
