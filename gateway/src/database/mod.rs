use clap::Parser;
use sqlx::Executor;
use sqlx::Pool;
use sqlx::Postgres;
use sqlx::postgres::PgPoolOptions;
use std::fmt;
use url::Url;

#[derive(Debug, Parser, Clone)]
pub struct DatabaseConfig {
    /// Hostname
    #[arg(long, env = "DATABASE_HOSTNAME")]
    pub db_hostname: String,
    /// Username
    #[arg(long, env = "DATABASE_USER")]
    pub db_user: String,
    /// Password
    #[arg(long, env = "DATABASE_PASSWORD")]
    pub db_password: String,
    /// Database Name
    #[arg(long, env = "DATABASE_NAME")]
    pub db_name: String,
}

impl fmt::Display for DatabaseConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "postgres://{}:{}@{}",
            &self.db_user, &self.db_password, &self.db_hostname
        )
    }
}

use sqlx::migrate::Migrator;
static MIGRATOR: Migrator = sqlx::migrate!();

impl DatabaseConfig {
    pub async fn new_client(&self) -> Result<Pool<Postgres>, sqlx::Error> {
        let mut database_url = Url::parse(&self.to_string()).unwrap();
        database_url.set_path("postgres");

        let init_pool = PgPoolOptions::new()
            .max_connections(1)
            .connect(database_url.as_str())
            .await?;

        let db_exists: (bool,) =
            sqlx::query_as("SELECT EXISTS(SELECT 1 FROM pg_database WHERE datname = $1)")
                .bind(&self.db_name)
                .fetch_one(&init_pool)
                .await?;

        if !db_exists.0 {
            init_pool
                .execute(format!(r#"CREATE DATABASE "{}""#, &self.db_name).as_str())
                .await?;
        }

        database_url.set_path(&self.db_name);
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(database_url.as_str())
            .await
            .unwrap();
        MIGRATOR.run(&pool).await?;
        Ok(pool)
    }
}
