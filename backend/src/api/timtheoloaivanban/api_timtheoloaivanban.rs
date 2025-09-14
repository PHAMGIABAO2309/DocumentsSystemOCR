use actix_web::{get, web, Responder, HttpResponse};
use sqlx::{MySql, Pool};
use serde::Deserialize;

use crate::api::timtheoloaivanban::db_timtheoloaivanban::SQL_TIM_THEO_LOAI_VAN_BAN;
use crate::api::timtheoloaivanban::model_timtheoloaivanban::{TimTheoLoaiVanBan, convert_json};
#[derive(Deserialize)]
struct QueryParams {
    typename: Option<String>,
}
#[get("/api/search_typedocuments")]
pub async fn api_json_timtheoloaivanban
(
    db_pool: web::Data<Pool<MySql>>,
    truyvan: web::Query<QueryParams>,
) -> impl Responder 
{
    let file_code = truyvan.typename.as_deref().map(|s| format!("%{}%", s)) .unwrap_or("%HS01%".to_string()); 
  
    let sql = sqlx::query_as::<_, TimTheoLoaiVanBan>(SQL_TIM_THEO_LOAI_VAN_BAN).bind(file_code).fetch_all(db_pool.get_ref()).await;

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