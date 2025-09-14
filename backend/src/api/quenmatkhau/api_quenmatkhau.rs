use actix_web::{post, web, Responder, HttpResponse};
use sqlx::{MySql, Pool};
use serde::{Deserialize};
use serde_json::json;
use crate::api::quenmatkhau::model_quenmatkhau::LoginForm;

#[derive(Deserialize)]
pub struct CheckEmailOrPhone {
    email_or_phone: String,
}

#[post("/api/quenmatkhau")]
pub async fn post_quenmatkhau(
    db_pool: web::Data<Pool<MySql>>,
    form: web::Json<CheckEmailOrPhone>,
) -> impl Responder {
    let query = r#"
        SELECT `ID`, `FullName`, `Password`, `Email`, `PhoneNumber` 
        FROM `account` 
        WHERE `Email` = ? OR `PhoneNumber` = ?
    "#;

    match sqlx::query_as::<_, LoginForm>(query)
        .bind(&form.email_or_phone)
        .bind(&form.email_or_phone)
        .fetch_optional(db_pool.get_ref())
        .await 
    {
        Ok(Some(user)) => HttpResponse::Ok().json(json!({
            "status": "success",
            "user": {
                "id": user.id,
                "fullname": user.full_name,
                "email": user.email,
                "phone": user.phone
            }
        })),
        Ok(None) => HttpResponse::Ok().json(json!({
            "status": "error",
            "message": "Email hoặc số điện thoại chưa được đăng ký."
        })),
        Err(e) => {
            eprintln!("DB error: {}", e);
            HttpResponse::InternalServerError().json(json!({
                "status": "error",
                "message": "Lỗi truy vấn database."
            }))
        }
    }
}
