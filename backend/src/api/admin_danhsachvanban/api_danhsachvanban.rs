use actix_web::{get, web, Responder, HttpResponse};
use sqlx::{MySql, Pool};
use serde_json::json;
use serde::Deserialize;
use chrono::NaiveDate;
#[derive(Deserialize)]
pub struct QueryParams {
    // Nếu bạn có tham số truy vấn, thêm vào đây
}
#[get("/api/danhsachvanban")]
pub async fn get_danhsachvanban_json(
    db_pool: web::Data<Pool<MySql>>,
   
) -> impl Responder {
    let query_result = sqlx::query!(
"SELECT 
    f.FileCode, f.Title, f.TypeId, f.StartDate, f.OranId,
    td.TypeName, 
    ido.ValidityStatus, ido.InfoId, ido.PosId,
    org.OranName,
    p.SingerInfo
FROM  files f
LEFT JOIN  type_documents td ON f.TypeId = td.TypeId
LEFT JOIN  organization org ON f.OranId = org.OranId
LEFT JOIN  infomation_documents_out ido ON ido.FileCode = f.FileCode
LEFT JOIN  positions p ON ido.PosId = p.PosId AND ido.TypeId = f.TypeId
ORDER BY CAST(SUBSTRING(f.FileCode, 3) AS UNSIGNED);

"
    )
    .fetch_all(db_pool.get_ref())
    .await;

    match query_result {
        Ok(records) => {
            let list_title: Vec<_> = records.into_iter()
                .map(|row| {
                    json!({
                        "title": row.Title,
                        "typeid": row.TypeId,
                        "typename": row.TypeName,
                        "filecode": row.FileCode,
                        "startdate": row.StartDate.map(|d: NaiveDate| d.to_string()).unwrap_or("N/A".to_string()),
                        "validitystatus": row.ValidityStatus,
                        "infoid": row.InfoId,
                        "posid": row.PosId,
                        "oranid": row.OranId,
                        "oranname": row.OranName,
                        "singerinfo": row.SingerInfo,
                    })
                })
                .collect();
            HttpResponse::Ok().json(list_title)
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Lỗi truy vấn database: {}", e)),
    }
}
