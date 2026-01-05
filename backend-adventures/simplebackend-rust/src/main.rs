use axum;
use clap::Parser;
use simplebackend_rust::{
    db::queries::query_promote_user,
    server::{self, state::AppState},
};
use sqlx::postgres::PgPoolOptions;
use tokio::{self, net::TcpListener};

mod calpstuff;
#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let db = PgPoolOptions::new()
        .max_connections(5)
        .connect(&std::env::var("DATABASE_URL").expect("Database URL not found!"))
        .await
        .expect("Failed To connect to DB");

    // ----- CLI Logic -----
    let cli = calpstuff::CalpStuff::try_parse().ok();
    if let Some(clap) = cli {
        tracing::info!("Running in CLI mode!!!!");

        if clap.promote {
            match query_promote_user(&db, &clap.email).await {
                Ok(Some(u)) => {
                    tracing::info!(
                        "Promoted U (email: {})to admin with userid {}",
                        u.email,
                        u.id
                    );
                }
                Ok(None) => {
                    tracing::info!("No user with the email {} found in the db!", clap.email);
                }
                Err(_) => {
                    tracing::info!("HTTP 500"); //scuffed statuscode
                }
            };

            return;
        }
    }
    // ----- Server Logic -----
    tracing_subscriber::fmt::init();

    let appstate = AppState { db };

    let app = server::router(appstate);

    let tcplister = TcpListener::bind("127.0.0.1:8080")
        .await
        .expect("TcpListener Failed to Bind!");

    axum::serve(tcplister, app).await.unwrap();
}
