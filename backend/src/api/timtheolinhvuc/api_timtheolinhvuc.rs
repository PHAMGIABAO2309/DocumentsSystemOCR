use actix_web::{get, web, Responder, HttpResponse};
use sqlx::{MySql, Pool};
use serde::Deserialize;

use crate::api::timtheolinhvuc::db_timtheolinhvuc::SQL_TIM_THEO_LINH_VUC;
use crate::api::timtheolinhvuc::model_timtheolinhvuc::{TimTheoLinhVuc, convert_json};
#[derive(Deserialize)]
struct QueryParams {
    fieldname: Option<String>,
}
#[get("/api/search_fields")]
pub async fn api_json_timtheolinhvuc
(
    db_pool: web::Data<Pool<MySql>>,
    truyvan: web::Query<QueryParams>,
) -> impl Responder 
{
    let file_code = truyvan.fieldname.as_deref().map(|s| format!("%{}%", s)) .unwrap_or("%HS01%".to_string()); 
  
    let sql = sqlx::query_as::<_, TimTheoLinhVuc>(SQL_TIM_THEO_LINH_VUC).bind(file_code).fetch_all(db_pool.get_ref()).await;

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