use diesel_async::pooled_connection::bb8::Pool;
use diesel_async::pooled_connection::bb8::PooledConnection;
use diesel_async::AsyncPgConnection;
use cipher_core::repository::Repository;
use cipher_core::repository::RepositoryError;
use cipher_core::repository::RepositoryProvider;

use crate::BackendError;

mod profile_repository;
mod staff_role_repository;
mod user_repository;

pub struct PostgresRepository<'a> {
    conn: PooledConnection<'a, AsyncPgConnection>,
}

impl<'a> PostgresRepository<'a> {
    pub fn new(conn: PooledConnection<'a, AsyncPgConnection>) -> Self {
        Self { conn }
    }
}

impl Repository for PostgresRepository<'_> {
    type BackendError = BackendError;
}

pub struct PostgresRepositoryProvider {
    pool: Pool<AsyncPgConnection>,
}

impl PostgresRepositoryProvider {
    pub fn new(pool: Pool<AsyncPgConnection>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl RepositoryProvider for PostgresRepositoryProvider {
    type BackendError = BackendError;

    type Repository<'a> = PostgresRepository<'a>;

    async fn get(&self) -> Result<Self::Repository<'_>, RepositoryError<Self::BackendError>> {
        self.pool
            .get()
            .await
            .map(PostgresRepository::new)
            .map_err(|err| RepositoryError(BackendError::from(err)))
    }
}
