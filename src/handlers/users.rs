// use axum::http::{Response, StatusCode};
use dotenv::dotenv;
use std::env;
use tokio_postgres::{NoTls};

pub async fn users_handler() -> Result<String, String> {
    dotenv().ok();
    let db_url: String = env::var("DATABASE_URL").map_err(|_| "db url is incorrect")?;

    let (client, connection) = tokio_postgres::connect(&db_url, NoTls)
        .await
        .map_err(|e| format!("Error connecting to db: {} ", e))?;

    client
        .execute(
            "CREATE TABLE IF NOT EXISTS users (
        id SERIAL PRIMARY KEY,
        name VARCHAR(255) NOT NULL
        )",
            &[],
        )
        .await
        .map_err(|e| format!("Error creating Users table: {}", e))?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let rows = client
        .query("SELECT * FROM users", &[])
        .await
        .map_err(|e| format!("User Table not found: {}", e))?;

    let mut response = String::new();
    for row in rows.iter() {
        let id: i32 = row.get(0);
        let name: String = row.get(1);
        response.push_str(&format!("{} - {}\n", id, name));
    }

    Ok(response)
}
