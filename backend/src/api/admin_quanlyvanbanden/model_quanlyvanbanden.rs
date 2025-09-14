use sqlx::{MySql,  Row, FromRow, Transaction};
use serde::{Deserialize, Serialize};
use chrono::NaiveDate;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct AddFiles {
    pub title: Option<String>,
    pub path: Option<String>,
    
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

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct DeleteFiles {
    #[sqlx(rename = "FileCode")]
    pub file_code: Option<String>,
    #[sqlx(rename = "InfoId")]
    pub info_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Update {
    #[sqlx(rename = "FileCode")]
    pub file_code: Option<String>,
    #[sqlx(rename = "Title")]
    pub title: Option<String>,
    #[sqlx(rename = "StartDate")]
    pub start_date: Option<NaiveDate>,
   #[sqlx(rename = "OranId")]
    pub oran_id: Option<String>,
    #[sqlx(rename = "FileNoNation")]
    pub file_no_nation: Option<String>,
    #[sqlx(rename = "TypeId")]
    pub type_id: Option<String>,
    #[sqlx(rename = "dateupdate")]
    pub date_update: Option<NaiveDate>,

    
    #[sqlx(rename = "InfoId")]
    pub info_id: Option<String>,
    #[sqlx(rename = "Receives")]
    pub receives: Option<String>,
    #[sqlx(rename = "ValidityStatus")]
    pub validity_status: Option<String>,
    #[sqlx(rename = "PosId")]
    pub pos_id: Option<String>,
    #[sqlx(rename = "FieldCode")]
    pub field_code: Option<String>,
    
}