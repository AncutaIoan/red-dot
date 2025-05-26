use std::sync::Arc;
use sqlx::PgPool;
use crate::repository::incident_repository::IncidentRepository;
use crate::repository::repository::PostgresRepository;
use crate::repository::user_repository::UserRepository;
use crate::service::incident_service::IncidentService;
use crate::service::user_service::UserService;

pub struct AppState {
    pub incident_state: IncidentState,
    pub user_state: UserState,
}

impl AppState {
    pub fn new(pool: PgPool) -> Self {
        let incident_repo = Arc::new(PostgresRepository::new(pool.clone())) as Arc<dyn IncidentRepository + Send + Sync>;
        let user_repo = Arc::new(PostgresRepository::new(pool.clone())) as Arc<dyn UserRepository + Send + Sync>;

        let incident_service = Arc::new(IncidentService::new(incident_repo));
        let user_service = Arc::new(UserService::new(user_repo));

        AppState {
            incident_state: IncidentState {
                incident_service
            },
            user_state: UserState {
                user_service
            },
        }
    }
}



#[derive(Clone)]
pub struct IncidentState {
    pub incident_service: Arc<IncidentService>
}

#[derive(Clone)]
pub struct UserState {
    pub user_service: Arc<UserService>
}