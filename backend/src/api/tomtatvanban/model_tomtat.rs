use serde::{Deserialize, Serialize};
use sqlx::FromRow; 
use serde_json::{json, Value};
use chrono::NaiveDate;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct TomTat {
    #[sqlx(rename = "OranName")]
    pub oranname: Option<String>,
     #[sqlx(rename = "TypeName")]
    pub typename: Option<String>,
    #[sqlx(rename = "SingerInfo")]
    pub singer_info: Option<String>,
    #[sqlx(rename = "StartDate")]
    pub start_date: Option<NaiveDate>,
     #[sqlx(rename = "EndDate")]
    pub end_date: Option<NaiveDate>,
    #[sqlx(rename = "FileNoNation")]
    pub file_no_nation: Option<String>,
    #[sqlx(rename = "Title")]
    pub title: Option<String>,
    #[sqlx(rename = "PosId")]
    pub pos_id: Option<String>,
    #[sqlx(rename = "FieldName")]
    pub field_name: Option<String>,
}


pub fn convert_json(records: Vec<TomTat>) -> Vec<Value> 
{
    records
        .into_iter()
        .map(|r| json!({ 
           "oranname": r.oranname,
            "typename": r.typename,
            "singer_info": r.singer_info,
            "startdate": r.start_date,
            "end_date": r.end_date,
            "file_no_nation": r.file_no_nation,
            "title": r.title , 
            "pos_id": r.pos_id,
            "field_name": r.field_name 
         }))
        .collect()
}