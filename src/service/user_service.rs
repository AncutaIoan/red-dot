use crate::models::user::User;
use std::sync::Arc;
use crate::repository::user_repository::UserRepository;

pub struct UserService {
    repo: Arc<dyn UserRepository + Send + Sync>,
}

impl UserService {
    pub fn new(repo: Arc<dyn UserRepository + Send + Sync>) -> Self {
        Self { repo }
    }

    pub async fn create_user(&self, user: &User) -> Result<User, sqlx::Error> {
        self.repo.create_user(user).await
    }

    pub async fn find_by_email(&self, email: &str) -> Result<Option<User>, sqlx::Error> {
        self.repo.find_by_email(email).await
    }
}
