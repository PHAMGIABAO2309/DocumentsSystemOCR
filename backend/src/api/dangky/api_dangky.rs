use actix_web::{post, web, Responder, HttpResponse};
use sqlx::{MySql, Pool};
use crate::api::dangky::db_dangky::{INSERT_ACCOUNT_SQL, GET_LAST_ACCOUNT_SQL};
use crate::api::dangky::model_dangky::{RegisterForm, get_new_code};

#[post("/api/register")]
pub async fn post_register(
    db_pool: web::Data<Pool<MySql>>,
    form: web::Json<RegisterForm>,
) -> impl Responder {
    match handle_insert(db_pool, form).await {
        Ok(_) => HttpResponse::Ok().body("Thêm tài khoản thành công"),
        Err(e) => HttpResponse::InternalServerError().body(format!("Lỗi: {}", e)),
    }
}

async fn handle_insert(
    db_pool: web::Data<Pool<MySql>>,
    form: web::Json<RegisterForm>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut tx = db_pool.begin().await?;
    let new_file_code = get_new_code(&mut tx, GET_LAST_ACCOUNT_SQL, "ID", "TK").await;

    sqlx::query(INSERT_ACCOUNT_SQL)
        .bind(&new_file_code)
        .bind(&form.full_name)
        .bind(&form.password)
        .bind(&form.email)
        .bind(&form.phone)
        .execute(&mut *tx).await?;

        tx.commit().await?;
        Ok(())
    }