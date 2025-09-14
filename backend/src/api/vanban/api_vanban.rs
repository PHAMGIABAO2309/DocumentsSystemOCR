use actix_web::{get, web, Responder, HttpResponse};
use sqlx::{MySql, Pool};
use serde::Deserialize;
use crate::api::vanban::db_vanban::SQL_VAN_BAN;
use crate::api::vanban::model_vanban::{VanBan, convert_json};

#[derive(Deserialize)]
pub struct QueryParams {
    pub file_code: Option<String>,
}

#[get("/api/vanban")]
pub async fn api_json_vanban
(
    db_pool: web::Data<Pool<MySql>>,
    truyvan: web::Query<QueryParams>,
) -> impl Responder 
{
    
    let file_code = truyvan.file_code.as_deref().unwrap_or("HS01");
  
    let sql = sqlx::query_as::<_, VanBan>(SQL_VAN_BAN).bind(file_code).fetch_all(db_pool.get_ref()).await;
    
    match sql 
    {
        Ok(save) => 
        {
            let kq_json = convert_json(save); 
            HttpResponse::Ok().json(kq_json)
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Lỗi truy vấn database: {}", e)),
    }
}