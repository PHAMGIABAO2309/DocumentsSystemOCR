use serde::{Deserialize, Serialize};
use chrono::NaiveDate;
use sqlx::FromRow; // Đây là để ánh xạ kết quả từ cơ sở dữ liệu vào struct
use serde_json::{json, Value};

#[derive(Debug, Serialize, Deserialize, FromRow)] // Đây là các trait cần thiết
pub struct Document {
    pub file_code: Option<String>, 
    pub code_number: Option<String>,
    pub file_catalog: Option<String>, 
    pub receives: Option<String>,     
    pub subject: Option<String>,      
    pub validitystatus: Option<String>,
    pub start_date: Option<NaiveDate>,
    pub subject_en: Option<String>,
    pub file_no_nation: Option<String>,
    pub title: Option<String>
}



//loai van ban Hàm chuyển đổi dữ liệu từ database thành JSON
pub fn convert_type_documents(records: Vec<TypeDocument>) -> Vec<Value> {
    records
        .into_iter()
        .map(|r| json!({ "typeid": r.type_id, "typename": r.type_name }))
        .collect()
}
// Struct đại diện cho kết quả truy vấn
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct TypeDocument {
    pub type_id: Option<String>,
    pub type_name: Option<String>,
}


//co quan ban hanh
pub fn convert_organization(records: Vec<Organization>) -> Vec<Value> {
    records
        .into_iter()
        .map(|r| json!({ "oranid": r.oran_id, "oranname": r.oran_name }))
        .collect()
}
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Organization {
    pub oran_id: Option<String>,
    pub oran_name: Option<String>,
}

//chuc vu
pub fn convert_position(records: Vec<Position>) -> Vec<Value> {
    records
        .into_iter()
        .map(|r| json!({ "posid": r.pos_id, "singerinfo": r.singer_info }))
        .collect()
}
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Position {
    pub pos_id: Option<String>,
    pub singer_info: Option<String>,
}

// linh vuc
pub fn convert_field(records: Vec<Field>) -> Vec<Value> {
    records
        .into_iter()
        .map(|r| json!({"fieldcode": r.field_code,  "fieldname": r.field_name }))
        .collect()
}
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Field {
    pub field_code: Option<String>,
    pub field_name: Option<String>,
}