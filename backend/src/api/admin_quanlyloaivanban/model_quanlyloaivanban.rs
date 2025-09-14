use serde::{Deserialize, Serialize};
use sqlx::FromRow; 
use serde_json::{json, Value};
pub fn convert_type_documents(records: Vec<TypeDocument>) -> Vec<Value> {
    records
        .into_iter()
        .map(|r| json!({ "typeid": r.type_id, "typename": r.type_name , "oranid": r.oran_id }))
        .collect()
}
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct TypeDocument {
    pub type_id: Option<String>,
    pub type_name: Option<String>,
    pub oran_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct UpdateTypeDocument {
    #[sqlx(rename = "TypeId")]
    pub type_id: Option<String>,
    #[sqlx(rename = "TypeName")]
    pub type_name: Option<String>,
    #[sqlx(rename = "OranId")]
    pub oran_id: Option<String>,
   
}
