use serde::{Deserialize, Serialize};
use sqlx::FromRow; 
use serde_json::{json, Value};
use chrono::NaiveDate;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct VanBan {
    #[sqlx(rename = "FileCode")]
    pub filecode: Option<String>,
     #[sqlx(rename = "CodeNumber")]
    pub codenumber: Option<String>,
    #[sqlx(rename = "FileCatalog")]
    pub file_catalog: Option<String>,
    #[sqlx(rename = "Receives")]
    pub receives: Option<String>,
    #[sqlx(rename = "Subject")]
    pub subject: Option<String>,
    #[sqlx(rename = "ValidityStatus")]
    pub validity_status: Option<String>,
    #[sqlx(rename = "Title")]
    pub title: Option<String>,
    #[sqlx(rename = "StartDate")]
    pub start_date: Option<NaiveDate>,
    #[sqlx(rename = "SubjectEN")]
    pub subject_en: Option<String>,
    #[sqlx(rename = "FileNoNation")]
    pub file_no_nation: Option<String>,
   
}


pub fn convert_json(records: Vec<VanBan>) -> Vec<Value> 
{
    records
        .into_iter()
        .map(|r| json!({ 
            "filecode": r.filecode, 
            "codenumber": r.codenumber,
            "file_catalog": r.file_catalog,
            "receives": r.receives,
            "subject": r.subject,
            "validity_status": r.validity_status,
            "title": r.title ,  
            "startdate": r.start_date,
            "subject_en": r.subject_en,
            "file_no_nation": r.file_no_nation
         }))
        .collect()
}