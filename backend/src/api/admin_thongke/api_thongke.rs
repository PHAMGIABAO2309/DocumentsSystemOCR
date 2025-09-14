use actix_web::{get, web, Responder, HttpResponse};
use sqlx::{MySql, Pool};
use serde_json::json;

#[get("/api/admin/thongke")]
pub async fn get_thongke_json(
    db_pool: web::Data<Pool<MySql>>,
) -> impl Responder {
    let query_result = sqlx::query!(
        r#"
        SELECT 
        DATE_FORMAT(StartDate, '%m/%Y') AS month,
        COUNT(*) AS count
    FROM files
    GROUP BY YEAR(StartDate), MONTH(StartDate)
    ORDER BY YEAR(StartDate), MONTH(StartDate)
        "#
    )
    .fetch_all(db_pool.get_ref())
    .await;

    match query_result {
        Ok(records) => {
            let result_json: Vec<_> = records.into_iter().map(|record| {
                json!({
                    "month": record.month,
                    "count": record.count,
                })
            }).collect();

            HttpResponse::Ok().json(result_json)
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Lỗi truy vấn database: {}", e)),
    }
}


#[get("/api/admin/thongketaikhoan")]
pub async fn get_thongketaikhoan_json(
    db_pool: web::Data<Pool<MySql>>,
) -> impl Responder {
    let query_result = sqlx::query!(
        r#"
        SELECT `ID`, `FullName` FROM `account` 
        "#
    )
    .fetch_all(db_pool.get_ref())
    .await;

    match query_result {
        Ok(records) => {
            let result_json: Vec<_> = records.into_iter().map(|record| {
                json!({
                    "id": record.ID,
                    "fullname": record.FullName
                })
            }).collect();

            HttpResponse::Ok().json(result_json)
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Lỗi truy vấn database: {}", e)),
    }
}

#[get("/api/admin/thongkeloaivanban")]
pub async fn get_thongkeloaivanban_json(
    db_pool: web::Data<Pool<MySql>>,
) -> impl Responder {
    let query_result = sqlx::query!(
        r#"
       SELECT t.TypeName, COUNT(f.Title) AS TotalTitles
FROM files f
JOIN type_documents t ON f.TypeId = t.TypeId
GROUP BY t.TypeName;

        "#
    )
    .fetch_all(db_pool.get_ref())
    .await;

    match query_result {
        Ok(records) => {
            let result_json: Vec<_> = records.into_iter().map(|record| {
                json!({
                    "typename": record.TypeName,
                    "totaltitle": record.TotalTitles
                })
            }).collect();

            HttpResponse::Ok().json(result_json)
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Lỗi truy vấn database: {}", e)),
    }
}



#[get("/api/admin/thongkecoquan")]
pub async fn get_thongkecoquan_json(
    db_pool: web::Data<Pool<MySql>>,
) -> impl Responder {
    let query_result = sqlx::query!(
        r#"
       SELECT o.OranName, COUNT(f.Title) AS TotalTitles
FROM files f
JOIN organization o ON f.OranId = o.OranId
GROUP BY o.OranName;

        "#
    )
    .fetch_all(db_pool.get_ref())
    .await;

    match query_result {
        Ok(records) => {
            let result_json: Vec<_> = records.into_iter().map(|record| {
                json!({
                    "oranname": record.OranName,
                    "totaltitle": record.TotalTitles
                })
            }).collect();

            HttpResponse::Ok().json(result_json)
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Lỗi truy vấn database: {}", e)),
    }
}