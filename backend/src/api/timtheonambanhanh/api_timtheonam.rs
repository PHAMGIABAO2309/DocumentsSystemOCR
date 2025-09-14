use actix_web::{get, web, Responder, HttpResponse};
use sqlx::{MySql, Pool};
use serde::Deserialize;

use crate::api::timtheonambanhanh::db_timtheonam::SQL_TIM_THEO_NAM;
use crate::api::timtheonambanhanh::model_timtheonam::{TimTheoNamBanHanh, convert_json};
#[derive(Deserialize)]
struct QueryParams {
    filecatalog: Option<String>,
}
#[get("/api/search_filecatalog")]
pub async fn api_json_timtheonambanhanh
(
    db_pool: web::Data<Pool<MySql>>,
    truyvan: web::Query<QueryParams>,
) -> impl Responder 
{
    let file_code = truyvan.filecatalog.as_deref().map(|s| format!("%{}%", s)) .unwrap_or("%HS01%".to_string()); 
  
    let sql = sqlx::query_as::<_, TimTheoNamBanHanh>(SQL_TIM_THEO_NAM).bind(file_code).fetch_all(db_pool.get_ref()).await;

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