use serde::{Deserialize, Serialize};
use sqlx::FromRow; 
use serde_json::{json, Value};
pub fn convert_account(records: Vec<Account>) -> Vec<Value> {
    records
        .into_iter()
        .map(|r| json!({ "id": r.id, "fullname": r.full_name , "password": r.password, "email": r.email, "phone": r.phone }))
        .collect()
}
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Account {
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


#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct UpdateAccountRequest {
    #[sqlx(rename = "FullName")]
    pub fullname: Option<String>,
    #[sqlx(rename = "Email")]
    pub email: Option<String>,
    #[sqlx(rename = "PhoneNumber")]
    pub phone: Option<String>,
    #[sqlx(rename = "ID")]
    pub id: Option<String>,
}


#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct DeleteAccountRequest {
    #[sqlx(rename = "ID")]
    pub id: Option<String>,
}