use sqlx::{PgPool, postgres::PgPoolOptions};
use std::env;

pub async fn connect_db() -> PgPool {
    let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| {
        panic!(
            "\n❌ DATABASE_URL is missing.\n\n\
             Create a .env file and add:\n\
             DATABASE_URL=postgres://lkp_user:password@localhost:5432/lkp_rust\n"
        )
    });

    PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
        .unwrap_or_else(|error| {
            panic!(
                "\n❌ Failed to connect to PostgreSQL.\n\n\
                 Error:\n{error}\n\n\
                 Fix:\n\
                 1. Make sure PostgreSQL is running:\n\
                    sudo systemctl start postgresql\n\n\
                 2. Prepare the project database:\n\
                    just db-ready\n\n\
                 3. Start development:\n\
                    just dev\n"
            )
        })
}
