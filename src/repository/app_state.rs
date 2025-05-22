use std::sync::Arc;
use sqlx::PgPool;
use crate::repository::incident_repository::IncidentRepository;
use crate::repository::repository::PostgresRepository;
use crate::repository::user_repository::UserRepository;

pub struct AppState {
    pub incident_repo: Arc<dyn IncidentRepository + Send + Sync>,
    pub user_repo: Arc<dyn UserRepository + Send + Sync>,
}

impl AppState {
    pub fn new(pool: PgPool) -> Self {
        let incident_repo = Arc::new(PostgresRepository::new(pool.clone())) as Arc<dyn IncidentRepository + Send + Sync>;
        let user_repo = Arc::new(PostgresRepository::new(pool.clone())) as Arc<dyn UserRepository + Send + Sync>;
        AppState {
            incident_repo,
            user_repo
        }
    }
}