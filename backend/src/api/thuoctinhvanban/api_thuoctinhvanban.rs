use actix_web::{get, web, Responder, HttpResponse};
use sqlx::{MySql, Pool};
use serde_json::json;
use crate::api::thuoctinhvanban::db_thuoctinhvanban::{GET_TYPE_DOCUMENTS_SQL, GET_ORGANIZATION_SQL, GET_FIELD_SQL};
use crate::api::thuoctinhvanban::model_thuoctinhvanban::{convert_type_documents, TypeDocument, convert_organization, Organization, convert_field, Field};

#[get("/api/thuoctinhvanban")]
pub async fn api_json_thuoctinhvanban(db_pool: web::Data<Pool<MySql>>) -> impl Responder {
    let (Ok(type_docs), Ok(orgs), Ok(fis)) = tokio::join!(
        sqlx::query_as::<_, TypeDocument>(GET_TYPE_DOCUMENTS_SQL).fetch_all(db_pool.get_ref()),
        sqlx::query_as::<_, Organization>(GET_ORGANIZATION_SQL).fetch_all(db_pool.get_ref()),
        sqlx::query_as::<_, Field>(GET_FIELD_SQL).fetch_all(db_pool.get_ref())
    ) else {
        return HttpResponse::InternalServerError().body("Lỗi truy vấn database");
    };

    HttpResponse::Ok().json(json!({
        "type_documents": convert_type_documents(type_docs),
        "organizations": convert_organization(orgs),
        "fields": convert_field(fis),
    }))
}
