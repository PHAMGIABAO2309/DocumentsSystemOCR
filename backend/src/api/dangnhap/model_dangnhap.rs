use serde::{Deserialize, Serialize};
use sqlx::FromRow;


#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct LoginForm {
    #[sqlx(rename = "ID")]
    pub id: Option<String>,
    #[sqlx(rename = "FullName")]
    pub full_name: Option<String>,
    #[sqlx(rename = "Password")]
    pub password: Option<String>,
    #[sqlx(rename = "Email")]
    pub email: Option<String>,
    #[sqlx(rename = "PhoneNumber")]
    pub phone: Option<String>,
}
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email_or_phone: String,
    pub password: String,
}