use actix_web::{get, web, Responder, HttpResponse};
use sqlx::{MySql, Pool};

use crate::db::hiennguoidung::test::SQL_GET_USER;
use crate::models::hiennguoidung::test::{Account, convert_account};

#[get("/api/user")]
pub async fn api_json_user
(
    database: web::Data<Pool<MySql>>,
) 
-> impl Responder 
{
    let sql = sqlx::query_as::<_, Account>(SQL_GET_USER).fetch_all(database.get_ref()).await;

    match sql 
    {
        Ok(save) => 
        {
            let kq_json = convert_account(save);
            HttpResponse::Ok().json(kq_json)
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Lỗi truy vấn database: {}", e)),
    }
}
