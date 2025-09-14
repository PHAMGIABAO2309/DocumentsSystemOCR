use sqlx::{MySql,  Row, FromRow, Transaction};
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct RegisterForm {
    pub id: Option<String>,
    pub full_name: Option<String>,
    pub password: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
}

pub fn gen_code(prefix: &str, last: Option<String>) -> String {
    last.and_then(|s| s.strip_prefix(prefix)?.parse::<u32>().ok())
        .map_or(format!("{}01", prefix), |n| format!("{}{:02}", prefix, n + 1))
}
pub async fn get_new_code( tx: &mut Transaction<'_, MySql>, sql: &str, column: &str, prefix: &str,
) -> String {
    let last_code = sqlx::query(sql)
        .fetch_optional(&mut **tx).await.ok().flatten()
        .and_then(|row| row.try_get::<String, _>(column).ok());
    gen_code(prefix, last_code)
}