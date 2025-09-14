use actix_web::{get, web, Responder, HttpResponse};
use sqlx::{MySql, Pool};
use serde::Deserialize;

use crate::api::goiytimkiem::model_goiytimkiem::{GoiYTimKiem, convert_json};

#[derive(Deserialize)]
struct QueryParams {
    titles: Option<String>,
}

#[get("/api/search_suggest")]
pub async fn api_json_goiytimkiem(
    db_pool: web::Data<Pool<MySql>>,
    truyvan: web::Query<QueryParams>,
) -> impl Responder {
    let keyword_input = truyvan.titles.clone().unwrap_or_default();

    // Tách từ khóa thành từng từ riêng (xóa khoảng trắng dư)
    let keywords: Vec<&str> = keyword_input
        .split_whitespace()
        .filter(|w| !w.is_empty())
        .collect();

    // Nếu không có từ khóa thì fallback về từ mặc định
    if keywords.is_empty() {
        return HttpResponse::BadRequest().body("Vui lòng nhập từ khóa tìm kiếm");
    }

    // Tạo chuỗi SQL LIKE động: Title LIKE ? AND Title LIKE ? ...
    let conditions: Vec<String> = keywords
        .iter()
        .map(|_| "Title LIKE ?".to_string())
        .collect();
    let where_clause = conditions.join(" AND ");

    let query_sql = format!(
        "SELECT FileCode, Title, StartDate, dateupdate, path FROM files WHERE {}",
        where_clause
    );

    // Tạo câu truy vấn và bind từng từ
    let mut query = sqlx::query_as::<_, GoiYTimKiem>(&query_sql);
    for word in &keywords {
        let like_pattern = format!("%{}%", word);
        query = query.bind(like_pattern);
    }

    match query.fetch_all(db_pool.get_ref()).await {
        Ok(records) => {
            let json_data = convert_json(records);
            HttpResponse::Ok().json(json_data)
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Lỗi truy vấn database: {}", e)),
    }
}
