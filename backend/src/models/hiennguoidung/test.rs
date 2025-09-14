use serde::{Deserialize, Serialize};
use sqlx::FromRow; 
use serde_json::{json, Value};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Account {
    #[sqlx(rename = "ID")]
    pub id: Option<String>,
    #[sqlx(rename = "FullName")]
    pub full_name: Option<String>,
   
    #[sqlx(rename = "Email")]
    pub email: Option<String>,
   
}


pub fn convert_account(records: Vec<Account>) -> Vec<Value> {
    records
        .into_iter()
        .map(|r| json!({ "id": r.id, "fullname": r.full_name ,  "email": r.email }))
        .collect()
}