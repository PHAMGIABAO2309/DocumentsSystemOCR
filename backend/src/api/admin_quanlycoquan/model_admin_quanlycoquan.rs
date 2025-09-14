use serde::{Deserialize, Serialize};
use sqlx::FromRow; 
use serde_json::{json, Value};
use sqlx::{MySql,  Row,  Transaction};
pub fn convert_account(records: Vec<Organization>) -> Vec<Value> {
    records
        .into_iter()
        .map(|r| json!({ "oranid": r.oran_id, "oranname": r.oran_name }))
        .collect()
}
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Organization {
    #[sqlx(rename = "OranId")]
    pub oran_id: Option<String>,
    #[sqlx(rename = "OranName")]
    pub oran_name: Option<String>,
   
}


// them co quan
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct AddOrganization {
    pub oran_id: Option<String>,
    pub oran_name: Option<String>,
   
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