use actix_web::{get, web, Responder, HttpResponse};
use sqlx::{MySql, Pool};
use serde::Deserialize;

use crate::api::timtheocoquanbanhanh::db_timtheocoquan::SQL_TIM_THEO_CO_QUAN_BAN_HANH;
use crate::api::timtheocoquanbanhanh::model_timtheocoquan::{TimTheoCoQuanBanHanh, convert_json};
#[derive(Deserialize)]
struct QueryParams {
    oranname: Option<String>,
}
#[get("/api/search_organization")]
pub async fn api_json_timtheocoquan
(
    db_pool: web::Data<Pool<MySql>>,
    truyvan: web::Query<QueryParams>,
) -> impl Responder 
{
    let file_code = truyvan.oranname.as_deref().map(|s| format!("%{}%", s)) .unwrap_or("%HS01%".to_string()); 
  
    let sql = sqlx::query_as::<_, TimTheoCoQuanBanHanh>(SQL_TIM_THEO_CO_QUAN_BAN_HANH).bind(file_code).fetch_all(db_pool.get_ref()).await;

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