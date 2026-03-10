use sqlx::postgres::PgPoolOptions;
use sqlx::types::time::Date;
use std::env;
use std::time::Instant;

#[derive(sqlx::FromRow, Debug)]
struct User {
    id: i32,
    name: String,
    dob: Option<Date>,
}

#[tokio::main]
async fn main() {
    println!("{:?}", env::var("DATABASE_URL"));
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://test:test@localhost:5432/sandbox")
        .await
        .unwrap();

    let q = sqlx::query_as(r#"SELECT id, name, dob FROM users"#);
    let start = Instant::now();
    let r: Vec<User> = q.fetch_all(&pool).await.unwrap();

    println!("{:?}", r);
    let dob = r[1].dob;
    println!("{:?}", dob);
    println!("{:?}", start.elapsed());

    sqlx::query!("select id from users")
        .fetch_all(&pool)
        .await
        .unwrap();
}
