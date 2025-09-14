use actix_web::{post, put, web, Responder, HttpResponse};
use actix_multipart::Multipart;
use futures_util::StreamExt;
use sqlx::{MySql, Pool};
use std::io::Write;
use std::fs::File;
use std::path::Path;
use serde_json::json;

use crate::api::admin_quanlyvanbanden::db_quanlyvanbanden::{INSERT_FILES_SQL,  DELETE_FILES_SQL, DELETE_IDO_SQL, DELETE_DOCUMENTS_EN_SQL, GET_LAST_FILECODE_SQL, GET_LAST_INFOID_SQL,  INSERT_IDO_SQL, INSERT_DOCUMENTS_EN_SQL};
use crate::api::admin_quanlyvanbanden::model_quanlyvanbanden::{DeleteFiles,get_new_code, Update};




#[post("/api/admin/themvanbanden")]
pub async fn post_admin_themvanbanden(
    db_pool: web::Data<Pool<MySql>>,
    mut payload: Multipart,
) -> impl Responder {
    let mut title = String::new();
    let mut filename_saved = String::new();

    // Duyệt qua các phần của multipart/form-data
    while let Some( field) = payload.next().await {
        let mut field = field.unwrap();

        let name = field.name().to_string();

        if name == "title" {
            // Đọc tiêu đề từ phần form
            while let Some(chunk) = field.next().await {
                let data = chunk.unwrap();
                title.push_str(&String::from_utf8_lossy(&data));
            }
        }

        if name == "file" { 
            let content_disposition = field.content_disposition();
            let original_filename = content_disposition.get_filename().unwrap();
            // Giữ lại tên gốc
            let safe_filename = Path::new(original_filename)
                .file_name() .unwrap() .to_string_lossy() .to_string();
            let filepath = format!("./static/luutrufile/{}", safe_filename);
            if Path::new(&filepath).exists() {
                return HttpResponse::BadRequest().body("Tên tệp đã tồn tại. Vui lòng đổi tên tệp.");
            }
            // Ghi file
            let mut f = File::create(&filepath).unwrap();
            while let Some(chunk) = field.next().await {
                let data = chunk.unwrap();
                f.write_all(&data).unwrap();
            }

            filename_saved = safe_filename.clone();
        }
    }
    // Thêm vào database
    match save_to_db(db_pool, title, filename_saved).await {
        Ok(_) => HttpResponse::Ok().body("Thêm văn bản và tải tệp thành công"),
        Err(e) => HttpResponse::InternalServerError().body(format!("Lỗi: {}", e)),
    }
}

async fn save_to_db(
    db_pool: web::Data<Pool<MySql>>,
    title: String,
    file_path: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut tx = db_pool.begin().await?;
    let new_file_code = get_new_code(&mut tx, GET_LAST_FILECODE_SQL, "FileCode", "HS").await;
    let new_info_id = get_new_code(&mut tx,GET_LAST_INFOID_SQL, "InfoId", "TT").await;
    let relative_path = format!("luutrufile/{}", file_path);

    sqlx::query(INSERT_FILES_SQL)
        .bind(&new_file_code)
        .bind(&title)
        .bind(&relative_path)
        .execute(&mut *tx).await?;

    sqlx::query(INSERT_IDO_SQL)
        .bind(&new_info_id)
        .bind(&new_file_code)
        .execute(&mut *tx).await?;

    sqlx::query(INSERT_DOCUMENTS_EN_SQL)
        .bind(&new_info_id)
        .execute(&mut *tx).await?;
    tx.commit().await?;
    Ok(())
}


#[put("/api/admin/delete_vanbanden")]
pub async fn delete_vanbanden(
    db_pool: web::Data<Pool<MySql>>,
    payload: web::Json<DeleteFiles>,
) -> impl Responder {
    // Xóa document
    if let Err(e) = sqlx::query(DELETE_DOCUMENTS_EN_SQL)
        .bind(&payload.info_id)
        .execute(db_pool.get_ref())
        .await
    {
        eprintln!("Lỗi xóa document: {}", e);
        return HttpResponse::InternalServerError().json(json!({
            "success": false,
            "message": "Lỗi xóa document"
        }));
    }
    // Xóa IDO
    if let Err(e) = sqlx::query(DELETE_IDO_SQL)
        .bind(&payload.info_id)
        .execute(db_pool.get_ref())
        .await
    {
        eprintln!("Lỗi xóa IDO: {}", e);
        return HttpResponse::InternalServerError().json(json!({
            "success": false,
            "message": "Lỗi xóa IDO"
        }));
    }
      // Xóa file
    if let Err(e) = sqlx::query(DELETE_FILES_SQL)
        .bind(&payload.file_code)
        .execute(db_pool.get_ref())
        .await
    {
        eprintln!("Lỗi xóa file: {}", e);
        return HttpResponse::InternalServerError().json(json!({
            "success": false,
            "message": "Lỗi xóa file"
        }));
    }

    

    HttpResponse::Ok().json(json!({
        "success": true,
        "message": "Xóa thành công"
    }))
}

#[put("/api/admin/update_vanbanden")]
pub async fn update_vanbanden(
    db_pool: web::Data<Pool<MySql>>,
    payload: web::Json<Update>,
) -> impl Responder {
    // Cập nhật bảng files
    let result1 = sqlx::query!(
        "UPDATE files 
        SET Title = ?, StartDate = ?, OranId = ?, FileNoNation = ?, TypeId = ?, dateupdate = ?
        WHERE FileCode = ?",
        payload.title,
        payload.start_date,
        payload.oran_id,
        payload.file_no_nation,
        payload.type_id,
        payload.date_update,
        payload.file_code
    )
    .execute(db_pool.get_ref())
    .await;

    // Cập nhật bảng infomation_documents_out
    let result2 = sqlx::query!(
        "UPDATE infomation_documents_out 
        SET LanId= 'VN', TypeId= ?, Receives= ?, FileCode= ?, ValidityStatus= ?, PosId= ?, FieldCode= ?
        WHERE InfoId = ?",
        payload.type_id,
        payload.receives,
        payload.file_code,
        payload.validity_status,
        payload.pos_id,
        payload.field_code,
        payload.info_id
    )
    .execute(db_pool.get_ref())
    .await;

    // Kiểm tra kết quả
    match (result1, result2) {
        (Ok(_), Ok(_)) => HttpResponse::Ok().json(json!({
            "success": true,
            "message": "Cập nhật thành công"
        })),
        (Err(e), _) => {
            eprintln!("Lỗi cập nhật files: {}", e);
            HttpResponse::InternalServerError().json(json!({
                "success": false,
                "message": "Lỗi cập nhật bảng files"
            }))
        }
        (_, Err(e)) => {
            eprintln!("Lỗi cập nhật infomation_documents_out: {}", e);
            HttpResponse::InternalServerError().json(json!({
                "success": false,
                "message": "Lỗi cập nhật bảng infomation_documents_out"
            }))
        }
    }
}



