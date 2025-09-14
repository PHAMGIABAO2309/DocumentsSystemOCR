use actix_web::{post, web, Responder, HttpResponse};
use sqlx::{MySql, Pool};
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
pub struct UpdatePasswordForm {
    email_or_phone: String,
    new_password: String,
}

#[post("/api/capnhatmatkhau")]
pub async fn update_password(
    db_pool: web::Data<Pool<MySql>>,
    form: web::Json<UpdatePasswordForm>,
) -> impl Responder {
    let query = r#"
        UPDATE `account` SET `Password` = ? WHERE `Email` = ? OR `PhoneNumber` = ?
    "#;

    match sqlx::query(query)
        .bind(&form.new_password)
        .bind(&form.email_or_phone)
        .bind(&form.email_or_phone)
        .execute(db_pool.get_ref())
        .await
    {
        Ok(result) => {
            if result.rows_affected() > 0 {
                HttpResponse::Ok().json(json!({
                    "status": "success",
                    "message": "Cập nhật mật khẩu thành công."
                }))
            } else {
                HttpResponse::Ok().json(json!({
                    "status": "error",
                    "message": "Không tìm thấy tài khoản với email hoặc số điện thoại này."
                }))
            }
        },
        Err(e) => {
            eprintln!("DB error: {}", e);
            HttpResponse::InternalServerError().json(json!({
                "status": "error",
                "message": "Lỗi truy vấn database."
            }))
        }
    }
}
