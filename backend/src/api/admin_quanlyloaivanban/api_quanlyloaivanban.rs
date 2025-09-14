use actix_web::{get, post, put, web, Responder, HttpResponse};
use sqlx::{MySql, Pool, query_as};
use serde_json::json;
use crate::api::admin_quanlyloaivanban::db_quanlyloaivanban::{GET_TYPE_DOCUMENTS_SQL, INSERT_TYPE_DOCUMENTS_SQL, CHECK_TYPE_DOCUMENTS_TYPE_ID_SQL, CHECK_TYPE_DOCUMENTS_TYPE_NAME_SQL, UPDATE_TYPE_DOCUMENTS_SQL, DELETE_TYPE_DOCUMENTS_SQL};
use crate::api::admin_quanlyloaivanban::model_quanlyloaivanban::{convert_type_documents, TypeDocument, UpdateTypeDocument};
#[get("/api/list_typedocuments")]
pub async fn get_listtype_json(db_pool: web::Data<Pool<MySql>>) -> impl Responder {
    let result = sqlx::query_as::<_, TypeDocument>(GET_TYPE_DOCUMENTS_SQL)
        .fetch_all(db_pool.get_ref())
        .await;
    match result {
        Ok(type_docs) => {
            HttpResponse::Ok().json(json!({
                "type_documents": convert_type_documents(type_docs)
            }))
        }
        Err(e) => {
            eprintln!("Lỗi truy vấn DB: {}", e);
            HttpResponse::InternalServerError().body("Lỗi truy vấn database")
        }
    }
}

// them loai van ban
#[post("/api/admin/add_typedocuments")]
pub async fn post_admin_add_typedocuments(
    db_pool: web::Data<Pool<MySql>>,
    form: web::Json<TypeDocument>,
) -> impl Responder {
    match handle_insert(db_pool, form).await {
        Ok(_) => HttpResponse::Ok().body("Thêm loai van ban thành công"),
        Err(e) => HttpResponse::InternalServerError().body(format!("Lỗi: {}", e)),
    }
}
async fn handle_insert(
    db_pool: web::Data<Pool<MySql>>,
    form: web::Json<TypeDocument>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut tx = db_pool.begin().await?;
    sqlx::query(INSERT_TYPE_DOCUMENTS_SQL)
        .bind(&form.type_id)
        .bind(&form.type_name)
        .bind(&form.oran_id)
        .execute(&mut *tx).await?;
    tx.commit().await?;
    Ok(())
}

// kiem tra co ton tai ma loai tai khoan
#[get("/api/admin/check_typedocuments_typeid/{type_id}")]
pub async fn check_typedocuments_typeid(
    db_pool: web::Data<Pool<MySql>>, type_id: web::Path<String>,
) -> impl Responder {
    let type_id_value = type_id.into_inner();
    let result = query_as::<_, (i64,)>(CHECK_TYPE_DOCUMENTS_TYPE_ID_SQL)
        .bind(&type_id_value).fetch_optional(db_pool.get_ref()) .await;
    match result {
        Ok(Some((count,))) if count > 0 => {
            HttpResponse::Ok().json(json!({ "type_id": type_id_value }))
        }, _ => HttpResponse::Ok().json(json!({})),
    }
}

// kiem tra co ton tai ten loai tai khoan
#[get("/api/admin/check_typedocuments_typename/{type_name}")]
pub async fn check_typedocuments_typename(
    db_pool: web::Data<Pool<MySql>>, type_name: web::Path<String>,
) -> impl Responder {
    let type_name_value = type_name.into_inner();
    let result = query_as::<_, (i64,)>(CHECK_TYPE_DOCUMENTS_TYPE_NAME_SQL)
        .bind(&type_name_value).fetch_optional(db_pool.get_ref()) .await;
    match result {
        Ok(Some((count,))) if count > 0 => {
            HttpResponse::Ok().json(json!({ "type_name": type_name_value }))
        }, _ => HttpResponse::Ok().json(json!({})),
    }
}

#[put("/api/admin/update_typedocument")]
pub async fn update_typedocument(
    db_pool: web::Data<Pool<MySql>>,
    payload: web::Json<UpdateTypeDocument>,
) -> impl Responder {
    let result = sqlx::query(UPDATE_TYPE_DOCUMENTS_SQL)
        .bind(&payload.type_name)
        .bind(&payload.oran_id) 
        .bind(&payload.type_id)
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

#[put("/api/admin/delete_typedocument")]
pub async fn delete_typedocument(
    db_pool: web::Data<Pool<MySql>>,
    payload: web::Json<UpdateTypeDocument>,
) -> impl Responder {
    let result = sqlx::query(DELETE_TYPE_DOCUMENTS_SQL)
        .bind(&payload.type_id) 
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