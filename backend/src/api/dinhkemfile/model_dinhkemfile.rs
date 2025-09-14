use serde::{Deserialize, Serialize};
use sqlx::FromRow; 
use serde_json::{json, Value};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct DinhKemFile {
    #[sqlx(rename = "FileCode")]
    pub filecode: Option<String>,
    #[sqlx(rename = "Title")]
    pub title: Option<String>,
    #[sqlx(rename = "path")]
    pub path: Option<String>,
   
}


pub fn convert_json(records: Vec<DinhKemFile>) -> Vec<Value> 
{
    records
        .into_iter()
        .map(|r| json!({ 
            "filecode": r.filecode, 
            "title": r.title ,  
            "path": r.path
         }))
        .collect()
}