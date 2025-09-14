use actix_web::{get, web, Responder, HttpResponse};
use sqlx::{MySql, Pool};
use serde::Deserialize;

use crate::api::dinhkemfile::db_dinhkemfile::SQL_DINH_KEM_FILE;
use crate::api::dinhkemfile::model_dinhkemfile::{DinhKemFile, convert_json};

#[derive(Deserialize)]
pub struct QueryParams {
    pub file_code: Option<String>,
}

#[get("/api/dinhkemfile")]
pub async fn get_dinhkemfile_json
(
    db_pool: web::Data<Pool<MySql>>,
    truyvan: web::Query<QueryParams>,
) -> impl Responder 
{
    
    let file_code = truyvan.file_code.as_deref().unwrap_or("HS01");
  
    let sql = sqlx::query_as::<_, DinhKemFile>(SQL_DINH_KEM_FILE).bind(file_code).fetch_all(db_pool.get_ref()).await;
    
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
