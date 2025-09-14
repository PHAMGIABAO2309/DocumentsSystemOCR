use actix_web::{get, put, web, Responder, HttpResponse};
use sqlx::{MySql, Pool};
use serde_json::json;
use crate::api::admin_quanlytaikhoan::db_admin_account::{GET_ACCOUNT_SQL, UPDATE_ACCOUNT_SQL, DELETE_ACCOUNT_SQL};
use crate::api::admin_quanlytaikhoan::model_admin_account::{convert_account, Account, UpdateAccountRequest, DeleteAccountRequest};
#[get("/api/admin/account")]
pub async fn get_account_json(db_pool: web::Data<Pool<MySql>>) -> impl Responder {
    let result = sqlx::query_as::<_, Account>(GET_ACCOUNT_SQL)
        .fetch_all(db_pool.get_ref())
        .await;
    match result {
        Ok(acc) => {
            HttpResponse::Ok().json(json!({
                "accounts": convert_account(acc)
            }))
        }
        Err(e) => {
            eprintln!("Lỗi truy vấn DB: {}", e);
            HttpResponse::InternalServerError().body("Lỗi truy vấn database")
        }
    }
}


#[put("/api/admin/update_account")]
pub async fn update_account(
    db_pool: web::Data<Pool<MySql>>,
    payload: web::Json<UpdateAccountRequest>,
) -> impl Responder {
    let result = sqlx::query(UPDATE_ACCOUNT_SQL)
        .bind(&payload.fullname)
        .bind(&payload.email)
        .bind(&payload.phone)
        .bind(&payload.id) 
        .execute(db_pool.get_ref())
        .await;

        match result {
            Ok(_) => HttpResponse::Ok().json(json!({"success": true, "message": "Cập nhật thành công"})),
            Err(e) => {
                eprintln!("Lỗi cập nhật DB: {}", e);
                HttpResponse::InternalServerError().json(json!({"success": false, "message": "Lỗi cập nhật database"}))
            }
        }
}


#[put("/api/admin/delete_account")]
pub async fn delete_account(
    db_pool: web::Data<Pool<MySql>>,
    payload: web::Json<DeleteAccountRequest>,
) -> impl Responder {
    let result = sqlx::query(DELETE_ACCOUNT_SQL)
        .bind(&payload.id) 
        .execute(db_pool.get_ref())
        .await;
        match result {
            Ok(_) => HttpResponse::Ok().json(json!({"success": true, "message": "Xóa thành công"})),
            Err(e) => {
                eprintln!("Lỗi cập nhật DB: {}", e);
                HttpResponse::InternalServerError().json(json!({"success": false, "message": "Lỗi xóa database"}))
            }
        }
}