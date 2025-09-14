use actix_web::{get, web, Responder, HttpResponse};
use sqlx::{MySql, Pool};

use crate::api::trangchu::db_trangchu::SQL_DANH_SACH_VAN_BAN;
use crate::api::trangchu::model_trangchu::{DanhSachVanBan, convert_json};

#[get("/api/trangchu/danhsachvanban")]
pub async fn api_json_trangchu_danhsachvanban
(
    database: web::Data<Pool<MySql>>,
) 
-> impl Responder 
{
    let sql = sqlx::query_as::<_, DanhSachVanBan>(SQL_DANH_SACH_VAN_BAN).fetch_all(database.get_ref()).await;

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
