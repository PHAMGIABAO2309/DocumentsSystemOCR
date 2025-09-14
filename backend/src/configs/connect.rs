use dotenvy::dotenv;
use sqlx::{MySql, Pool};
use std::env;

pub async fn ketnoi_cosodulieu() -> (Pool<MySql>,  String) {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("Thiếu DATABASE_URL");
    let host = env::var("HOST").unwrap_or("127.0.0.1".into());
    let port = env::var("PORT").unwrap_or("3000".into());
    let address = format!("{host}:{port}");

    let pool = Pool::connect(&db_url).await.expect("Kết nối MySQL thất bại");

    println!("✅ Đã kết nối MySQL");

    (pool,  address)
}
