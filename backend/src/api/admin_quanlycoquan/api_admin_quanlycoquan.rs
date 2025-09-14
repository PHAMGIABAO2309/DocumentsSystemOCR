use actix_web::{get, post, put, web, Responder, HttpResponse};
use sqlx::{MySql, Pool,query_as};
use serde_json::json;
use crate::api::admin_quanlycoquan::db_admin_quanlycoquan::{GET_ORGANIZATION_SQL, GET_LAST_ORGANIZATION_SQL, INSERT_ORGANIZATION_SQL, CHECK_ORGANIZATION_SQL, UPDATE_ORGANIZATION_SQL, DELETE_ORGANIZATION_SQL};
use crate::api::admin_quanlycoquan::model_admin_quanlycoquan::{convert_account, Organization, AddOrganization, get_new_code};
#[get("/api/admin/organization")]
pub async fn get_organization_json(db_pool: web::Data<Pool<MySql>>) -> impl Responder {
    let result = sqlx::query_as::<_, Organization>(GET_ORGANIZATION_SQL).fetch_all(db_pool.get_ref()).await;
    match result {
        Ok(org) => {
            HttpResponse::Ok().json(json!({
                "organizations": convert_account(org)
            }))
        }
        Err(e) => {
            eprintln!("Lỗi truy vấn DB: {}", e);
            HttpResponse::InternalServerError().body("Lỗi truy vấn database")
        }
    }
}

#[get("/api/admin/get_new_organization_code")]
pub async fn get_new_organization_code(
    db_pool: web::Data<Pool<MySql>>,
) -> impl Responder {
    match handle_get_new_code(db_pool).await {
        Ok(new_code) => HttpResponse::Ok().body(new_code),
        Err(e) => HttpResponse::InternalServerError().body(format!("Lỗi: {}", e)),
    }
}
async fn handle_get_new_code(
    db_pool: web::Data<Pool<MySql>>,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut tx = db_pool.begin().await?;
    let new_code = get_new_code(&mut tx, GET_LAST_ORGANIZATION_SQL, "OranId", "CQ").await;
    tx.commit().await?;
    Ok(new_code)
}

#[post("/api/admin/add_organization")]
pub async fn post_admin_add_organization(
    db_pool: web::Data<Pool<MySql>>,
    form: web::Json<AddOrganization>,
) -> impl Responder {
    match handle_insert(db_pool, form).await {
        Ok(_) => HttpResponse::Ok().body("Thêm cơ quan thành công"),
        Err(e) => HttpResponse::InternalServerError().body(format!("Lỗi: {}", e)),
    }
}
async fn handle_insert(
    db_pool: web::Data<Pool<MySql>>,
    form: web::Json<AddOrganization>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut tx = db_pool.begin().await?;
    sqlx::query(INSERT_ORGANIZATION_SQL)
        .bind(&form.oran_id)
        .bind(&form.oran_name)
        .execute(&mut *tx).await?;
    tx.commit().await?;
    Ok(())
}



#[get("/api/admin/check_organization_name/{oran_name}")]
pub async fn check_organization_name(
    db_pool: web::Data<Pool<MySql>>,
    oran_name: web::Path<String>,
) -> impl Responder {
    let oran_name_value = oran_name.into_inner();
    let result: Result<(i64,), sqlx::Error> = query_as(CHECK_ORGANIZATION_SQL)
        .bind(&oran_name_value)  
        .fetch_one(db_pool.get_ref())
        .await;
    match result {
        Ok((count,)) => {  
            if count > 0 {
                HttpResponse::Ok().json(json!({
                    "exists": true,
                    "organization": AddOrganization {
                        oran_name: Some(oran_name_value.clone()),  
                        oran_id: Some("Ma co quan".to_string())  
                    }
                }))
            } else {
                HttpResponse::Ok().json(json!({"exists": false}))
            }
        }
        Err(e) => {
            eprintln!("Lỗi truy vấn DB: {}", e);
            HttpResponse::InternalServerError().body("Lỗi truy vấn database")
        }
    }
}


#[put("/api/admin/update_organization")]
pub async fn update_organization(
    db_pool: web::Data<Pool<MySql>>,
    payload: web::Json<Organization>,
) -> impl Responder {
    let result = sqlx::query(UPDATE_ORGANIZATION_SQL)
        .bind(&payload.oran_name)
        .bind(&payload.oran_id) 
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

#[put("/api/admin/delete_organization")]
pub async fn delete_organization(
    db_pool: web::Data<Pool<MySql>>,
    payload: web::Json<Organization>,
) -> impl Responder {
    let result = sqlx::query(DELETE_ORGANIZATION_SQL)
        .bind(&payload.oran_id) 
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
