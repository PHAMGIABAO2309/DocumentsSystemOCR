use serde::{Deserialize, Serialize};
use sqlx::FromRow; 
use serde_json::{json, Value};
use chrono::NaiveDate;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct TimTheoLoaiVanBan {
    #[sqlx(rename = "FileCode")]
    pub filecode: Option<String>,
    #[sqlx(rename = "Title")]
    pub title: Option<String>,
    #[sqlx(rename = "StartDate")]
    pub start_date: Option<NaiveDate>,
   #[sqlx(rename = "dateupdate")]
    pub date_update: Option<NaiveDate>,
    #[sqlx(rename = "TypeName")]
    pub typename: Option<String>,
   
}


pub fn convert_json(records: Vec<TimTheoLoaiVanBan>) -> Vec<Value> 
{
    records
        .into_iter()
        .map(|r| json!({ 
            "filecode": r.filecode, 
            "title": r.title ,  
            "startdate": r.start_date,
            "dateupdate": r.date_update,
            "typename": r.typename
         }))
        .collect()
}